//! Type1 Implementation of `Template` trait
//! 
//! Example:  
//! ```Rust
//! ```
//! 

use education::{Degree, EduInner, Education};
use info::{InfoInner, PersonalInfo};

use super::{Template, Typography};
use crate::*;

pub struct TemplateType1 {
    typography: Type1Typography,
    resume: Resume
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
            resume: Resume::default()
        }
    }

    pub fn personal_info<S>(
        &mut self,
        name: &'static S,
        phone: &'static S,
        email: &'static S,
        github: &'static S
    )

    where S: AsRef<str>
    {
        let type1_info_inner = Type1InfoInner (
            name.as_ref(),
            phone.as_ref(),
            email.as_ref(),
            github.as_ref(),
            Type1InfoTimeSituation::default()
        );
        let type1_info = PersonalInfo::new(type1_info_inner);
        self.resume.append_element(type1_info);
    }

    pub fn undergraduate<S>(
        &mut self,
        school: &'static S,
        major: &'static S,
        year: (u32, u32),
        month: (u8, u8),
        situation: (&'static S, &'static S)
    )
    where S: AsRef<str>
    {
        let type1_edu_inner = Type1Undergraduate {
            school: school.as_ref(),
            major: major.as_ref(),
            time_situation: Type1UndergraduateTimeSituation (
                Type1UndergraduateTime {
                    year,
                    month
                },
                Type1UndergraduateSituation {
                    province: String::from(situation.0.as_ref()),
                    city: String::from(situation.1.as_ref())
                }
            )
        };
        let mut type1_degree = Education::default();
        type1_degree.append_inner(type1_edu_inner);
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

pub struct Type1InfoInner<'a> (
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    Type1InfoTimeSituation,
);

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
pub struct Type1InfoTimeSituation (
    Type1InfoTime,
    Type1InfoSituation
);

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
    time_situation: Type1UndergraduateTimeSituation
}

impl<'a> IntoInner for Type1Undergraduate<'a> {
    fn to_inner(&self) -> Box<dyn Inner> {
        Box::new(self.time_situation.clone())
    }
}

impl<'a> EduInner for Type1Undergraduate<'a> {
    fn experience(&self) -> Vec<Degree> {
        vec![Degree::Undergraduate(String::from(self.school), String::from(self.major))]
    }
}

#[derive(Clone)]
pub struct Type1UndergraduateTimeSituation (
    Type1UndergraduateTime,
    Type1UndergraduateSituation
);

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
    month: (u8, u8)
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
    city: String
}

impl Situation for Type1UndergraduateSituation {
    fn province(&self) -> Option<String> {
        Some(self.province.clone())
    }
    fn city(&self) -> Option<String> {
        Some(self.city.clone())
    }
}