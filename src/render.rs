//! Rendering Latex Document
//!
//! Example:  
//! ```Rust
//! ```
//!

extern crate latex;
extern crate lazy_static;
use crate::{
    addtolength, ifhavecityprovince, ifhaveinfo, ifhavemonthyear, month, skill::SkillInner,
};
use crate::{
    education::{Degree, EduInner},
    honor::HonorInner,
    info::InfoInner,
    proj::ProjInner,
    template::Template,
    work::WorkInner,
};
use latex::{Document, Element, PreambleElement, Section};
use lazy_static::lazy_static;

lazy_static! {
    static ref TEST_NAME: String = String::from("XXX");
    static ref TEST_PHONE: String = String::from("+86 1234-5678-910");
    static ref TEST_EMAIL: String = String::from("1234567@89.com");
    static ref TEST_GITHUB: String = String::from("https://github.com/XXX");
    static ref TEST_SCHOOL: String = String::from("Kassel College");
    static ref TEST_MAJOR: String = String::from("Dragon Slayer");
    static ref TEST_PROVINCE: String = String::from("Hubei");
    static ref TEST_CITY: String = String::from("Wuhan");
    static ref TEST_COMPANY: String = String::from("RIOT");
    static ref TEST_POSITION: String = String::from("Game Developer");
    static ref TEST_CONTENT: Vec<String> = {
        vec![
            String::from("Developing LOL in RIOT"),
            String::from("Game Server Testing"),
        ]
    };
    static ref TEST_PROJ: String = String::from("RISC-V Processor With Rustlang");
    static ref TEST_GROUP: String = String::from("RISC-V Foundation");
    static ref TEST_LANG: String = String::from("Rust");
    static ref TEST_HONOR: String = String::from("S20 Champion");
    static ref TEST_DESCRIPTION: String = String::from("Rank One in the 20th LOL World Champions");
    static ref TEST_ITEM: (String, Vec<String>) = (
        String::from("Languages"),
        vec![
            String::from("Chinese"),
            String::from("English"),
            String::from("Japanese")
        ]
    );
}

pub struct Type1Render {}

