use crate::{
    file::TemplateFile,
    template::TemplateParser,
    templates::{exec::ExecTemplate, include::IncludeTemplate},
};

pub mod cli;
pub mod file;
pub mod options;
pub mod template;
pub mod templates;

pub const OPTIONS_DELIMITER: &str = "::";

fn main() {
    let parser = TemplateParser::default();

    let mut file = TemplateFile::new("./test.html").unwrap();
    let templates = file.parse_templates(&parser);

    dbg!(templates);
}
