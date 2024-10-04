use std::marker::PhantomData;

use ego_tree::iter::Children;
use scraper::Element;
use scraper::{node::Attrs, ElementRef, Html};

pub struct BeforeElement<'a, OuterState> {
    pub element: ElementRef<'a>,
    pub outer_state: OuterState,
}

pub struct Open<'a, OuterState> {
    element: ElementRef<'a>,
    attrs: Attrs<'a>,
    pub outer_state: OuterState,
}

pub struct InElement<'a, OuterState, I: Iterator<Item = ElementRef<'a>>> {
    pub element: ElementRef<'a>,
    pub children: I,
    pub outer_state: OuterState,
}

impl<'a, OuterState> BeforeElement<'a, OuterState> {
    pub fn tag_open_start(self, name: &str) -> Open<'a, OuterState> {
        assert_eq!(self.element.value().name(), name);
        Open {
            element: self.element,
            attrs: self.element.value().attrs(),
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

    pub fn tag_open_end(
        mut self,
    ) -> InElement<'a, OuterState, impl Iterator<Item = ElementRef<'a>>> {
        assert_eq!(self.attrs.next(), None);
        // .next_sibling_element().unwrap(),
        // .first_element_child().unwrap()
        InElement {
            // TODO FIXME I think this skips text
            element: self.element,
            children: self.element.child_elements(),
            outer_state: self.outer_state,
        }
    }
}

impl<'a, OuterState, I: Iterator<Item = ElementRef<'a>>> InElement<'a, OuterState, I> {
    pub fn child_tag_open_start(mut self, name: &str) -> Open<'a, OuterState> {
        let child = self.children.next().unwrap();
        assert_eq!(child.value().name(), name);
        Open {
            element: child,
            attrs: child.value().attrs(),
            outer_state: self.outer_state,
        }
    }

    pub fn close_element(mut self) -> OuterState {
        assert_eq!(self.children.next(), None);
        // TODO FIXME verify same element
        self.outer_state
    }
}