impl Type1Render {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render<S>(
        name: &'static S,
        phone: &'static S,
        email: &'static S,
        github: &'static S,
        education: &'static Vec<(
            S,          // 0. school
            S,          // 1. major
            (u32, u32), // 2. year
            (u8, u8),   // 3. month
            (S, S),     // 4. situation
        )>,
        work: &'static Vec<(
            S,          // 0. company
            S,          // 1. position,
            Vec<S>,     // 2. content
            (u32, u32), // 3. year
            (u8, u8),   // 4. month
            (S, S),     // 5. situation
        )>,
        project: &'static Vec<(
            S,          // 0. project name
            S,          // 1. group,
            Option<S>,  // 2. lang
            (u32, u32), // 3. year
            (u8, u8),   // 4. month
        )>,
        honor: &'static Vec<(
            S,         // 0. honor name
            S,         // 1. honor description
            (u32, u8), // 2. honor time (year, month)
        )>,
        skill: &'static Vec<(S, Vec<S>)>,
    ) -> Document
    where
        S: AsRef<str>,
    {
        use crate::template::type1::TemplateType1;
        let mut doc = Document::default();
        let mut template = TemplateType1::new();
        doc.preamble.use_package("latexsym");
        doc.preamble.use_package("titlesec");
        doc.preamble.use_package("marvosym");
        doc.preamble.use_package("verbatim");
        doc.preamble.use_package("enumitem");
        doc.preamble.use_package("tabularx");
        doc.preamble.use_package("tikz");
        doc.preamble.use_package("palatino");
        doc.preamble.push(PreambleElement::UsePackage {
            package: String::from("fullpage"),
            argument: Some(String::from("empty")),
        });
        doc.preamble.push(PreambleElement::UsePackage {
            package: String::from("color"),
            argument: Some(String::from("usenames,dvipsnames")),
        });
        doc.preamble.push(PreambleElement::UsePackage {
            package: String::from("hyperref"),
            argument: Some(String::from("hidelinks")),
        });
        doc.preamble.push(PreambleElement::UsePackage {
            package: String::from("babel"),
            argument: Some(String::from("english")),
        });
        Self::render_typography(&mut doc, &template);
        doc.preamble.push(PreambleElement::UserDefined(String::from(
            r"\urlstyle{same}",
        )));
        doc.preamble
            .push(PreambleElement::UserDefined(String::from(r"\raggedbottom")));
        doc.preamble
            .push(PreambleElement::UserDefined(String::from(r"\raggedright")));
        doc.preamble.push(PreambleElement::UserDefined(String::from(
            r"\setlength{\tabcolsep}{0cm}",
        )));
        doc.preamble.push(PreambleElement::UserDefined(String::from(
            r"\titleformat{\section}{
\vspace{-4pt}\scshape\raggedright\large
}{}{0em}{}[\color{black}\titlerule \vspace{-5pt}]",
        )));
        doc.preamble.push(PreambleElement::UserDefined(String::from(
            r"\pdfgentounicode=1",
        )));
        doc.preamble.new_command(
            "CVItem",
            1,
            r"
\item\small{
    {#1 \vspace{-2pt}}
    }",
        );
        doc.preamble.new_command(
            "CVSubheading",
            4,
            r"
\vspace{-2pt}\item
    \begin{tabular*}{0.97\textwidth}[t]{l@{\extracolsep{\fill}}r}
        \textbf{#1} & #2 \\
        \small#3 & \small #4 \\
    \end{tabular*}\vspace{-7pt}",
        );
        doc.preamble.new_command(
            "CVSubSubheading",
            2,
            r"
    \item
    \begin{tabular*}{0.97\textwidth}{l@{\extracolsep{\fill}}r}
      \text{\small#1} & \text{\small #2} \\
    \end{tabular*}\vspace{-7pt}",
        );
        doc.preamble
            .new_command("CVSubItem", 1, r"\CVItem{#1}\vspace{-4pt}");
        doc.preamble.push(PreambleElement::UserDefined(String::from(
            r"\renewcommand\labelitemii{$\vcenter{\hbox{\tiny$\bullet$}}$}",
        )));
        doc.preamble.push(PreambleElement::UserDefined(String::from(
            r"\newcommand{\CVSubHeadingListStart}{\begin{itemize}[leftmargin=0.5cm, label={}]}",
        )));
        doc.preamble.push(PreambleElement::UserDefined(String::from(
            r"\newcommand{\CVSubHeadingListEnd}{\end{itemize}}",
        )));
        doc.preamble.push(PreambleElement::UserDefined(String::from(
            r"\newcommand{\CVItemListStart}{\begin{itemize}}",
        )));
        doc.preamble.push(PreambleElement::UserDefined(String::from(
            r"\newcommand{\CVItemListEnd}{\end{itemize}\vspace{-5pt}}",
        )));
        template.personal_info(name, phone, email, github);
        for d in education {
            template.undergraduate(&d.0, &d.1, d.2, d.3, (&d.4 .0, &d.4 .1));
        }
        for d in work {
            template.fulltime_work(&d.0, &d.1, &d.2, d.3, d.4, (&d.5 .0, &d.5 .1));
        }
        for d in project {
            template.project(&d.0, &d.1, d.2.as_ref(), (d.3 .0, d.3 .1), (d.4 .0, d.4 .1));
        }
        for d in honor {
            template.honor(&d.0, &d.1, (d.2 .0, d.2 .1));
        }
        for d in skill {
            template.skill(d);
        }
        for elem in &template.resume().elements {
            if let Some(info) = elem.info_inner() {
                Type1Render::render_info(&mut doc, info);
            }
        }
        Self::render_edu_head(&mut doc);
        for elem in &template.resume().elements {
            if let Some(edu) = elem.edu_inner() {
                Type1Render::render_education(&mut doc, &edu);
            }
        }
        Self::render_edu_tail(&mut doc);
        Self::render_work_head(&mut doc);
        for elem in &template.resume().elements {
            if let Some(work) = elem.work_inner() {
                Type1Render::render_work(&mut doc, &work);
            }
        }
        Self::render_work_tail(&mut doc);
        Self::render_proj_head(&mut doc);
        for elem in &template.resume().elements {
            if let Some(project) = elem.proj_inner() {
                Self::render_project(&mut doc, &project);
            }
        }
        Self::render_proj_tail(&mut doc);
        Self::render_honor_head(&mut doc);
        for elem in &template.resume().elements {
            if let Some(honor) = elem.honor_inner() {
                Self::render_honor(&mut doc, &honor);
            }
        }
        Self::render_honor_tail(&mut doc);
        Self::render_skill_head(&mut doc);
        for elem in &template.resume().elements {
            if let Some(skill) = elem.skill_inner() {
                Self::render_skill(&mut doc, &skill);
            }
        }
        Self::render_skill_tail(&mut doc);
        doc
    }

    pub fn render_typography<'a>(
        doc: &'a mut Document,
        template: &'a dyn Template,
    ) -> &'a mut Document {
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
            doc.preamble.push(PreambleElement::UserDefined(s));
        }
        doc
    }

    pub fn render_info<'a>(
        doc: &'a mut Document,
        info: Box<&'a dyn InfoInner>,
    ) -> &'a mut Document {
        let minipage = format!(
            r#"\begin{{minipage}}[c]{{0.05\textwidth}}
\-\
\end{{minipage}}"#
        );
        doc.push(Element::UserDefined(minipage));
        let minipage = String::from(
            r"\begin{minipage}[c]{0.2\textwidth}
\begin{tikzpicture}
    \clip (0,0) circle (1.75cm);
    \node at (0,-.7) {\includegraphics[width = 9cm]{portrait}}; 
    % if necessary the picture may be moved by changing the at (coordinates)
    % width defines the 'zoom' of the picture
\end{tikzpicture}
\hfill\vline\hfill
\end{minipage}",
        );
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
        let info = format!(
            r#"\begin{{minipage}}[c]{{0.4\textwidth}}
    \textbf{{\Huge \scshape{{{}}}}} \\ \vspace{{1pt}} 
    % \scshape sets small capital letters, remove if desired
    \small{{{}}} \\
    \href{{mailto:you@provider.com}}{{\underline{{{}}}}}\\
    % Be sure to use a professional *personal* email address
    \href{{{}}}{{\underline{{{}}}}}
\end{{minipage}}"#,
            name, phone, email, github_url, github_name
        );
        doc.push(Element::UserDefined(info));
        doc
    }

    pub fn render_edu_head(doc: &mut Document) -> &mut Document {
        let mut section = Section::new("Education");
        section.push(
            r"\CVSubHeadingListStart
%    \CVSubheading % Example
%      {Degree Achieved}{Years of Study}
%      {Institution of Study}{Where it is located}",
        );
        doc.push(section);
        doc
    }

    pub fn render_edu_tail(doc: &mut Document) -> &mut Document {
        let mut section = String::from("");
        section.push_str(r"\CVSubHeadingListEnd");
        doc.push(Element::UserDefined(section));
        doc
    }

    pub fn render_education<'a>(
        doc: &'a mut Document,
        edu: &Vec<Box<&'a dyn EduInner>>,
    ) -> &'a mut Document {
        let mut section = String::new();
        for e in edu {
            section.push_str("    \\CVSubheading\n");
            let degree_shcool_major;
            match e.experience() {
                Degree::Undergraduate(school, major) => {
                    degree_shcool_major = ("Undergraduate", school, major);
                }
                Degree::Master(school, major) => {
                    degree_shcool_major = ("Master", school, major);
                }
                Degree::Doctor(school, major) => {
                    degree_shcool_major = ("Doctor", school, major);
                }
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
            section.push_str(
                format!(
                    "        {{{{{} $|$ \\emph{{\\small{{Major: {}}}}}}}}}{{{}}}\n",
                    degree_shcool_major.0,
                    degree_shcool_major.2,
                    time.as_str()
                )
                .as_str(),
            );
            section.push_str(
                format!(
                    "        {{{}}}{{{}}}\n",
                    degree_shcool_major.1, province_city
                )
                .as_str(),
            );
        }
        doc.push(Element::UserDefined(section));
        doc
    }

    pub fn render_work_head(doc: &mut Document) -> &mut Document {
        let mut section = Section::new("Work Experience");
        section.push(
            r"\CVSubHeadingListStart
%    \CVSubheading %Example
%      {What you did}{When you worked there}
%      {Who you worked for}{Where they are located}
%      \CVItemListStart
%        \CVItem{Why it is important to this employer}
%      \CVItemListEnd",
        );
        doc.push(section);
        doc
    }

    pub fn render_work_tail(doc: &mut Document) -> &mut Document {
        let mut section = String::from("");
        section.push_str(r"\CVSubHeadingListEnd");
        doc.push(Element::UserDefined(section));
        doc
    }

    pub fn render_work<'a>(
        doc: &'a mut Document,
        work: &Vec<Box<&'a dyn WorkInner>>,
    ) -> &'a mut Document {
        let mut section = String::from("");
        for w in work {
            let company = w.company();
            let position = w.position();
            let content = w.content();
            let year = w.to_inner().time().year();
            let month = w.to_inner().time().month();
            let time = ifhavemonthyear!(year, month);
            let province_city = ifhavecityprovince!(
                w.to_inner().situation().province(),
                w.to_inner().situation().city()
            );
            let mut cv_items = String::from(r"    \CVItemListStart");
            for c in content {
                cv_items.push_str("\n        \\CVItem{");
                cv_items.push_str(c.as_str());
                cv_items.push_str("}");
            }
            cv_items.push_str("\n    \\CVItemListEnd");
            section.push_str(
                format!(
                    r"    \CVSubheading
    {{{}}}{{{}}}
    {{{}}}{{{}}}
{}",
                    position, time, company, province_city, cv_items
                )
                .as_str(),
            );
        }
        doc.push(Element::UserDefined(section));
        doc
    }

    pub fn render_proj_head(doc: &mut Document) -> &mut Document {
        let mut section = Section::new("Projects and Research");
        section.push(
            r"\CVSubHeadingListStart
%   \CVSubheading
%      {Title of Work}{When}
%      {Institution you worked with}{where}",
        );
        doc.push(section);
        doc
    }

    pub fn render_proj_tail(doc: &mut Document) -> &mut Document {
        let mut section = String::from("");
        section.push_str(r"\CVSubHeadingListEnd");
        doc.push(Element::UserDefined(section));
        doc
    }

    pub fn render_project<'a>(
        doc: &'a mut Document,
        projs: &Vec<Box<&'a dyn ProjInner>>,
    ) -> &'a mut Document {
        let mut section = String::from("");
        for p in projs {
            let proj_name = p.project();
            let group = p.group();
            let lang = match p.lang() {
                Some(l) => l,
                None => String::from(""),
            };
            let year = p.to_inner().time().year();
            let month = p.to_inner().time().month();
            let time = ifhavemonthyear!(year, month);
            let province_city = ifhavecityprovince!(
                p.to_inner().situation().province(),
                p.to_inner().situation().city()
            );
            section.push_str("    \\CVSubheading");
            section.push_str(
                format!(
                    r"
    {{{{{}}} $|$ \emph{{\small{{{}}}}}}}{{{}}}
    {{{}}}{{{}}}",
                    proj_name, lang, time, group, province_city
                )
                .as_str(),
            );
        }
        doc.push(Element::UserDefined(section));
        doc
    }

    pub fn render_honor_head(doc: &mut Document) -> &mut Document {
        let mut section = Section::new("Honors and Awards");
        section.push(
            r"\CVSubHeadingListStart
%   \CVSubheading
%      {What}{When}
%      {Short Description}{}",
        );
        doc.push(section);
        doc
    }

    pub fn render_honor_tail(doc: &mut Document) -> &mut Document {
        let mut section = String::from("");
        section.push_str(r"\CVSubHeadingListEnd");
        doc.push(Element::UserDefined(section));
        doc
    }

    pub fn render_honor<'a>(
        doc: &'a mut Document,
        honors: &Vec<Box<&'a dyn HonorInner>>,
    ) -> &'a mut Document {
        let mut section = String::from("");
        for h in honors {
            let honor_name = h.honor();
            let description = h.description();
            let time = h.time();
            let year = time.0;
            let month = match time.1 {
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
                _ => panic!("Inexistent Month!"),
            };
            section.push_str("    \\CVSubheading");
            section.push_str(
                format!(
                    r"
    {{{}}}{{{} {}}}
    {{{}}}{{}}",
                    honor_name, month, year, description
                )
                .as_str(),
            );
        }
        doc.push(Element::UserDefined(section));
        doc
    }

    pub fn render_skill_head(doc: &mut Document) -> &mut Document {
        let mut section = Section::new("Skills");
        section.push(
            r"\begin{itemize}[leftmargin=0.5cm, label={}]
    \small{\item{",
        );
        doc.push(section);
        doc
    }

    pub fn render_skill_tail(doc: &mut Document) -> &mut Document {
        let mut section = String::from("");
        section.push_str(
            r"    }}
\end{itemize}",
        );
        doc.push(Element::UserDefined(section));
        doc
    }

    pub fn render_skill<'a>(
        doc: &'a mut Document,
        skills: &Vec<Box<&'a dyn SkillInner>>,
    ) -> &'a mut Document {
        let mut section = String::new();
        for s in skills {
            let item = s.items();
            let k = item.0;
            let mut v = String::new();
            for i in item.1 {
                v.push_str(i.as_str());
                v.push_str(" ");
            }
            section.push_str(format!("    \\textbf{{{}}}{{: {}}} \\\\", k, v).as_str());
        }
        doc.push(Element::UserDefined(section));
        doc
    }
}

