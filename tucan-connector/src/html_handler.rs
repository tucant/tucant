use std::iter::Peekable;

use data_encoding::BASE64URL_NOPAD;
use ego_tree::iter::Children;
use ego_tree::NodeRef;
use scraper::node::Attrs;
use scraper::Node;
use sha3::{Digest, Sha3_256};

pub struct Root<'a> {
    node: NodeRef<'a, Node>,
}

pub struct BeforeDoctype;

pub struct AfterDoctype;

pub struct InRoot<'a, OuterState, RootSubState> {
    node: NodeRef<'a, Node>,
    children: Children<'a, Node>,
    #[allow(unused)]
    sub_state: RootSubState,
    outer_state: OuterState,
}

pub struct BeforeNode<'a, OuterState> {
    node: NodeRef<'a, Node>,
    outer_state: OuterState,
}

pub struct Open<'a, OuterState> {
    element: NodeRef<'a, Node>,
    attrs: Attrs<'a>,
    outer_state: OuterState,
}

pub struct InElement<'a, OuterState> {
    element: NodeRef<'a, Node>,
    children: Peekable<Children<'a, Node>>,
    outer_state: OuterState,
}

impl<'a> Root<'a> {
    #[must_use]
    pub fn new(node: NodeRef<'a, Node>) -> Self {
        assert_eq!(*node.value(), Node::Document);
        Self { node }
    }

    #[must_use]
    pub fn document_start(self) -> InRoot<'a, Self, BeforeDoctype> {
        InRoot {
            node: self.node,
            children: self.node.children(),
            sub_state: BeforeDoctype,
            outer_state: self,
        }
    }
}

impl<'a> InRoot<'a, Root<'a>, BeforeDoctype> {
    #[track_caller]
    #[must_use] pub fn doctype(mut self) -> InRoot<'a, Root<'a>, AfterDoctype> {
        let child_node = self.children.next().expect("expected child but none left");
        let Some(_child_element) = child_node.value().as_doctype() else {
            panic!("unexpected element {:?}", child_node.value())
        };
        InRoot {
            node: self.node,
            children: self.children,
            sub_state: AfterDoctype,
            outer_state: self.outer_state,
        }
    }
}

impl<'a> InRoot<'a, Root<'a>, AfterDoctype> {
    pub fn end_document(mut self) {
        assert_eq!(self.children.next(), None);
    }
}

impl<'a, OuterState> InRoot<'a, OuterState, AfterDoctype> {
    #[track_caller]
    pub fn skip_whitespace(mut self) -> Self {
        let child_node = self.children.next().expect("expected child but none left");
        let Some(child_element) = child_node.value().as_text() else {
            panic!("unexpected element {:?}", child_node.value())
        };
        assert!(child_element.trim().is_empty(), "{child_element:?}");
        InRoot {
            node: self.node,
            children: self.children,
            sub_state: AfterDoctype,
            outer_state: self.outer_state,
        }
    }

    pub fn next_child_tag_open_start(mut self, name: &str) -> Open<'a, Self> {
        let child_node = self.children.next().expect("expected child but one left");
        let Some(child_element) = child_node.value().as_element() else {
            panic!("unexpected element {:?}", child_node.value())
        };
        assert_eq!(child_element.name(), name);
        Open {
            element: child_node,
            attrs: child_element.attrs(),
            outer_state: self,
        }
    }
}

impl<'a, OuterState> BeforeNode<'a, OuterState> {
    pub fn next_child_tag_open_start(self, name: &str) -> Open<'a, OuterState> {
        let Some(element) = self.node.value().as_element() else {
            panic!("unexpected element {:?}", self.node.value())
        };
        assert_eq!(element.name(), name);
        Open {
            element: self.node,
            attrs: element.attrs(),
            outer_state: self.outer_state,
        }
    }
}

impl<'a, OuterState> Open<'a, OuterState> {
    #[track_caller]
    pub fn attribute(mut self, name: &str, value: &str) -> Self {
        assert_eq!(
            self.attrs.next().expect("expected attribute but none left"),
            (name, value)
        );
        self
    }

