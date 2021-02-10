//! Personal Information Implementation of `ResumeElement` Trait  
//! Maily included name, phone, email, etc.  
//!
//! These informations were mostly placed at the head of resume.  
//!
//! Example:  
//! ```Rust
//! ```
//!
//!

use crate::{Inner, IntoInner, ResumeElement};
pub struct PersonalInfo {
    pub inner: Box<dyn InfoInner>,
}

impl PersonalInfo {
    pub fn new<I: InfoInner + 'static>(inner: I) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl ResumeElement for PersonalInfo {
    fn title(&self) -> Option<String> {
        None
    }
    fn inner(&self) -> Vec<Box<dyn Inner>> {
        let mut inner = Vec::new();
        inner.push(self.inner.to_inner());
        inner
    }
    fn info_inner(&self) -> Option<Box<&dyn InfoInner>> {
        Some(Box::new(self.inner.as_ref()))
    }
}

pub trait InfoInner: IntoInner {
    fn name(&self) -> Option<String>;
    fn phone(&self) -> Option<String>;
    fn email(&self) -> Option<String>;
    fn github(&self) -> Option<String>;
    fn other(&self) -> Option<(String, String)>;
}
