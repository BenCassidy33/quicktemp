use crate::{template::TemplateParser, templates::include::IncludeTemplate};

pub mod cli;
pub mod options;
pub mod template;
pub mod templates;

fn main() {
    let mut parser = TemplateParser::new();
    parser.register(IncludeTemplate::new());

    let template = parser
        .parse("include ./src/main.rs ./src/template.rs, allow_subdir = true, pretty = true")
        .unwrap();

    dbg!(template);
}