    #[track_caller]
    pub fn attribute_value(mut self, expected_name: &str) -> (Self, String) {
        let (name, value) = self.attrs.next().expect("expected attribute but none left");
        assert_eq!(name, expected_name);
        (self, value.to_owned())
    }

    #[track_caller]
    pub fn tag_open_end(mut self) -> InElement<'a, OuterState> {
        let _element = self
            .element
            .value()
            .as_element()
            .expect("expected element but not an element");
        assert_eq!(self.attrs.next(), None, "unexpected attribute");
        InElement {
            element: self.element,
            children: self.element.children().peekable(),
            outer_state: self.outer_state,
        }
    }
}

impl<'a, OuterState> InElement<'a, OuterState> {
    pub fn peek(&mut self) -> Option<&NodeRef<'a, Node>> {
        self.children.peek()
    }

    pub fn next_any_child(mut self) -> (Self, NodeRef<'a, Node>) {
        let next_child = self.children.next().expect("expected child but none left");
        (self, next_child)
    }

    #[track_caller]
    pub fn skip_whitespace(mut self) -> Self {
        let child_node = self
            .children
            .next()
            .expect("expected child with text but got no children. maybe there is a closing tag?");
        let Some(child_element) = child_node.value().as_text() else {
            panic!("unexpected element {:?}", child_node.value())
        };
        assert!(child_element.trim().is_empty(), "{child_element:?}");
        self
    }

    #[track_caller]
    pub fn text(mut self) -> (Self, String) {
        let child_node = self
            .children
            .next()
            .expect("expected child with text but got no children. maybe there is a closing tag?");
        let Some(child_element) = child_node.value().as_text() else {
            panic!("unexpected element {:?}", child_node.value())
        };
        (self, child_element.to_string())
    }

    #[track_caller]
    pub fn skip_text(mut self, text: &str) -> Self {
        let child_node = self
            .children
            .next()
            .expect("expected child with text but got no children. maybe there is a closing tag?");
        let Some(child_element) = child_node.value().as_text() else {
            panic!("unexpected element {:?}", child_node.value())
        };
        match BASE64URL_NOPAD.decode(text.as_bytes()) {
            Ok(value) if value.len() == 32 => {
                let actual_hash = BASE64URL_NOPAD.encode(&Sha3_256::digest(&**child_element));
                assert_eq!(actual_hash, text);
            }
            _ => {
                assert_eq!(
                    &**child_element,
                    text,
                    "{}",
                    BASE64URL_NOPAD.encode(&Sha3_256::digest(&**child_element))
                );
            }
        }

        self
    }

    #[track_caller]
    pub fn skip_comment(mut self, expected_hash: &str) -> Self {
        let child_node = self.children.next().expect("expected child but none left");
        let Some(child_element) = child_node.value().as_comment() else {
            panic!("unexpected element {:?}", child_node.value())
        };
        let actual_hash = BASE64URL_NOPAD.encode(&Sha3_256::digest(&**child_element));
        assert_eq!(actual_hash, expected_hash);
        self
    }

    #[track_caller]
    pub fn next_child_tag_open_start(mut self, name: &str) -> Open<'a, Self> {
        let _element = self.element.value().as_element().expect("expected element");
        let child_node = self.children.next().expect("expected one more child");
        let Some(child_element) = child_node.value().as_element() else {
            panic!("unexpected element {:?}", child_node.value())
        };
        assert_eq!(child_element.name(), name);
        Open {
            element: child_node,
            attrs: child_node
                .value()
                .as_element()
                .expect("expected child to be element")
                .attrs(),
            outer_state: self,
        }
    }

    #[track_caller]
    pub fn close_element(mut self, name: &str) -> OuterState {
        assert_eq!(
            self.children.next().map(|child| child.value()),
            None,
            "expected there to be no more children"
        );
        assert_eq!(
            self.element
                .value()
                .as_element()
                .expect("expected element")
                .name(),
            name
        );
        self.outer_state
    }
}
