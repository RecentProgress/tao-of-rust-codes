//! 《Rust编程之道》
//!
//!  这里记录本书中涉及的所有示例代码。

//! Rust安装的所有细节可以在 [附录A][appendix] 中找到。
//!
//! 正文从 [第一章: 新时代的语言][ch1] 开始。
//!
//! [appendix]: ../../doc/tao_of_rust/appendix/index.html
//! [ch1]: ../../doc/tao_of_rust/ch1/index.html

#![doc(html_playground_url = "https://play.rust-lang.org/",
       test(no_crate_inject, attr(deny(warnings))),
       test(attr(allow(dead_code, deprecated, unused_variables, unused_mut))))]

pub mod ch1;
pub mod ch2;
pub mod ch3;
pub mod ch4;
pub mod ch5;
pub mod ch6;
pub mod ch7;
pub mod ch8;
pub mod ch9;
pub mod ch10;
pub mod ch11;
pub mod ch12;
pub mod ch13;

pub mod appendix;