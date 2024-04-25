/*
    HTML template requirements:

    handle void elements

    don't add closing slashes (not valid html)

    do not add "on" events to elements,

    this will require knowing parent element and last attribute

    should there be an all or nothing quit attidue? if one wrong thing happens
    than nothing is built
*/

use txml::{StackBit, Template, TxmlBuilder};

#[derive(Debug)]
pub enum Injection<'a, E> {
    Text(&'a str),
    Attr(&'a str),
    AttrValue(&'a str, &'a str),
    Callback(&'a str, E),
    Template(Template<'a, E>),
    List(Vec<Injection<'a, E>>),
}

type NonCallback = ();

pub struct StaticHtmlBuilder<'a> {
    result: String,
    tab_count: usize,
    stack: Vec<StackBit<'a, Injection<'a, NonCallback>>>,
}

impl<'a> StaticHtmlBuilder<'_> {
    // eventually this is the cache step ::new(1024) max build steps
    pub fn new() -> StaticHtmlBuilder<'a> {
        StaticHtmlBuilder {
            result: "".to_string(),
            tab_count: 0,
            stack: Vec::new(),
        }
    }

    pub fn build(&self) -> String {
        self.result.clone()
    }

    pub fn reset(mut self) {
        self = StaticHtmlBuilder {
            result: "".to_string(),
            tab_count: 0,
            stack: Vec::new(),
        };
    }
}

// tale of two builders
// TemplateBuilder for caching -> { text: Vec(), descendants: [] }
// StaticHtmlBuilder for the actual page page

impl<'a> TxmlBuilder<'a, Injection<'a, NonCallback>> for StaticHtmlBuilder<'_> {
    // steps
    fn push_node(&self, tag: &'a str) {}
    fn add_attr(&self, attr: &'a str) {}
    fn add_attr_value(&self, value: &'a str) {}
    fn push_text(&self, text: &'a str) {}
    fn pop_node(&self, tag: &'a str) {}
    fn pop_independent_node(&self) {}

    // injections
    fn add_attr_map(&self, injections: Injection<'a, NonCallback>) {}
    fn get_descendants(
        &self,
        injections: Injection<'a, NonCallback>,
    ) -> Vec<StackBit<'a, Injection<'a, NonCallback>>> {
        //
        Vec::new()
    }
}

// Injections could be entirely external to the "builder"

// where E is for event callbacks

fn is_html_void_element(tag: &str) -> bool {
    match tag {
        "area" => true,
        "base" => true,
        "br" => true,
        "col" => true,
        "embed" => true,
        "hr" => true,
        "img" => true,
        "input" => true,
        "link" => true,
        "meta" => true,
        "param" => true,
        "source" => true,
        "track" => true,
        "wbr" => true,
        _ => false,
    }
}

fn add_close_tagname(result: &mut String, tab_count: usize, text: &str) -> () {
    // tab_count -= 1;
    result.push_str(&"\t".repeat(tab_count));
    result.push_str("</");
    result.push_str(text);
    result.push_str(">\n");
}

fn add_independent_node(result: &mut String, tab_count: usize, text: &str) -> () {
    result.push_str("/>\n");
    // tab_count -= 1;
}

fn add_node_closed(result: &mut String, tab_count: usize, text: &str) -> () {
    result.push_str(">\n");
    // tab_count += 1;
}

fn add_tag(result: &mut String, tab_count: usize, text: &str) -> () {
    result.push_str(&"\t".repeat(tab_count));
    result.push_str("<");
    result.push_str(text);
}

fn add_text(result: &mut String, tab_count: usize, text: &str) -> () {
    result.push_str(&"\t".repeat(tab_count));
    result.push_str(text.trim());
    result.push_str("\n");
}

fn add_attr(result: &mut String, attr: &str) -> () {
    result.push_str(" ");
    result.push_str(attr);
}

fn add_attr_value(result: &mut String, attr: &str, value: &str) -> () {
    result.push_str(" ");
    result.push_str(attr);
    result.push_str("=\"");
    result.push_str(value);
    result.push_str("\"");
}

//
pub fn html<'a, T>(template_str: &'a str, injections: Vec<T>) -> Template<'a, T> {
    Template {
        kind: "html",
        template_str: template_str,
        injections: injections,
    }
}