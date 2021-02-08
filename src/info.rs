//! Personal information implementation of `ResumeElement` trait  
//! Maily included name, phone, email, etc.  
//!
//! These informations were mostly placed at the head of resume.  
//! 
//! Example:  
//! ```Rust
//! ```
//! 
//! 

use crate::{
    Inner,
    ResumeElement,
    IntoInner,
};
#[derive(Default)]
pub struct PersonalInfo {
    inners: Vec<Box<dyn InfoInner>>
}

impl PersonalInfo {
    fn append_inner<I: InfoInner + 'static>(&mut self, inner: I) {
        self.inners.push(Box::new(inner));
    }
}

impl ResumeElement for PersonalInfo {
    fn title(&self) -> Option<String> {
        None
    }
    fn inner(&self) -> Vec<Box<dyn Inner>> {
        let mut inner = Vec::new();
        for info_inner in &self.inners {
            inner.push(info_inner.to_inner());
        }
        inner
    }
}

pub trait InfoInner: IntoInner {
    fn name(&self) -> Option<String>;
    fn phone(&self) -> Option<String>;
    fn email(&self) -> Option<String>;
    fn github(&self) -> Option<String>;
    fn other(&self) -> Option<(String, String)>;
}


