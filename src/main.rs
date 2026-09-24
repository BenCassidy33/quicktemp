#![allow(warnings)]

use quicktemp::{items::file::TemplateFile, parser::TemplateParser};

fn main() -> anyhow::Result<()> {
    let parser = TemplateParser::default();
    let mut file = TemplateFile::new("./test.html").unwrap();
    parser.parse(&mut file)?;
    dbg!(file.templates());

    Ok(())
}
