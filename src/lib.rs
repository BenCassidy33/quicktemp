#![allow(warnings)]

use crate::parser::TemplateParser;

pub mod cli;
pub mod items;
pub mod info;
pub mod options;
pub mod parser;
pub mod preprocessor;
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

pub trait Template: std::fmt::Debug {
    fn descriminator(&self) -> &str;
    fn parse(&self, s: &str, info: info::TemplateInfo) -> anyhow::Result<Option<Box<dyn Template>>>;
    fn exec(&self) -> anyhow::Result<String>;
    fn info(&self) -> &info::TemplateInfo;
}

pub trait TemplateItem {
    fn parse_template(
        &mut self,
        parser: &TemplateParser,
        template: &Box<dyn Template>,
    ) -> anyhow::Result<()>;
}
