use parsley::{get_text_from_step, parse_str_with_reserved_tags, ParsleySieve, Step, StepKind};
use txml::Template;

mod safe_html;

use safe_html::{SafetySieve, TagInfo};

/*
    This is meant to "pretty" an html document output by `html`.
    it is not used by a "doc" builder
*/

struct PretyHtmlBuilder {}

impl PretyHtmlBuilder {
    pub fn new() -> PretyHtmlBuilder {
        PretyHtmlBuilder {}
    }

    // add decision sieve here:
    //
    // self, html_str, sieve
    pub fn build(&self, sieve: impl SafetySieve + ParsleySieve, html_str: &str) -> String {
        // check for already built results
        let mut results = "".to_string();

        // parse string with reserved words
        for step in parse_str_with_reserved_tags(sieve, &html_str, StepKind::Initial) {
            // push_step(&mut results, &html_str, step);
        }

        results
    }
}

// pre elements must respsect boundaries
// that's really it use

fn push_step(results: &mut String, sieve: impl SafetySieve, template_str: &str, step: Step) {
    match step.kind {
        // steps
        // StepKind::Tag => push_element(results, step)
        // StepKind::ElementClosed => close_element(results, step)
        // StepKind::VoidElementClosed => close_void_element(results, step)
        // StepKind::Attr => add_attr(results, step)
        // StepKind::AttrValue => add_attr_value(results, step)
        // StepKind::AttrValueUnquoted => add_attr_value_unquoted(results, step)
        // StepKind::Text => push_text(results, step)
        // StepKind::TailTag => pop_element(results, step)
        // // injections
        // StepKind::AttrMapInjection => push_attr_map_injection(results, step)
        // StepKind::DescendantInjection => push_descendant_injection(results, step)
        // StepKind::InjectionSpace => push_injection_space(results, step)
        // StepKind::InjectionConfirmed => push_injection_confirmed(results, step)
        // all other steps silently pass through
        _ => {}
    }
}

// fn push_element(&mut results: String, step: Step) {
//     let tag = get_text_from_step(template_str, &step)
//     tags.push(tag.to_string());

//     results.push('\n');
//     results.push_str(&"\t".repeat(tab_count));
//     results.push_str("<");
//     results.push_str(tag);
// }

// fn close_element(&mut results: String, step: Step) {
//     results.push_str(">");
// }

// fn close_void_element(&mut results: String, step: Step) {
//     results.push_str(">");
// }

// fn pop_element(&mut results: String, step: Step) {
//     results.push('\n');
//     results.push_str(&"\t".repeat(tab_count));
//     results.push_str("</");
//     results.push_str(tag);
//     results.push_str(">");
// }

// fn push_text(&mut results: String, text: &str) {
//     results.push_str(text);
// }

// fn add_attr(&mut results: String, step: Step) {
//     results.push(' ');
//     results.push_str(tag);
// }

// fn add_attr_value(&mut results: String, step: Step) {
//     results.push_str("=\"");
//     results.push_str(tag);
//     results.push('"');
// }

// fn add_attr_value_unquoted(&mut results: String, step: Step) {
//     results.push('=');
//     results.push_str(tag);
// }

// // injections
// fn push_attr_map_injection(&mut results: String, glyph: &str) {
//     results.push_str(glyph);
// }

// fn push_descendant_injection(&mut results: String, glyph: &str) {
//     results.push_str(glyph);
// }

// fn push_injection_space(&mut results: String, text: &str) {
//     results.push_str(text);
// }

// fn push_injection_confirmed(&mut results: String, glyph: &str) {
//     results.push_str(glyph);
// }
