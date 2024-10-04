use std::marker::PhantomData;

use scraper::Element;
use scraper::{node::Attrs, ElementRef, Html};

pub struct BeforeElement<'a> {
    pub element: ElementRef<'a>,
}

pub struct Open<'a> {
    element: ElementRef<'a>,
    attrs: Attrs<'a>,
}

pub struct HtmlHandler<'a, State = BeforeElement<'a>> {
    pub state: State,
    pub phantom_data: PhantomData<&'a ()>,
}

impl<'a> HtmlHandler<'a, BeforeElement<'a>> {
    pub fn tag_open_start(self, name: &str) -> HtmlHandler<'a, Open<'a>> {
        assert_eq!(self.state.element.value().name(), name);

        HtmlHandler {
            state: Open {
                element: self.state.element,
                attrs: self.state.element.value().attrs(),
            },
            phantom_data: PhantomData,
        }
    }
}

impl<'a> HtmlHandler<'a, Open<'a>> {
    pub fn attribute(mut self, name: &str, value: &str) -> HtmlHandler<'a, Open<'a>> {
        assert_eq!(self.state.attrs.next().unwrap(), (name, value));
        HtmlHandler {
            state: Open {
                element: self.state.element,
                attrs: self.state.attrs,
            },
            phantom_data: PhantomData,
        }
    }

    pub fn tag_open_end(mut self) -> HtmlHandler<'a, BeforeElement<'a>> {
        assert_eq!(self.state.attrs.next(), None);
        // .next_sibling_element().unwrap(),
        HtmlHandler {
            state: BeforeElement {
                // TODO FIXME I think this skips text
                element: self.state.element.first_element_child().unwrap(),
            },
            phantom_data: PhantomData,
        }
    }
}