#[macro_export]
macro_rules! addtolength {
    ($s:expr, $x:expr, $e:expr) => {
        let command = format!(r"\addtolength{{\{}}}{{{}cm}}", $s, $x);
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
            format!(
                "{} {} -- {} {}",
                month!(month.0),
                year.0,
                month!(month.1),
                year.1
            )
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
            _ => panic!("Inexistent Month!"),
        }
    };
}

#[test]
fn addtolength_test() {
    let mut document = Document::default();
    addtolength!("oddsidemargin", -1, document);
    assert_eq!(
        latex::print(&document).unwrap(),
        String::from(
            r"\documentclass{article}
\addtolength{\oddsidemargin}{-1cm}
\begin{document}
\end{document}
"
        )
    );
}

#[test]
fn type1_render_typography_test() {
    use crate::template::Typography;
    use crate::Resume;
    struct SimpleTemplate {
        typography: SimpleTypography,
        resume: Resume,
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
                resume: Resume::default(),
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
    let document = Type1Render::render_typography(&mut document, &template);
    let should_be = format!(
        r"\documentclass{{article}}
\addtolength{{\oddsidemargin}}{{-1cm}}
\addtolength{{\evensidemargin}}{{-1cm}}
\addtolength{{\textwidth}}{{2cm}}
\addtolength{{\topmargin}}{{-1cm}}
\addtolength{{\textheight}}{{2cm}}
\begin{{document}}
\end{{document}}
"
    );
    assert_eq!(latex::print(&document).unwrap(), should_be);
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
        &(*TEST_GITHUB),
    );
    let resume = template.resume();
    let mut doc = Document::default();
    for elem in &resume.elements {
        if let Some(info) = elem.info_inner() {
            Type1Render::render_info(&mut doc, info);
        }
    }
    let should_be = format!(
        r"\documentclass{{article}}
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
"
    );
    assert_eq!(should_be, latex::print(&doc).unwrap());
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
        (&(*TEST_PROVINCE), &(*TEST_CITY)),
    );
    template.undergraduate(
        &(*TEST_SCHOOL),
        &(*TEST_MAJOR),
        (2015, 2019),
        (9, 7),
        (&(*TEST_PROVINCE), &(*TEST_CITY)),
    );
    let resume = template.resume();
    let mut doc = Document::default();
    let doc = Type1Render::render_edu_head(&mut doc);
    for elem in &resume.elements {
        if let Some(edu) = elem.edu_inner() {
            Type1Render::render_education(doc, &edu);
        }
    }
    Type1Render::render_edu_tail(doc);
    let should_be = format!(
        r"\documentclass{{article}}
\begin{{document}}
\section{{Education}}

\CVSubHeadingListStart
%    \CVSubheading % Example
%      {{Degree Achieved}}{{Years of Study}}
%      {{Institution of Study}}{{Where it is located}}

    \CVSubheading
        {{{{Undergraduate $|$ \emph{{\small{{Major: Dragon Slayer}}}}}}}}{{September 2015 -- July 2019}}
        {{Kassel College}}{{Wuhan, Hubei}}

    \CVSubheading
        {{{{Undergraduate $|$ \emph{{\small{{Major: Dragon Slayer}}}}}}}}{{September 2015 -- July 2019}}
        {{Kassel College}}{{Wuhan, Hubei}}

\CVSubHeadingListEnd
\end{{document}}
"
    );
    assert_eq!(should_be, latex::print(&doc).unwrap());
}

