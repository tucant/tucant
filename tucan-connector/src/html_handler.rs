use std::marker::PhantomData;

use ego_tree::iter::Children;
use ego_tree::{NodeRef, Tree};
use scraper::{node::Attrs, ElementRef, Html};
use scraper::{Element, Node};

pub struct Root<'a> {
    node: NodeRef<'a, Node>,
}

pub struct BeforeDoctype;

pub struct AfterDoctype;

pub struct InRoot<'a, OuterState, RootSubState> {
    node: NodeRef<'a, Node>,
    children: Children<'a, Node>,
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
    children: Children<'a, Node>,
    outer_state: OuterState,
}

impl<'a> Root<'a> {
    pub fn new(node: NodeRef<'a, Node>) -> Self {
        assert_eq!(*node.value(), Node::Document);
        Self { node }
    }

    pub fn document_start(self) -> InRoot<'a, Root<'a>, BeforeDoctype> {
        InRoot {
            node: self.node,
            children: self.node.children(),
            sub_state: BeforeDoctype,
            outer_state: self,
        }
    }
}

// TODO outer state here could be hardcoded to Root
impl<'a, OuterState> InRoot<'a, OuterState, BeforeDoctype> {
    pub fn doctype(mut self) -> InRoot<'a, OuterState, AfterDoctype> {
        let child_node = self.children.next().unwrap();
        let child_element = child_node
            .value()
            .as_doctype()
            .unwrap_or_else(|| panic!("unexpected element {:?}", child_node.value()));
        InRoot {
            node: self.node,
            children: self.children,
            sub_state: AfterDoctype,
            outer_state: self.outer_state,
        }
    }
}

impl<'a, OuterState> InRoot<'a, OuterState, AfterDoctype> {
    pub fn tag_open_start(mut self, name: &str) -> Open<'a, InRoot<'a, OuterState, AfterDoctype>> {
        let child_node = self.children.next().unwrap();
        let child_element = child_node
            .value()
            .as_element()
            .unwrap_or_else(|| panic!("unexpected element {:?}", child_node.value()));
        assert_eq!(child_element.name(), name);
        Open {
            element: child_node,
            attrs: child_element.attrs(),
            outer_state: self,
        }
    }
}

impl<'a, OuterState> BeforeNode<'a, OuterState> {
    pub fn tag_open_start(self, name: &str) -> Open<'a, OuterState> {
        let element = self
            .node
            .value()
            .as_element()
            .unwrap_or_else(|| panic!("unexpected element {:?}", self.node.value()));
        assert_eq!(element.name(), name);
        Open {
            element: self.node,
            attrs: element.attrs(),
            outer_state: self.outer_state,
        }
    }
}

impl<'a, OuterState> Open<'a, OuterState> {
    pub fn attribute(mut self, name: &str, value: &str) -> Open<'a, OuterState> {
        assert_eq!(self.attrs.next().unwrap(), (name, value));
        Open {
            element: self.element,
            attrs: self.attrs,
            outer_state: self.outer_state,
        }
    }

    pub fn tag_open_end(mut self) -> InElement<'a, OuterState> {
        let element = self.element.value().as_element().unwrap();
        assert_eq!(self.attrs.next(), None);
        // .next_sibling_element().unwrap(),
        // .first_element_child().unwrap()
        InElement {
            element: self.element,
            children: self.element.children(),
            outer_state: self.outer_state,
        }
    }
}

impl<'a, OuterState> InElement<'a, OuterState> {
    pub fn child_tag_open_start(mut self, name: &str) -> Open<'a, InElement<'a, OuterState>> {
        let element = self.element.value().as_element().unwrap();
        let child_node = self.children.next().unwrap();
        let child_element = child_node
            .value()
            .as_element()
            .unwrap_or_else(|| panic!("unexpected element {:?}", child_node.value()));
        assert_eq!(child_element.name(), name);
        Open {
            element: child_node,
            attrs: child_node.value().as_element().unwrap().attrs(),
            outer_state: InElement {
                element: self.element,
                children: self.children,
                outer_state: self.outer_state,
            },
        }
    }

    pub fn close_element(mut self) -> OuterState {
        assert_eq!(self.children.next(), None);
        // TODO FIXME verify same element
        self.outer_state
    }
}
