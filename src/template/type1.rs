//! Type1 Implementation of `Template` trait
//!
//! Example:  
//! ```Rust
//! ```
//!

use education::{Degree, EduInner, Education};
use honor::Honor;
use info::{InfoInner, PersonalInfo};
use proj::Project;
use work::{Work, WorkClass, WorkInner};

use super::{Template, Typography};
use crate::*;

pub struct TemplateType1 {
    typography: Type1Typography,
    resume: Resume,
}

impl Template for TemplateType1 {
    fn resume(&self) -> &Resume {
        &self.resume
    }

    fn typography(&self) -> Box<dyn Typography> {
        Box::new(self.typography)
    }
}

impl TemplateType1 {
    pub fn new() -> Self {
        Self {
            typography: Type1Typography,
            resume: Resume::default(),
        }
    }

    pub fn personal_info<S>(
        &mut self,
        name: &'static S,
        phone: &'static S,
        email: &'static S,
        github: &'static S,
    ) -> &mut Self
    where
        S: AsRef<str>,
    {
        let type1_info_inner = Type1InfoInner(
            name.as_ref(),
            phone.as_ref(),
            email.as_ref(),
            github.as_ref(),
            Type1InfoTimeSituation::default(),
        );
        let type1_info = PersonalInfo::new(type1_info_inner);
        self.resume.append_element(type1_info);
        self
    }

    pub fn undergraduate<S>(
        &mut self,
        school: &'static S,
        major: &'static S,
        year: (u32, u32),
        month: (u8, u8),
        situation: (&'static S, &'static S),
    ) -> &mut Self
    where
        S: AsRef<str>,
    {
        let type1_edu_inner = Type1Undergraduate {
            school: school.as_ref(),
            major: major.as_ref(),
            time_situation: Type1UndergraduateTimeSituation(
                Type1UndergraduateTime { year, month },
                Type1UndergraduateSituation {
                    province: String::from(situation.0.as_ref()),
                    city: String::from(situation.1.as_ref()),
                },
            ),
        };
        let mut type1_degree = Education::default();
        type1_degree.append_inner(type1_edu_inner);
        self.resume.append_element(type1_degree);
        self
    }

    pub fn internship<S>(
        &mut self,
        company: &'static S,
        position: &'static S,
        content: &'static Vec<S>,
        year: (u32, u32),
        month: (u8, u8),
        situation: (&'static S, &'static S),
    ) -> &mut Self
    where
        S: AsRef<str>,
    {
        let c: Vec<&str> = content.iter().map(|c| c.as_ref()).collect();
        let type1_internship_inner = Type1Internship {
            company: company.as_ref(),
            position: position.as_ref(),
            content: c,
            time_situation: Type1WorkTimeSituation(
                Type1WorkTime { year, month },
                Type1WorkSituation {
                    province: String::from(situation.0.as_ref()),
                    city: String::from(situation.1.as_ref()),
                },
            ),
        };
        let mut type1_internship = Work::default();
        type1_internship.append_inner(type1_internship_inner);
        self.resume.append_element(type1_internship);
        self
    }

    pub fn fulltime_work<S>(
        &mut self,
        company: &'static S,
        position: &'static S,
        content: &'static Vec<S>,
        year: (u32, u32),
        month: (u8, u8),
        situation: (&'static S, &'static S),
    ) -> &mut Self
    where
        S: AsRef<str>,
    {
        let c: Vec<&str> = content.iter().map(|c| c.as_ref()).collect();
        let type1_fulltime_inner = Type1FullTimeWork {
            company: company.as_ref(),
            position: position.as_ref(),
            content: c,
            time_situation: Type1WorkTimeSituation(
                Type1WorkTime { year, month },
                Type1WorkSituation {
                    province: String::from(situation.0.as_ref()),
                    city: String::from(situation.1.as_ref()),
                },
            ),
        };
        let mut type1_fulltime = Work::default();
        type1_fulltime.append_inner(type1_fulltime_inner);
        self.resume.append_element(type1_fulltime);
        self
    }

    pub fn project<S>(
        &mut self,
        proj_name: &'static S,
        group: &'static S,
        lang: Option<&'static S>,
        year: (u32, u32),
        month: (u8, u8),
    ) -> &mut Self
    where
        S: AsRef<str>,
    {
        let type1_proj_inner = Type1ProjectInner {
            proj_name: proj_name.as_ref(),
            group: group.as_ref(),
            lang: match lang {
                Some(l) => Some(l.as_ref()),
                None => None,
            },
            time_situation: Type1ProjTimeSituation(
                Type1ProjTime { year, month },
                Type1ProjSituation::default(),
            ),
        };
        let mut type1_project = Project::default();
        type1_project.append_inner(type1_proj_inner);
        self.resume.append_element(type1_project);
        self
    }

