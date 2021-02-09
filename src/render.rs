//! Rendering Latex Document
//! 
//! Example:  
//! ```Rust
//! ```
//! 

extern crate latex;
extern crate lazy_static;
use latex::{
    Document,
    PreambleElement,
    Element
};
use lazy_static::lazy_static;
use crate::{education::Education, info::PersonalInfo, template::{Template}, work::Work};
use crate::{addtolength, ifhaveinfo};

lazy_static! {
    static ref TEST_NAME: String = String::from("XXX");
    static ref TEST_PHONE: String = String::from("+86 1234-5678-910");
    static ref TEST_EMAIL: String = String::from("1234567@89.com");
    static ref TEST_GITHUB: String = String::from("https://github.com/XXX");
}

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
        let minipage = format!(r#"\begin{{minipage}}[c]{{0.05\textwidth}}
\-\
\end{{minipage}}"#);
        doc.push(Element::UserDefined(minipage));
        let minipage = String::from(r"\begin{minipage}[c]{0.2\textwidth}
\begin{tikzpicture}
\clip (0,0) circle (1.75cm);
\node at (0,-.7) {\includegraphics[width = 9cm]{portrait}}; 
% if necessary the picture may be moved by changing the at (coordinates)
% width defines the 'zoom' of the picture
\end{tikzpicture}
\hfill\vline\hfill
\end{minipage}");
        doc.push(Element::UserDefined(minipage));
        let name = ifhaveinfo!(info.inner.name());
        let phone = ifhaveinfo!(info.inner.phone());
        let email = ifhaveinfo!(info.inner.email());
        let github_url = ifhaveinfo!(info.inner.github());
        let mut github_name = String::from(github_url.as_str());
        assert!(github_name.len() > 8);
        for _ in 0..8 {
            github_name.remove(0);
        }
        let info = format!(r#"\begin{{minipage}}[c]{{0.4\textwidth}}
    \textbf{{\Huge \scshape{{{}}}}} \\ \vspace{{1pt}} 
    % \scshape sets small capital letters, remove if desired
    \small{{{}}} \\
    \href{{mailto:you@provider.com}}{{\underline{{{}}}}}\\
    % Be sure to use a professional *personal* email address
    \href{{{}}}{{\underline{{{}}}}}
\end{{minipage}}"#, name, phone, email, github_url, github_name);
        doc.push(Element::UserDefined(info));
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

#[macro_export]
macro_rules! ifhaveinfo {
    ($o:expr) => {
        if let Some(item) = $o {
            item
        } else {
            String::from("")
        }
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

#[test]
fn render_typography_test() {
    use crate::Resume;
    use crate::template::Typography;
    struct SimpleTemplate {
        typography: SimpleTypography,
        resume: Resume
    };
    impl Template for SimpleTemplate {
        fn typography(&self) -> Box<dyn Typography> {
            Box::new(self.typography)
        }
        fn resume(&self) -> &Resume {
            &self.resume
        }
    }
    impl SimpleTemplate {
        pub fn new() -> Self {
            Self {
                typography: SimpleTypography {},
                resume: Resume::default()
            }
        }
    }
    #[derive(Clone, Copy)]
    struct SimpleTypography {}
    impl Typography for SimpleTypography {
        fn oddsidemargin(&self) -> Option<i32> {
            Some(-1)
        }
        fn evensidemargin(&self) -> Option<i32> {
            Some(-1)
        }
        fn textwidth(&self) -> Option<i32> {
            Some(2)
        }
        fn topmargin(&self) -> Option<i32> {
            Some(-1)
        }
        fn textheight(&self) -> Option<i32> {
            Some(2)
        }
        fn other(&self) -> Option<String> {
            None
        }
    }

    let template = SimpleTemplate::new();
    let mut document = Document::default();
    let document = Render::render_typography(&mut document, template);
    let should_be = format!(r"\documentclass{{article}}
\addtolength{{\oddsidemargin}}{{-1cm}}
\addtolength{{\evensidemargin}}{{-1cm}}
\addtolength{{\textwidth}}{{2cm}}
\addtolength{{\topmargin}}{{-1cm}}
\addtolength{{\textheight}}{{2cm}}
\begin{{document}}
\end{{document}}
");
    assert_eq!(
        latex::print(&document).unwrap(),
        should_be
    );
}

#[test]
fn render_info_test() {
    use crate::template::type1::TemplateType1;
    let mut template = TemplateType1::new();
    template.personal_info(
        &(*TEST_NAME),
        &(*TEST_PHONE),
        &(*TEST_EMAIL),
        &(*TEST_GITHUB)
    );
    todo!()
}

#[test]
fn render_education_test() {
    // TODO
}

#[test]
fn render_work_test() {
    // TODO
}