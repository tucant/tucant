use std::marker::PhantomData;

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

    pub fn tag_open_end(mut self) -> BeforeElement<'a, OuterState> {
        assert_eq!(self.attrs.next(), None);
        // .next_sibling_element().unwrap(),
        BeforeElement {
            // TODO FIXME I think this skips text
            element: self.element.first_element_child().unwrap(),
            outer_state: self.outer_state,
        }
    }
}
