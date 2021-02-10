# resume-rs
[English](./README.md) [中文](./README_ch.md)  
用 `Rust` 语言实现的简历模板。  

## 李再赣神魔
这个项目主要目的是 `Rust` 语言的编程练习，在写的过程中尝试使用一些陌生的语法，不断地给自己挖坑，还好最终能填上。  
写这个项目的动机是想要写简历投实习，然后突发奇想我能不能通过编程来生成简历的 `Latex` 模板，顺便用上最近学的一些 `Rust` 的高级语法。  
别笑！  

## Example
添加依赖到 `Cargo.toml`  
```Toml
[dependencies]
resume = { git = "https://github.com/SKTT1Ryze/resume-rs" }
lazy_static = "1.4.0"
latex = { git = "https://github.com/SKTT1Ryze/latex-rs" }
```

然后看[这里](./example/src/main.rs)  

## 开源协议
[MIT License](./LICENSE)  

## 贡献
随时欢迎通过 `Pull Request` 来贡献代码。  

 