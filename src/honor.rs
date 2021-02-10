//! Honor Implementation of `ResumeElement` Trait
//! Mainly include what honors you have got and simple description of them
//!
//! Example:  
//! ```Rust
//! ```
//!

use crate::{Inner, IntoInner, ResumeElement};

#[derive(Default)]
pub struct Honor {
    inners: Vec<Box<dyn HonorInner>>,
}

impl Honor {
    pub fn append_inner<I>(&mut self, inner: I)
    where
        I: HonorInner + 'static,
    {
        self.inners.push(Box::new(inner));
    }
}

impl ResumeElement for Honor {
    fn title(&self) -> Option<String> {
        Some(String::from("HONORS AND AWARDS"))
    }

    fn inner(&self) -> Vec<Box<dyn Inner>> {
        let mut inner = Vec::new();
        for honor_inner in &self.inners {
            inner.push(honor_inner.to_inner());
        }
        inner
    }

    fn honor_inner(&self) -> Option<Vec<Box<&dyn HonorInner>>> {
        let honor_inners: Vec<Box<&dyn HonorInner>> =
            self.inners.iter().map(|e| Box::new(e.as_ref())).collect();
        Some(honor_inners)
    }
}

pub trait HonorInner: IntoInner {
    fn honor(&self) -> String;
    fn description(&self) -> String;
    // (year, month)
    fn time(&self) -> (u32, u8);
}