    pub fn honor<S>(
        &mut self,
        honor_name: &'static S,
        description: &'static S,
        time: (u32, u8), // (year, month)
    ) -> &mut Self
    where
        S: AsRef<str>,
    {
        let type1_honor_inner = Type1HonorInner {
            honor: honor_name.as_ref(),
            description: description.as_ref(),
            time: time,
            time_situation: Type1HonorTimeSituation::default(),
        };
        let mut type1_honor = Honor::default();
        type1_honor.append_inner(type1_honor_inner);
        self.resume.append_element(type1_honor);
        self
    }
}

#[derive(Clone, Copy)]
pub struct Type1Typography;

impl Typography for Type1Typography {
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
}

pub struct Type1InfoInner<'a>(&'a str, &'a str, &'a str, &'a str, Type1InfoTimeSituation);

impl<'a> IntoInner for Type1InfoInner<'a> {
    fn to_inner(&self) -> Box<dyn Inner> {
        Box::new(self.4)
    }
}

impl<'a> InfoInner for Type1InfoInner<'a> {
    fn name(&self) -> Option<String> {
        Some(String::from(self.0))
    }
    fn phone(&self) -> Option<String> {
        Some(String::from(self.1))
    }
    fn email(&self) -> Option<String> {
        Some(String::from(self.2))
    }
    fn github(&self) -> Option<String> {
        Some(String::from(self.3))
    }
    fn other(&self) -> Option<(String, String)> {
        None
    }
}

#[derive(Clone, Copy, Default)]
pub struct Type1InfoTimeSituation(Type1InfoTime, Type1InfoSituation);

#[derive(Clone, Copy, Default)]
struct Type1InfoTime;
impl Time for Type1InfoTime {}

#[derive(Clone, Copy, Default)]
struct Type1InfoSituation;
impl Situation for Type1InfoSituation {}

impl Inner for Type1InfoTimeSituation {
    fn time(&self) -> Box<dyn Time> {
        Box::new(self.0)
    }
    fn situation(&self) -> Box<dyn Situation> {
        Box::new(self.1)
    }
}

pub struct Type1Undergraduate<'a> {
    school: &'a str,
    major: &'a str,
    time_situation: Type1UndergraduateTimeSituation,
}

impl<'a> IntoInner for Type1Undergraduate<'a> {
    fn to_inner(&self) -> Box<dyn Inner> {
        Box::new(self.time_situation.clone())
    }
}

impl<'a> EduInner for Type1Undergraduate<'a> {
    fn experience(&self) -> Degree {
        Degree::Undergraduate(String::from(self.school), String::from(self.major))
    }
}

#[derive(Clone)]
pub struct Type1UndergraduateTimeSituation(Type1UndergraduateTime, Type1UndergraduateSituation);

impl Inner for Type1UndergraduateTimeSituation {
    fn time(&self) -> Box<dyn Time> {
        Box::new(self.0)
    }
    fn situation(&self) -> Box<dyn Situation> {
        Box::new(self.1.clone())
    }
}
#[derive(Clone, Copy)]
pub struct Type1UndergraduateTime {
    year: (u32, u32),
    month: (u8, u8),
}
impl Time for Type1UndergraduateTime {
    fn year(&self) -> Option<(u32, u32)> {
        Some(self.year)
    }
    fn month(&self) -> Option<(u8, u8)> {
        Some(self.month)
    }
}
#[derive(Clone)]
pub struct Type1UndergraduateSituation {
    province: String,
    city: String,
}

impl Situation for Type1UndergraduateSituation {
    fn province(&self) -> Option<String> {
        Some(self.province.clone())
    }
    fn city(&self) -> Option<String> {
        Some(self.city.clone())
    }
}

pub struct Type1Internship<'a> {
    company: &'a str,
    position: &'a str,
    content: Vec<&'a str>,
    time_situation: Type1WorkTimeSituation,
}

impl<'a> IntoInner for Type1Internship<'a> {
    fn to_inner(&self) -> Box<dyn Inner> {
        Box::new(self.time_situation.clone())
    }
}

impl<'a> WorkInner for Type1Internship<'a> {
    fn class(&self) -> WorkClass {
        WorkClass::Internship
    }
    fn company(&self) -> String {
        String::from(self.company)
    }
    fn position(&self) -> String {
        String::from(self.position)
    }
    fn content(&self) -> Vec<String> {
        self.content.iter().map(|c| String::from(*c)).collect()
    }
}

