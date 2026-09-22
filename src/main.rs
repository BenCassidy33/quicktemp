use crate::{file::TemplateFile, template::TemplateParser, templates::include::IncludeTemplate};

pub mod cli;
pub mod file;
pub mod options;
pub mod template;
pub mod templates;

fn main() {
    let mut parser = TemplateParser::new();
    parser.register(IncludeTemplate::new());

    let mut file = TemplateFile::new("./test.html").unwrap();
    let templates = file.parse_templates(&parser);
}
