//! A crate for generating Latex Resume documents programatically.
//!
//! This library tries to create simple and useful interface to 
//! help you writting your resume with `rustlang`.  
//! 
//! Imagine that it' s such a cool thing that writting your resume by programming!  
//! 

pub mod info;
pub mod education;
pub mod work;

/// The resume, included `ResumeClass` 
/// and some object implemented `ResumeElement` trait
pub struct Resume {
    class: ResumeClass,
    elements: Vec<Box<dyn ResumeElement>>
}

impl Default for Resume {
    fn default() -> Self {
        Self {
            class: ResumeClass::Programmer,
            elements: Vec::new()
        }
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
}


pub trait Inner {
    fn time(&self) -> Box<dyn Time>;
    fn situation(&self) -> Box<dyn Situation>;
}

pub trait Time {
    fn year(&self) -> Option<u32>;
    fn month(&self) -> Option<String>;
    fn day(&self) -> Option<u32>;
}

pub trait Situation {
    fn galaxy(&self) -> Option<String>;
    fn planet(&self) -> Option<String>;
    fn country(&self) -> Option<String>;
    fn province(&self) -> Option<String>;
    fn city(&self) -> Option<String>;
}

pub trait IntoInner {
    fn to_inner(&self) -> Box<dyn Inner>;
}