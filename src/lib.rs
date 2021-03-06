//! A Crate for Generating Latex Resume Documents Programatically.
//!
//! This library tries to create simple and useful interface to
//! help you writting your resume with `rustlang`.  
//!
//! Imagine that it' s such a cool thing that writting your resume by programming!  
//!

use education::EduInner;
use honor::HonorInner;
use info::InfoInner;
use proj::ProjInner;
use skill::SkillInner;
use work::WorkInner;

pub mod education;
pub mod honor;
pub mod info;
pub mod proj;
pub mod render;
pub mod skill;
pub mod template;
pub mod work;

/// The resume, included `ResumeClass`
/// and some object implemented `ResumeElement` trait
pub struct Resume {
    _class: ResumeClass,
    pub elements: Vec<Box<dyn ResumeElement>>,
}

impl Default for Resume {
    fn default() -> Self {
        Self {
            _class: ResumeClass::Programmer,
            elements: Vec::new(),
        }
    }
}

impl Resume {
    fn append_element<E>(&mut self, elem: E)
    where
        E: ResumeElement + 'static,
    {
        self.elements.push(Box::new(elem));
    }
}
/// Classification of Resume
pub enum ResumeClass {
    Programmer,
    Other,
}

pub trait ResumeElement {
    fn title(&self) -> Option<String>;
    fn inner(&self) -> Vec<Box<dyn Inner>>;
    fn info_inner(&self) -> Option<Box<&dyn InfoInner>> {
        None
    }
    fn edu_inner(&self) -> Option<Vec<Box<&dyn EduInner>>> {
        None
    }
    fn work_inner(&self) -> Option<Vec<Box<&dyn WorkInner>>> {
        None
    }
    fn proj_inner(&self) -> Option<Vec<Box<&dyn ProjInner>>> {
        None
    }
    fn honor_inner(&self) -> Option<Vec<Box<&dyn HonorInner>>> {
        None
    }
    fn skill_inner(&self) -> Option<Vec<Box<&dyn SkillInner>>> {
        None
    }
}

pub trait Inner {
    fn time(&self) -> Box<dyn Time>;
    fn situation(&self) -> Box<dyn Situation>;
}

pub trait Time {
    fn year(&self) -> Option<(u32, u32)> {
        None
    }
    fn month(&self) -> Option<(u8, u8)> {
        None
    }
    fn day(&self) -> Option<(u32, u32)> {
        None
    }
}

pub trait Situation {
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

pub trait IntoInner {
    fn to_inner(&self) -> Box<dyn Inner>;
}