#[test]
fn type1_render_work_test() {
    use crate::template::type1::TemplateType1;
    use crate::template::Template;
    let mut template = TemplateType1::new();
    template.internship(
        &(*TEST_COMPANY),
        &(*TEST_POSITION),
        &(*TEST_CONTENT),
        (2020, 2025),
        (1, 1),
        (&(*TEST_PROVINCE), &(*TEST_CITY)),
    );
    template.internship(
        &(*TEST_COMPANY),
        &(*TEST_POSITION),
        &(*TEST_CONTENT),
        (2020, 2025),
        (1, 1),
        (&(*TEST_PROVINCE), &(*TEST_CITY)),
    );
    let resume = template.resume();
    let mut doc = Document::default();
    let doc = Type1Render::render_work_head(&mut doc);
    for elem in &resume.elements {
        if let Some(works) = elem.work_inner() {
            Type1Render::render_work(doc, &works);
        }
    }
    Type1Render::render_work_tail(doc);
    let should_be = format!(
        r"\documentclass{{article}}
\begin{{document}}
\section{{Work Experience}}

\CVSubHeadingListStart
%    \CVSubheading %Example
%      {{What you did}}{{When you worked there}}
%      {{Who you worked for}}{{Where they are located}}
%      \CVItemListStart
%        \CVItem{{Why it is important to this employer}}
%      \CVItemListEnd

    \CVSubheading
    {{Game Developer}}{{January 2020 -- January 2025}}
    {{RIOT}}{{Wuhan, Hubei}}
    \CVItemListStart
        \CVItem{{Developing LOL in RIOT}}
        \CVItem{{Game Server Testing}}
    \CVItemListEnd
    \CVSubheading
    {{Game Developer}}{{January 2020 -- January 2025}}
    {{RIOT}}{{Wuhan, Hubei}}
    \CVItemListStart
        \CVItem{{Developing LOL in RIOT}}
        \CVItem{{Game Server Testing}}
    \CVItemListEnd
\CVSubHeadingListEnd
\end{{document}}
"
    );
    assert_eq!(should_be, latex::print(&doc).unwrap());
}

