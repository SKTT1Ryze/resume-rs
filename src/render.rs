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
    Element,
    Section
};
use lazy_static::lazy_static;
use crate::{education::{EduInner, Degree}, info::{InfoInner}, template::{self, Template}, work::{WorkInner}};
use crate::{addtolength, ifhaveinfo, ifhavemonthyear, ifhavecityprovince, month};

lazy_static! {
    static ref TEST_NAME: String = String::from("XXX");
    static ref TEST_PHONE: String = String::from("+86 1234-5678-910");
    static ref TEST_EMAIL: String = String::from("1234567@89.com");
    static ref TEST_GITHUB: String = String::from("https://github.com/XXX");
    static ref TEST_SCHOOL: String = String::from("Kassel College");
    static ref TEST_MAJOR: String = String::from("Dragon Slayer");
    static ref TEST_PROVINCE: String = String::from("Hubei");
    static ref TEST_CITY: String = String::from("Wuhan");
}

pub struct Type1Render {}

impl Type1Render {
    pub fn new() -> Self {
        Self {}
    }

    fn render_typography(doc: &mut Document, template: impl Template) -> &mut Document{
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

    fn render_info<'a>(doc: &'a mut Document, info: Box<&'a dyn InfoInner>) -> &'a mut Document {
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
        let name = ifhaveinfo!(info.name());
        let phone = ifhaveinfo!(info.phone());
        let email = ifhaveinfo!(info.email());
        let github_url = ifhaveinfo!(info.github());
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

    fn render_education<'a>(doc: &'a mut Document, edu: &Vec<Box<&'a dyn EduInner>>) -> &'a mut Document {
        let mut section = Section::new("Education");
        for e in edu {
            section.push(r"\CVSubHeadingListStart
%    \CVSubheading % Example
%      {Degree Achieved}{Years of Study}
%      {Institution of Study}{Where it is located}
    \CVSubheading");
            let degree_shcool_major;
            match e.experience() {
                Degree::Undergraduate(school, major) => {
                    degree_shcool_major = ("Undergraduate", school, major);
                    
                },
                Degree::Master(school, major) => {
                    degree_shcool_major = ("Master", school, major);
                },
                Degree::Doctor(school, major) => {
                    degree_shcool_major = ("Doctor", school, major);
                },
                Degree::Other(_message) => {
                    todo!()    
                }
            }
            let year = e.to_inner().time().year();
            let month = e.to_inner().time().month();
            let time = ifhavemonthyear!(year, month);
            let province_city = ifhavecityprovince!(
                e.to_inner().situation().province(),
                e.to_inner().situation().city()
            );
            section.push(format!(
                r"        {{{{{} $|$ \emph{{\small{{Major: {}}}}}}}}}{{{}}}",
                degree_shcool_major.0,
                degree_shcool_major.2,
                time.as_str()
            ).as_str());
            section.push(format!(
                r"        {{{}}}{{{}}}",
                degree_shcool_major.1,
                province_city
            ).as_str());
            section.push(r"\CVSubHeadingListEnd");
        }
        doc.push(section);
        doc
    }

    fn render_work<'a>(doc: &'a mut Document, work: Box<&'a dyn WorkInner>) -> &'a mut Document {
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

#[macro_export]
macro_rules! ifhavemonthyear {
    ($y:expr, $m:expr) => {
        if let (Some(year), Some(month)) = ($y, $m) {
            format!("{} {} -- {} {}", month!(month.0), year.0, month!(month.1), year.1)
        } else {
            String::from("")
        }
    };
}

#[macro_export]
macro_rules! ifhavecityprovince {
    ($p:expr, $c:expr) => {
        if let (Some(province), Some(city)) = ($p, $c) {
            format!("{}, {}", city, province)
        } else {
            String::from("")
        }
    };
}

#[macro_export]
macro_rules! month {
    ($m:expr) => {
        match $m {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => panic!("Inexistent Month!")
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
fn type1_render_typography_test() {
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
    let document = Type1Render::render_typography(&mut document, template);
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
fn type1_render_info_test() {
    use crate::template::type1::TemplateType1;
    use crate::template::Template;
    let mut template = TemplateType1::new();
    template.personal_info(
        &(*TEST_NAME),
        &(*TEST_PHONE),
        &(*TEST_EMAIL),
        &(*TEST_GITHUB)
    );
    let resume = template.resume();
    let mut doc = Document::default();
    for elem in &resume.elements {
        if let Some(info) = elem.info_inner() {
            Type1Render::render_info(&mut doc, info);
        }
    }
    let should_be = format!(r"\documentclass{{article}}
\begin{{document}}
\begin{{minipage}}[c]{{0.05\textwidth}}
\-\
\end{{minipage}}
\begin{{minipage}}[c]{{0.2\textwidth}}
\begin{{tikzpicture}}
    \clip (0,0) circle (1.75cm);
    \node at (0,-.7) {{\includegraphics[width = 9cm]{{portrait}}}}; 
    % if necessary the picture may be moved by changing the at (coordinates)
    % width defines the 'zoom' of the picture
\end{{tikzpicture}}
\hfill\vline\hfill
\end{{minipage}}
\begin{{minipage}}[c]{{0.4\textwidth}}
    \textbf{{\Huge \scshape{{XXX}}}} \\ \vspace{{1pt}} 
    % \scshape sets small capital letters, remove if desired
    \small{{+86 1234-5678-910}} \\
    \href{{mailto:you@provider.com}}{{\underline{{1234567@89.com}}}}\\
    % Be sure to use a professional *personal* email address
    \href{{https://github.com/XXX}}{{\underline{{github.com/XXX}}}}
\end{{minipage}}
\end{{document}}
");
    assert_eq!(
        should_be,
        latex::print(&doc).unwrap()
    );
}

#[test]
fn type1_render_education_test() {
    use crate::template::type1::TemplateType1;
    use crate::template::Template;
    let mut template = TemplateType1::new();
    template.undergraduate(
        &(*TEST_SCHOOL),
        &(*TEST_MAJOR),
        (2015, 2019),
        (9, 7),
        (&(*TEST_PROVINCE), &(*TEST_CITY))
    );
    let resume = template.resume();
    let mut doc = Document::default();
    for elem in &resume.elements {
        if let Some(edu) = elem.edu_inner() {
            Type1Render::render_education(&mut doc, &edu);
        }
    }
    let should_be = format!(r"\documentclass{{article}}
\begin{{document}}
\section{{Education}}

\CVSubHeadingListStart
%    \CVSubheading % Example
%      {{Degree Achieved}}{{Years of Study}}
%      {{Institution of Study}}{{Where it is located}}
    \CVSubheading

        {{{{Undergraduate $|$ \emph{{\small{{Major: Dragon Slayer}}}}}}}}{{September 2015 -- July 2019}}

        {{Kassel College}}{{Wuhan, Hubei}}

\CVSubHeadingListEnd

\end{{document}}
");
    assert_eq!(
        should_be,
        latex::print(&doc).unwrap()
    );
}

#[test]
fn type1_render_work_test() {
    // TODO
}