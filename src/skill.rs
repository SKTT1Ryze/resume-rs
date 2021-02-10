//! Skill Implementation of `ResumeElement` Trait
//! Mainly include the lang, tools
//!
//! Example:  
//! ```Rust
//! ```
//!

use crate::{Inner, IntoInner, ResumeElement};

#[derive(Default)]
pub struct Skill {
    inners: Vec<Box<dyn SkillInner>>,
}

impl Skill {
    pub fn append_inner<I>(&mut self, inner: I)
    where
        I: SkillInner + 'static,
    {
        self.inners.push(Box::new(inner));
    }
}

impl ResumeElement for Skill {
    fn title(&self) -> Option<String> {
        Some(String::from("SKILL"))
    }

    fn inner(&self) -> Vec<Box<dyn Inner>> {
        let mut inner = Vec::new();
        for skill_inner in &self.inners {
            inner.push(skill_inner.to_inner());
        }
        inner
    }

    fn skill_inner(&self) -> Option<Vec<Box<&dyn SkillInner>>> {
        let skill_inners: Vec<Box<&dyn SkillInner>> =
            self.inners.iter().map(|e| Box::new(e.as_ref())).collect();
        Some(skill_inners)
    }
}

pub trait SkillInner: IntoInner {
    fn items(&self) -> (String, Vec<String>);
}
