//! Type1 Implementation of `Template` trait
//! 
//! Example:  
//! ```Rust
//! ```
//! 

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
    where S: AsRef<str> + 'static
    {
        let type1_info_inner = Type1InfoInner (
            name.as_ref(),
            phone.as_ref(),
            email.as_ref(),
            github.as_ref(),
            Type1TimeSituation::default()
        );
        let type1_info = PersonalInfo::new(type1_info_inner);
        self.resume.append_element(type1_info);
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
    Type1TimeSituation,
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
pub struct Type1TimeSituation (
    Type1Time,
    Type1Situation
);

#[derive(Clone, Copy, Default)]
struct Type1Time;
impl Time for Type1Time {
    fn year(&self) -> Option<u32> {
        None
    }
    fn month(&self) -> Option<String> {
        None
    }
    fn day(&self) -> Option<u32> {
        None
    }
}
#[derive(Clone, Copy, Default)]
struct Type1Situation;
impl Situation for Type1Situation {
    fn galaxy(&self) -> Option<String> {
        None
    }
    fn planet(&self) -> Option<String> {
        None
    }
    fn country(&self) -> Option<String> {
        None
    }
    fn province(&self) -> Option<String> {
        None
    }
    fn city(&self) -> Option<String> {
        None
    }
    fn other(&self) -> Option<String> {
        None
    }
}

impl Inner for Type1TimeSituation {
    fn time(&self) -> Box<dyn Time> {
        Box::new(self.0)
    }
    fn situation(&self) -> Box<dyn Situation> {
        Box::new(self.1)
    }
}
