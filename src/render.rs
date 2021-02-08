//! Rendering Latex Document
//! 
//! Example:  
//! ```Rust
//! ```
//! 

extern crate latex;

use latex::{Document, PreambleElement};
use crate::{
    education::Education,
    info::PersonalInfo,
    template::Template,
    work::Work
};
use crate::addtolength;

pub struct Render {}

impl Render {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render_typography(doc: &mut Document, template: impl Template) -> &mut Document{
        let typography = template.typography();
        if let Some(x) = typography.oddsidemargin() {
            addtolength!("oddsidemargin", x, doc);
        }
        if let Some(x) = typography.evensidemargin() {
            addtolength!("evensidemargin", x, doc);
        }
        if let Some(x) = typography.textwidth() {
            addtolength!("textwidth", x, doc);
        }
        if let Some(x) = typography.topmargin() {
            addtolength!("topmargin", x, doc);
        }
        if let Some(x) = typography.textheight() {
            addtolength!("textheight", x, doc);
        }
        if let Some(s) = typography.other() {
            doc.preamble.push(
                PreambleElement::UserDefined(s)
            );
        }
        doc
    }

    pub fn render_info(doc: &mut Document, info: Box<PersonalInfo>) -> &mut Document {
        todo!();
        doc
    }

    pub fn render_education(doc: &mut Document, edu: Box<Education>) -> &mut Document {
        todo!();
        doc
    }

    pub fn render_work(doc: &mut Document, work: Box<Work>) -> &mut Document {
        todo!();
        doc
    }
}

#[macro_export]
macro_rules! addtolength {
    ($s:expr, $x:expr, $e:expr) => {
        let command = format!(r"\addtolength{{\{}}}{{{}cm}}",$s, $x);
            let preamble_elem = PreambleElement::UserDefined(command);
            $e.preamble.push(preamble_elem);
    };
}

#[test]
fn addtolength_test() {
    let mut document = Document::default();
    addtolength!("oddsidemargin", -1, document);
    assert_eq!(
        latex::print(&document).unwrap(),
        String::from(r"\documentclass{article}
\addtolength{\oddsidemargin}{-1cm}
\begin{document}
\end{document}
")
    );
}