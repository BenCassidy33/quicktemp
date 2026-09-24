#![allow(warnings)]

use std::sync::LazyLock;

use regex::Regex;

use crate::{
    file::TemplateFile,
    template::{TemplateInfo, TemplateParser},
    templates::{exec::ExecTemplate, include::IncludeTemplate},
};

pub mod cli;
pub mod file;
pub mod options;
pub mod preprocessor;
pub mod template;
pub mod templates;

#[macro_export]
macro_rules! regex {
    ($name:ident, $($arg:tt)*) => {
        static $name: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            let pattern = format!($($arg)*);
            regex::Regex::new(&pattern)
                .unwrap_or_else(|e| panic!("invalid regex pattern: {} ({})", pattern, e))
        });
    };
}

pub const OPTIONS_DELIMITER: &str = "::";

fn main() {
    let parser = TemplateParser::default();

    let mut file = TemplateFile::new("./test.html").unwrap();
    file.parse_templates(&parser).unwrap();

    dbg!(file.templates());

    for template in file.templates() {
        dbg!(file.get_raw(&template));
    }
}
