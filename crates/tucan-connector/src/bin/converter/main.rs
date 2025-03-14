use std::fs;

use ego_tree::NodeRef;
use itertools::Itertools;
use scraper::{Html, Node};

#[must_use]
pub fn to_string(node: NodeRef<Node>, depth: usize) -> String {
    match node.value() {
        Node::Document => node.children().map(|child| to_string(child, 0)).join(""),
        Node::Fragment => todo!(),
        Node::Doctype(_doctype) => "<!doctype html>".to_owned(),
        Node::Comment(comment) => "\n".to_owned() + &" ".repeat(depth) + "<!--\"" + &comment.replace('"', "\\\"") + "\"-->",
        Node::Text(text) => {
            if text.trim().is_empty() {
                "_".to_owned()
            } else {
                "\"".to_owned() + &text.replace('\n', "\\n").replace('\t', "\\t").replace('"', "\\\"") + "\""
            }
        }
        Node::Element(element) => "\n".to_owned() + &" ".repeat(depth) + "<" + element.name() + &element.attrs.iter().map(|(key, value)| format!(" {}=\"{}\"", key.local, value)).join("") + ">" + &node.children().map(|child| to_string(child, depth + 2)).join("") + "</" + element.name() + ">",
        Node::ProcessingInstruction(_processing_instruction) => todo!(),
    }
}

pub fn main() {
    let document = Html::parse_document(&fs::read_to_string("input.html").unwrap());
    println!("{}", to_string(document.tree.root(), 0));
}