pub struct Type1FullTimeWork<'a> {
    company: &'a str,
    position: &'a str,
    content: Vec<&'a str>,
    time_situation: Type1WorkTimeSituation,
}

impl<'a> IntoInner for Type1FullTimeWork<'a> {
    fn to_inner(&self) -> Box<dyn Inner> {
        Box::new(self.time_situation.clone())
    }
}

impl<'a> WorkInner for Type1FullTimeWork<'a> {
    fn class(&self) -> WorkClass {
        WorkClass::FullTime
    }
    fn company(&self) -> String {
        String::from(self.company)
    }
    fn position(&self) -> String {
        String::from(self.position)
    }
    fn content(&self) -> Vec<String> {
        self.content.iter().map(|c| String::from(*c)).collect()
    }
}

#[derive(Clone)]
pub struct Type1WorkTimeSituation(Type1WorkTime, Type1WorkSituation);
impl Inner for Type1WorkTimeSituation {
    fn time(&self) -> Box<dyn Time> {
        Box::new(self.0)
    }
    fn situation(&self) -> Box<dyn Situation> {
        Box::new(self.1.clone())
    }
}

#[derive(Clone, Copy)]
pub struct Type1WorkTime {
    year: (u32, u32),
    month: (u8, u8),
}
impl Time for Type1WorkTime {
    fn year(&self) -> Option<(u32, u32)> {
        Some(self.year)
    }
    fn month(&self) -> Option<(u8, u8)> {
        Some(self.month)
    }
}

#[derive(Clone)]
pub struct Type1WorkSituation {
    province: String,
    city: String,
}
impl Situation for Type1WorkSituation {
    fn province(&self) -> Option<String> {
        Some(self.province.clone())
    }
    fn city(&self) -> Option<String> {
        Some(self.city.clone())
    }
}

pub struct Type1ProjectInner<'a> {
    proj_name: &'a str,
    group: &'a str,
    lang: Option<&'a str>,
    time_situation: Type1ProjTimeSituation,
}

impl<'a> IntoInner for Type1ProjectInner<'a> {
    fn to_inner(&self) -> Box<dyn Inner> {
        Box::new(self.time_situation)
    }
}

impl<'a> ProjInner for Type1ProjectInner<'a> {
    fn project(&self) -> String {
        String::from(self.proj_name)
    }
    fn group(&self) -> String {
        String::from(self.group)
    }
    fn lang(&self) -> Option<String> {
        if let Some(l) = self.lang {
            Some(String::from(l))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Type1ProjTimeSituation(Type1ProjTime, Type1ProjSituation);

impl Inner for Type1ProjTimeSituation {
    fn time(&self) -> Box<dyn Time> {
        Box::new(self.0)
    }
    fn situation(&self) -> Box<dyn Situation> {
        Box::new(self.1)
    }
}

#[derive(Clone, Copy)]
pub struct Type1ProjTime {
    year: (u32, u32),
    month: (u8, u8),
}
impl Time for Type1ProjTime {
    fn year(&self) -> Option<(u32, u32)> {
        Some(self.year)
    }
    fn month(&self) -> Option<(u8, u8)> {
        Some(self.month)
    }
}

#[derive(Clone, Copy, Default)]
pub struct Type1ProjSituation {}
impl Situation for Type1ProjSituation {
    fn province(&self) -> Option<String> {
        None
    }
    fn city(&self) -> Option<String> {
        None
    }
}

pub struct Type1HonorInner<'a> {
    honor: &'a str,
    description: &'a str,
    time: (u32, u8), // (year, month)
    time_situation: Type1HonorTimeSituation,
}

impl<'a> IntoInner for Type1HonorInner<'a> {
    fn to_inner(&self) -> Box<dyn Inner> {
        Box::new(self.time_situation)
    }
}

impl<'a> HonorInner for Type1HonorInner<'a> {
    fn honor(&self) -> String {
        String::from(self.honor)
    }
    fn description(&self) -> String {
        String::from(self.description)
    }
    fn time(&self) -> (u32, u8) {
        self.time
    }
}

#[derive(Clone, Copy, Default)]
pub struct Type1HonorTimeSituation(Type1HonorTime, Type1HonorSituation);

impl Inner for Type1HonorTimeSituation {
    fn time(&self) -> Box<dyn Time> {
        Box::new(self.0)
    }
    fn situation(&self) -> Box<dyn Situation> {
        Box::new(self.1)
    }
}

#[derive(Clone, Copy, Default)]
pub struct Type1HonorTime {}
impl Time for Type1HonorTime {}

#[derive(Clone, Copy, Default)]
pub struct Type1HonorSituation {}
impl Situation for Type1HonorSituation {}
