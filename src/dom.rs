#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::collections::HashMap;

pub struct Node {
    // data common to all nodes:
    pub children: Vec<Node>,

    // data specific to each node type:
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

pub fn comment(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Comment(data) }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}

pub fn print_dom(root: &Node) {
    _print_dom_helper(root, 0);
}

fn _print_dom_helper(root: &Node, depth: usize) {
    match &root.node_type {
        NodeType::Text(text) => {
            println!("{} - Text: {}", "\t".repeat(depth), text);
        },
        NodeType::Comment(_) => {},
        NodeType::Element(e) => {
            println!("{} - {}", "\t".repeat(depth), e.tag_name);
            for (k, v) in e.attributes.iter() {
                println!("{} + {}: {}", "\t".repeat(depth+1), k, v)
            }
        },
    }
    for child in root.children.iter() {
        _print_dom_helper(child, depth+1);
    }
}
