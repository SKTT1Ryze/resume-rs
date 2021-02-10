# resume-rs
[English](./README_en.md) [中文](./README.md)  
Resume Template Written with Rust  

## What it is
It sounds cool that we write our resume by programming!  
Most of the time we can use some `Latex` templates to write beautiful resume and it's very fast.  
Sometimes we want to custom our templates, but don't want to learn `Latex` syntax, then this crate will help.  
You can generate your resume in `Latex` format by writting `Rust` code.  
If you want to render the `Latex` document as `pdf`, I recommend [overleaf](https://cn.overleaf.com/).  
However, there's a advance directive that I write this crate mainly for `Rust programming practice`.  
So, don't make it seriously.(hhh)  

## Example
Add some dependencies to your `Cargo.toml`  
```Toml
[dependencies]
resume = { git = "https://github.com/SKTT1Ryze/resume-rs" }
lazy_static = "1.4.0"
latex = { git = "https://github.com/SKTT1Ryze/latex-rs" }
```

Then see [main.rs](./example/src/main.rs).  

## License
[MIT License](./LICENSE)  

## Contributing
This repo always welcomes for your `Pull Request`.  
