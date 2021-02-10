//! Project Implementation of `ResumeElement` Trait
//! Mainly include what project you took part in and which group you
//! work in  
//!
//! Example:  
//! ```Rust
//! ```
//!

use crate::{Inner, IntoInner, ResumeElement};

#[derive(Default)]
pub struct Project {
    inners: Vec<Box<dyn ProjInner>>,
}

impl Project {
    pub fn append_inner<I>(&mut self, inner: I)
    where
        I: ProjInner + 'static,
    {
        self.inners.push(Box::new(inner));
    }
}

impl ResumeElement for Project {
    fn title(&self) -> Option<String> {
        Some(String::from("PROJECTS AND RESEARCH"))
    }

    fn inner(&self) -> Vec<Box<dyn Inner>> {
        let mut inner = Vec::new();
        for proj_inner in &self.inners {
            inner.push(proj_inner.to_inner());
        }
        inner
    }
    fn proj_inner(&self) -> Option<Vec<Box<&dyn ProjInner>>> {
        let proj_inners: Vec<Box<&dyn ProjInner>> =
            self.inners.iter().map(|e| Box::new(e.as_ref())).collect();
        Some(proj_inners)
    }
}

pub trait ProjInner: IntoInner {
    fn project(&self) -> String;
    fn group(&self) -> String;
    fn lang(&self) -> Option<String>;
}
