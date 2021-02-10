//! Example for `resume-rs` Crate
//!

extern crate latex;
extern crate lazy_static;
extern crate resume;

use lazy_static::lazy_static;
use resume::render::Type1Render;

lazy_static! {
    static ref NAME: String = String::from("XXX");
    static ref PHONE: String = String::from("+86 1234-5678-910");
    static ref EMAIL: String = String::from("1234567@89.com");
    static ref GITHUB: String = String::from("https://github.com/XXX");
    static ref EDUCATION: Vec<(String, String, (u32, u32), (u8, u8), (String, String))> = vec![(
        String::from("Kassel College"),
        String::from("Dragon Slayer"),
        (2015, 2018),
        (9, 7),
        (String::from("Hubei"), String::from("Wuhan"))
    )];
    static ref WORK: Vec<(
        String,
        String,
        Vec<String>,
        (u32, u32),
        (u8, u8),
        (String, String)
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
            (String::from("China"), String::from("Beijing"))
        )
    ];
    static ref PROJECT: Vec<(String, String, Option<String>, (u32, u32), (u8, u8))> = vec![
        (
            String::from("RISC-V Superscalar Out-of-Order Processor"),
            String::from("Sifive"),
            Some(String::from("Rust \\& Chisel")),
            (2030, 2033),
            (7, 7)
        ),
        (
            String::from("Alpha Go Plus"),
            String::from("Google"),
            Some(String::from("Rust \\& C \\& C++")),
            (2034, 2036),
            (8, 8)
        ),
        (
            String::from("Async Kernel"),
            String::from("Home"),
            Some(String::from("Rust")),
            (2020, 2021),
            (12, 8)
        )
    ];
    static ref HONOR: Vec<(String, String, (u32, u8))> = vec![
        (
            String::from("S20 Champion"),
            String::from("Rank One in the 20th LOL World Champions"),
            (2030, 10)
        ),
        (
            String::from("XingYu Cup Champion"),
            String::from("The Champion of 5th XingYu Cup"),
            (2030, 10)
        )
    ];
    static ref SKILL: Vec<(String, Vec<String>)> = vec![
        (
            String::from("Languages"),
            vec![
                String::from("Chinese"),
                String::from("English"),
                String::from("Japanese")
            ]
        ),
        (
            String::from("Programming"),
            vec![String::from("Rust"), String::from("Chisel")]
        )
    ];
}

fn main() {
    let latex_doc = Type1Render::render(
        &(*NAME),
        &(*PHONE),
        &(*EMAIL),
        &(*GITHUB),
        &(*EDUCATION),
        &(*WORK),
        &(*PROJECT),
        &(*HONOR),
        &(*SKILL),
    );
    println!("{}", latex::print(&latex_doc).unwrap());
}