#[test]
fn type1_render_proj_test() {
    use crate::template::type1::TemplateType1;
    use crate::template::Template;
    let mut template = TemplateType1::new();
    template.project(
        &(*TEST_PROJ),
        &(*TEST_GROUP),
        Some(&(*TEST_LANG)),
        (2030, 2032),
        (3, 3),
    );
    template.project(
        &(*TEST_PROJ),
        &(*TEST_GROUP),
        Some(&(*TEST_LANG)),
        (2030, 2032),
        (3, 3),
    );
    let resume = template.resume();
    let mut doc = Document::default();
    let doc = Type1Render::render_proj_head(&mut doc);
    for elem in &resume.elements {
        if let Some(projs) = elem.proj_inner() {
            Type1Render::render_project(doc, &projs);
        }
    }
    Type1Render::render_proj_tail(doc);
    let should_be = format!(
        r"\documentclass{{article}}
\begin{{document}}
\section{{Projects and Research}}

\CVSubHeadingListStart
%   \CVSubheading
%      {{Title of Work}}{{When}}
%      {{Institution you worked with}}{{where}}

    \CVSubheading
    {{{{RISC-V Processor With Rustlang}} $|$ \emph{{\small{{Rust}}}}}}{{March 2030 -- March 2032}}
    {{RISC-V Foundation}}{{}}
    \CVSubheading
    {{{{RISC-V Processor With Rustlang}} $|$ \emph{{\small{{Rust}}}}}}{{March 2030 -- March 2032}}
    {{RISC-V Foundation}}{{}}
\CVSubHeadingListEnd
\end{{document}}
"
    );
    assert_eq!(should_be, latex::print(&doc).unwrap());
}

