use std::{fmt::Debug, ops::Range, path::PathBuf};

use crate::{
    file::TemplateFile,
    templates::{exec::ExecTemplate, include::IncludeTemplate},
};

pub trait Template: Debug {
    fn descriminator(&self) -> &str;
    fn parse(&self, s: &str, info: TemplateInfo) -> anyhow::Result<Box<dyn Template>>;
    fn exec(&self) -> anyhow::Result<String>;
    fn info(&self) -> &TemplateInfo;
}

pub struct TemplateParser {
    registered_templates: Vec<Box<dyn Template>>,
    files: Vec<TemplateFile>,
}

impl Default for TemplateParser {
    fn default() -> Self {
        Self::default_templates()
    }
}

impl TemplateParser {
    pub fn new(files: Vec<TemplateFile>) -> anyhow::Result<Self> {
        Ok(Self {
            registered_templates: Vec::new(),
            files,
        })
    }

    pub fn parse_files(&self) -> anyhow::Result<Vec<String>> {
        todo!()
    }

    pub fn default_templates() -> Self {
        let mut parser = Self::new(Vec::new()).expect("Failed to parse default templates!");

        parser.register(IncludeTemplate::new());
        parser.register(ExecTemplate::new());

        parser
    }

    pub fn add_file(&mut self, file: TemplateFile) {
        self.files.push(file);
    }

    pub fn register<T: Template + 'static>(&mut self, template: T) {
        self.registered_templates.push(Box::new(template));
    }

    pub fn parse(&self, s: &str, info: TemplateInfo) -> anyhow::Result<Box<dyn Template>> {
        for temp in &self.registered_templates {
            // TODO: Remove this clone
            if s.starts_with(temp.descriminator())
                && let Ok(t) = temp.parse(s, info.clone())
            {
                return Ok(t);
            }
        }

        Err(anyhow::anyhow!("Not a valid template: {s}"))
    }
}

#[derive(Debug, Default, Clone)]
pub struct TemplateInfo {
    file: PathBuf,
    line: usize,
    col_start: usize,
    col_end: usize,
}

impl TemplateInfo {
    pub fn new(file: PathBuf, line: usize, col_start: usize, col_end: usize) -> Self {
        Self {
            file,
            line,
            col_start,
            col_end,
        }
    }

    pub fn get_info_from_file(template: &str, file: &str, file_path: &PathBuf) -> TemplateInfo {
        let pos = file
            .find(template)
            .expect("Could not get template position info from file content!");
        let before = &file[..pos];
        let nl_count = &file[..pos].matches("\n").count();
        let rnl = before.rfind("\n").unwrap_or(0);

        let ldelim_pos = before
            .rfind("{{")
            .expect("Failed to get `{{` for template!");
        let ldelim_pos = pos - rnl - (pos - ldelim_pos);

        let rdelim_pos = &file[pos..]
            .find("}}")
            .expect("Failed to get `}}` for template!");
        let rdelim_pos = pos + rdelim_pos + "}}".len();

        // TODO: Pretty sure pos - rnl is wrong
        let t = TemplateInfo::new(file_path.clone(), *nl_count, ldelim_pos, rdelim_pos);
        t.get_raw_from_file(file);
        t
    }

    pub fn get_raw_from_file(&self, file: &str) -> String {
        file.lines()
            .nth(self.line)
            .expect("Failed to get raw template from info!")
            .to_string()
    }
}
