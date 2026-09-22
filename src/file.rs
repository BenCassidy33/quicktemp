use crate::template::{Template, TemplateParser};
use regex::Regex;
use std::{path::PathBuf, sync::LazyLock};

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = r"\{\{(.*?)\}\}";
    Regex::new(pattern).expect("invalid regex pattern")
});

pub struct TemplateFile {
    path: PathBuf,
    content: String,
    raw_templates: Vec<String>,
    templates: Vec<Box<dyn Template>>,
}

// TODO: Make this lazy so the entire file is not read at runtime...
impl TemplateFile {
    pub fn new<P: Into<PathBuf>>(path: P) -> anyhow::Result<Self> {
        let path = path.into();
        let content = std::fs::read_to_string(&path)?;
        let captures = REGEX.captures_iter(&content);
        let mut raw_templates = Vec::new();

        for cap in captures {
            let cap_str = cap.get_match().as_str();
            if !cap_str.starts_with("{{") || !cap_str.ends_with("}}") {
                return Err(anyhow::anyhow!(
                    "Templates must start with `{{` and end with `}}`."
                ));
            }

            raw_templates.push(cap_str[2..cap_str.len() - 3].trim().to_string());
        }

        Ok(Self {
            path,
            content,
            raw_templates,
            templates: Vec::new(),
        })
    }

    pub fn parse_templates(
        &mut self,
        parser: &TemplateParser,
    ) -> anyhow::Result<&Vec<Box<dyn Template>>> {
        for template in &self.raw_templates {
            let temp = parser.parse(template)?;
            self.templates.push(temp);
        }

        Ok(&self.templates)
    }

    pub fn raw_templates(&self) -> &Vec<String> {
        &self.raw_templates
    }

    pub fn templates(&self) -> &Vec<Box<dyn Template>> {
        &self.templates
    }
}
