//! Education Experience Implementation of `ResumeElement` Trait  
//! Maily include undergraduate, master, doctor, etc.  
//! 
//! Example:  
//! ```Rust
//! ```
//! 

use crate::{Inner, IntoInner, ResumeElement};

#[derive(Default)]
pub struct Education {
    inners: Vec<Box<dyn EduInner>>
}

impl Education {
    pub fn append_inner<I: EduInner + 'static>(&mut self, inner: I) {
        self.inners.push(Box::new(inner));
    }
}

impl ResumeElement for Education {
    fn title(&self) -> Option<String> {
        Some(String::from("Education"))
    }
    fn inner(&self) -> Vec<Box<dyn Inner>> {
        let mut inner = Vec::new();
        for edu_inner in &self.inners {
            inner.push(edu_inner.to_inner())
        }
        inner
    }
    fn edu_inner(&self) -> Option<Vec<Box<&dyn EduInner>>> {
        let edu_inners: Vec<Box<&dyn EduInner>> = self.inners.iter()
            .map(|e| Box::new(e.as_ref())).collect();
        Some(edu_inners)
    }
}

pub trait EduInner: IntoInner {
    fn experience(&self) -> Degree;
}

pub enum Degree {
    // (School, Major)
    Undergraduate(String, String),
    Master(String, String),
    Doctor(String, String),
    Other(String)
}