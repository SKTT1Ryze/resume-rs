//! Example for `resume-rs` Crate
//! 

extern crate lazy_static;
extern crate resume;
extern crate latex;

use lazy_static::lazy_static;
use resume::render::Type1Render;

lazy_static! {
    static ref NAME: String = String::from("XXX");
    static ref PHONE: String = String::from("+86 1234-5678-910");
    static ref EMAIL: String = String::from("1234567@89.com");
    static ref GITHUB: String = String::from("https://github.com/XXX");
    static ref EDUCATION: Vec<(
        String, String, (u32, u32), (u8, u8), (String, String)
    )> = vec![
        (String::from("Kassel College"), String::from("Dragon Slayer"), (2015, 2018), (9, 7), (String::from("Hubei"), String::from("Wuhan")))
    ];
    static ref WORK: Vec<(
        String, String, Vec<String>, (u32, u32), (u8, u8), (String, String)
    )> = vec![
        (
            String::from("RIOT"),
            String::from("Game Developer"),
            vec![
                String::from("Developing LOL in RIOT"),
                String::from("Game Server Testing"),
                ],
            (2020, 2025),
            (1, 1),
            (String::from("California"), String::from("West Los Angeles"))
        ),
        (
            String::from("Google"),
            String::from("Fushia Developer"),
            vec![
            String::from("Developing Fuchsia OS in Google"),
            String::from("Develop Fuchsia OS with Rust"),
            ],
            (2026, 2030),
            (2, 2),
            (String::from("China"), String::from("Beijing")))
    ];
}

fn main() {
    let latex_doc = Type1Render::render(
        (-1, -1, 2, -1, 2),
        &(*NAME),
        &(*PHONE),
        &(*EMAIL),
        &(*GITHUB),
        &(*EDUCATION),
        &(*WORK)
    );
    println!("{}", latex::print(&latex_doc).unwrap());
}
