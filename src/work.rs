//! Work experience implementation of `ResumeElement` trait  
//! Maily include company, position, simple introduction of job content, etc.  
//! 
//! Example:  
//! ```Rust
//! ```
//! 

use crate::{
    Inner,
    IntoInner,
    ResumeElement
};
#[derive(Default)]
pub struct Work {
    inners: Vec<Box<dyn WorkInner>>
}

impl Work {
    fn append_inner<I: WorkInner + 'static>(&mut self, inner: I) {
        self.inners.push(Box::new(inner));
    }
}

impl ResumeElement for Work {
    fn title(&self) -> Option<String> {
        Some(String::from("Work Experience"))
    }

    fn inner(&self) -> Vec<Box<dyn Inner>> {
        let mut inner = Vec::new();
        for work_inner in &self.inners {
            inner.push(work_inner.to_inner());
        }
        inner
    }
}

pub trait WorkInner: IntoInner {
    fn company(&self) -> String;
    fn position(&self) -> String;
    fn content(&self) -> Option<String>;
}