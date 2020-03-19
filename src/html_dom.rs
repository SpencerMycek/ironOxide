#![allow(dead_code)]
#[deny(warnings)]
#[warn(rust_2018_idioms)]

use std::collections::HashMap;
use std::fmt;
use std::convert::TryFrom;
use std::convert::TryInto;
use super::Result;

pub static TEST_HTML: &str = "<!DOCTYPE html>
<html>
<head>
I should be able to put whatever I want into here without errors
</head>
</html>";


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
    pub fn new(name: HtmlTagType, text: &str) -> Self {
        let attrs = HashMap::new();
        let text = text;
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

    fn parse_html(html: &str) -> Node {
        let chars: Vec<char> = html.chars().collect();
        let mut index: usize = 0;
        let mut tag: String = String::new();
        
        for _ in 1..=2 {
            let result = next_tag(&chars, index);
            index = result.0;
            tag = result.1.trim().to_string();
        }
        let tag_end: isize = close_tag(&chars, index, &tag).unwrap();

        let size = tag.len();
        if &tag[1..size-1] == "html" {
            return Node::new(HtmlTagType::HTML, &html[index..usize::try_from(tag_end).unwrap()-size-1])
        } else {
            return Node::new(HtmlTagType::P, "Main Node")
        }
    }

    fn parse_htmlfrag(__html: &str) -> Node {
        return Node::new(HtmlTagType::P, "Frag Node");
    }
    
    fn parse_doctype(html: &str) -> Result<DocType> {
        let chars: Vec<char> = html.chars().collect();
        let doctype = next_tag(&chars, 0).1;

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

fn next_tag(html: &Vec<char>, mut index: usize) -> (usize, String) {
    let size = html.len();
    let mut tag = String::new();

    for (i, c) in (index..size).enumerate() {
        tag.push(html[c]);
        if html[c] == '>' {
            index += i+1;
            break
        }
    }
    return (index, tag);
}

fn close_tag(html: &Vec<char>, mut index: usize, tag: &String) -> Result<isize> {
    let size = html.len();
    let mut close_tag: String = tag.clone();
    close_tag.insert(1, '/');
    let mut buffer = String::new();

    for c in index..size {
            index += 1;
        if html[c] == ' ' || html[c] == '\n' {
            buffer.clear();
        } else {
            buffer.push(html[c]);
            if buffer == close_tag {
                return Ok(isize::try_from(index).unwrap())
            }
        }
    }
    Ok(-1)
}

