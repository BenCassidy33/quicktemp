use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
};

use crate::template::{Template, TemplateParser};
use regex::Regex;

const REGEX: &str = r"\{\{(.*?)\}\}";

pub struct TemplateFile {
    path: PathBuf,
    content: String,
    raw_templates: Vec<String>,
    templates: Vec<Box<dyn Template>>,
}

// TODO: Make this lazy so the entire file is not read at runtime...
impl TemplateFile {
    pub fn new<P: Into<PathBuf>>(path: P) -> anyhow::Result<Self> {
        let regex = Regex::new(REGEX).expect("Failed to parse regex");

        let path = path.into();
        let content = std::fs::read_to_string(&path)?;
        let captures = regex.captures(&content);

        if captures.is_none() {
            return Ok(Self {
                path,
                content,
                raw_templates: Vec::new(),
                templates: Vec::new(),
            });
        }

        let mut raw_templates = Vec::new();

        for (idx, cap) in captures.unwrap().iter().enumerate() {
            if idx % 2 == 0 {
                continue;
            }

            raw_templates.push(cap.unwrap().as_str().trim().to_string());
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