#[test]
fn type1_render_honor_test() {
    use crate::template::type1::TemplateType1;
    use crate::template::Template;
    let mut template = TemplateType1::new();
    template.honor(&(*TEST_HONOR), &(*TEST_DESCRIPTION), (2030, 10));
    template.honor(&(*TEST_HONOR), &(*TEST_DESCRIPTION), (2030, 10));
    let resume = template.resume();
    let mut doc = Document::default();
    let doc = Type1Render::render_honor_head(&mut doc);
    for elem in &resume.elements {
        if let Some(honors) = elem.honor_inner() {
            Type1Render::render_honor(doc, &honors);
        }
    }
    Type1Render::render_honor_tail(doc);
    let should_be = format!(
        r"\documentclass{{article}}
\begin{{document}}
\section{{Honors and Awards}}

\CVSubHeadingListStart
%   \CVSubheading
%      {{What}}{{When}}
%      {{Short Description}}{{}}

    \CVSubheading
    {{S20 Champion}}{{October 2030}}
    {{Rank One in the 20th LOL World Champions}}{{}}
    \CVSubheading
    {{S20 Champion}}{{October 2030}}
    {{Rank One in the 20th LOL World Champions}}{{}}
\CVSubHeadingListEnd
\end{{document}}
"
    );
    assert_eq!(should_be, latex::print(&doc).unwrap());
}

#[test]
fn type1_render_skill_test() {
    use crate::template::type1::TemplateType1;
    use crate::template::Template;
    let mut template = TemplateType1::new();
    template.skill(&(*TEST_ITEM));
    template.skill(&(*TEST_ITEM));
    let resume = template.resume();
    let mut doc = Document::default();
    let doc = Type1Render::render_skill_head(&mut doc);
    for elem in &resume.elements {
        if let Some(skill) = elem.skill_inner() {
            Type1Render::render_skill(doc, &skill);
        }
    }
    Type1Render::render_skill_tail(doc);
    let should_be = format!(
        r"\documentclass{{article}}
\begin{{document}}
\section{{Skills}}

\begin{{itemize}}[leftmargin=0.5cm, label={{}}]
    \small{{\item{{

    \textbf{{Languages}}{{: Chinese English Japanese }} \\
    \textbf{{Languages}}{{: Chinese English Japanese }} \\
    }}}}
\end{{itemize}}
\end{{document}}
"
    );
    assert_eq!(should_be, latex::print(&doc).unwrap());
}
