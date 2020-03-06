#![allow(dead_code)]
#[deny(warnings)]
#[warn(rust_2018_idioms)]

use std::collections::HashMap;
use std::fmt;
use super::Result;

pub enum HtmlTagType {
    TEXT, // Special tag type, used to Store untagged text
    DOCTYPE,
    A,
    BR,
    DIV,
    H1,
    HEAD,
    HR,
    HTML,
    META,
    NAV,
    P,
    SPAN,
    TITLE,
}

pub struct Tag {
    pub name: HtmlTagType,
    pub attrs: HashMap<String, String>,
    pub text: String,
}

impl Tag {
    pub fn new(name: HtmlTagType, attrs: HashMap<String, String>, text: &str) -> Self {
        let text = text.to_string();
        Tag { name, attrs, text }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Tag {
    pub fn to_string(&self) -> &String {
        return &self.text;
    }
}


pub struct Node {
    pub tag: Tag,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(name: HtmlTagType) -> Self {
        let attrs = HashMap::new();
        let text = "Main Node";
        let tag = Tag::new(name, attrs, text);
        let children = Vec::<Node>::new();
        Node { tag, children }
    }

    pub fn add_child(& mut self, node: Node) {
        self.children.push(node);
    }

    pub fn to_string(&self, depth: u16) -> String {
        let mut text = self.tag.to_string().to_owned();
        text.push('\n');
        for node in &self.children {
            for _ in 0..depth+1 {
                text.push('\t');
            }
            text += &node.to_string(depth+1);
            text.push('\n');
        }
        return text;
    }
}


pub enum DocType {
    HTML5,
}

pub struct Dom {
    pub root: Node,
    pub doctype: DocType
}

impl Dom {
    
    pub fn parse_fragment(fragment: &str) -> Self {
        let root = Dom::parse_htmlfrag(fragment);
        let doctype = DocType::HTML5;
        Dom { root, doctype }
    }
    
    pub fn parse_document(document: &str) -> Result<Self> {
        let root = Dom::parse_html(document);
        let doctype = Dom::parse_doctype(document)?;
        Ok(Dom { root, doctype })
    }

    pub fn root_element(&self) -> &Node {
        return &self.root;
    }

    fn parse_html(__html: &str) -> Node {
        return Node::new(HtmlTagType::P);
    }

    fn parse_htmlfrag(__html: &str) -> Node {
        return Node::new(HtmlTagType::P);
    }
    
    fn parse_doctype(html: &str) -> Result<DocType> {
        let mut doctype: String = String::from("");
        for c in html.chars() {
            doctype.push(c);
            if c == '>' {
                break;
            }
        }
        let doctype = doctype.replace(&['<', '>'][..], "");
        let size = doctype.split(' ').count();

        for (i, string) in doctype.split(' ').enumerate() {
            if i == 0 && string != "!DOCTYPE" {
                return Err(Box::from("No DOCTYPE found"));
            } else if i == 1 {
                if string == "html" && size == 2 {
                    return Ok(DocType::HTML5);
                }
            }
        }
        return Err(Box::from("No matching DOCTYPE"));
    }
}

