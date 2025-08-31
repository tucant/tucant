use std::fs;

use ego_tree::NodeRef;
use html_handler::{MyNode, parse_document};
use itertools::Itertools;

#[must_use]
pub fn to_string(node: NodeRef<MyNode>, depth: usize) -> String {
    match node.value() {
        MyNode::Document => node.children().map(|child| to_string(child, 0)).join(""),
        MyNode::Fragment => todo!(),
        MyNode::Doctype(_doctype) => "<!doctype html>".to_owned(),
        MyNode::Text(text) => "\"".to_owned() + &text.replace('\n', "\\n").replace('\t', "\\t").replace('"', "\\\"") + "\"",
        MyNode::Element(element) => {
            "\n".to_owned()
                + &" ".repeat(depth)
                + "<"
                + element.name()
                + &element.attrs.iter().map(|(key, value)| format!(" {}=\"{}\"", key.local, value)).join("")
                + ">"
                + &node.children().map(|child| to_string(child, depth + 2)).join("")
                + "</"
                + element.name()
                + ">"
        }
        MyNode::ProcessingInstruction(_processing_instruction) => todo!(),
    }
}

pub fn main() {
    let document = parse_document(&fs::read_to_string("input.html").unwrap());
    println!("{}", to_string(document.root(), 0));
}
