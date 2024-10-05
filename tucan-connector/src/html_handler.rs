use std::marker::PhantomData;

use ego_tree::iter::Children;
use ego_tree::NodeRef;
use scraper::{node::Attrs, ElementRef, Html};
use scraper::{Element, Node};

pub struct BeforeNode<'a, OuterState> {
    pub node: NodeRef<'a, Node>,
    pub outer_state: OuterState,
}

pub struct Open<'a, OuterState> {
    element: NodeRef<'a, Node>,
    attrs: Attrs<'a>,
    pub outer_state: OuterState,
}

pub struct InElement<'a, OuterState> {
    pub element: NodeRef<'a, Node>,
    pub children: Children<'a, Node>,
    pub outer_state: OuterState,
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
        let child = self.children.next().unwrap();
        assert_eq!(child.value().as_element().unwrap().name(), name);
        Open {
            element: child,
            attrs: child.value().as_element().unwrap().attrs(),
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
