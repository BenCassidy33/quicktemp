use crate::{Template, TemplateItem, info::TemplateInfo, parser::TemplateParser, preprocessor};
use regex::Regex;
use std::{path::PathBuf, sync::LazyLock};

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = r"\{\{(.*?)\}\}";
    Regex::new(pattern).expect("invalid regex pattern")
});

#[derive(Debug)]
pub struct TemplateFile {
    path: PathBuf,
    content: String,
    raw_templates: Vec<(String, TemplateInfo)>,
    templates: Vec<Box<dyn Template>>,
}

// TODO: Make this lazy so the entire file is not read at runtime...
impl TemplateFile {
    pub fn new<P: Into<PathBuf>>(path: P) -> anyhow::Result<Self> {
        let path = path.into();
        let content = preprocessor::preprocess(std::fs::read_to_string(&path)?);
        let captures = REGEX.captures_iter(&content);
        let mut raw_templates = Vec::new();

        for cap in captures {
            let cap_str = cap.get_match().as_str();
            if !cap_str.starts_with("{{") || !cap_str.ends_with("}}") {
                return Err(anyhow::anyhow!(
                    "Templates must start with `{{` and end with `}}`."
                ));
            }

            let raw_template = cap_str[2..cap_str.len() - 3].trim().to_string();
            let info = TemplateInfo::get_info_from_file(&raw_template, &content, &path);
            raw_templates.push((raw_template, info));
        }

        Ok(Self {
            path,
            content,
            raw_templates,
            templates: Vec::new(),
        })
    }

    pub fn raw_templates(&self) -> &Vec<(String, TemplateInfo)> {
        &self.raw_templates
    }

    pub fn templates(&self) -> &Vec<Box<dyn Template>> {
        &self.templates
    }

    pub fn get_raw(&self, template: &Box<dyn Template>) -> String {
        let info = template.info();
        info.get_raw_from_file(&self.content)
    }
}

impl TryInto<TemplateFile> for PathBuf {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<TemplateFile, Self::Error> {
        TemplateFile::new(self)
    }
}

impl TemplateItem for TemplateFile {
    fn parse_template(
        &mut self,
        parser: &TemplateParser,
        template: &Box<dyn Template>,
    ) -> anyhow::Result<()> {
        for (raw, info) in &self.raw_templates {
            if let Some(temp) = template.parse(raw, info.clone())? {
                self.templates.push(temp);
            }
        }

        Ok(())
    }
}
