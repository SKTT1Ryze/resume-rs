//! Some Implementations of Resume Template  
//! 
//! 

pub mod type1;

use crate::Resume;

pub trait Template {
    fn typography(&self) -> Box<dyn Typography>;
    fn resume(&self) -> &Resume;
}

pub trait Typography {
    fn oddsidemargin(&self) -> Option<i32> {
        None
    }
    fn evensidemargin(&self) -> Option<i32> {
        None
    }
    fn textwidth(&self) -> Option<i32> {
        None
    }
    fn topmargin(&self) -> Option<i32> {
        None
    }
    fn textheight(&self) -> Option<i32> {
        None
    }
    fn other(&self) -> Option<String> {
        None
    }
}

