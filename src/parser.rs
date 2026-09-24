use std::{ops::Range, path::PathBuf};

use crate::{
    Template, TemplateItem,
    info::TemplateInfo,
    items::file::TemplateFile,
    templates::{self, exec::ExecTemplate, include::IncludeTemplate},
};

pub struct TemplateParser {
    registered_templates: Vec<Box<dyn Template>>,
}

impl Default for TemplateParser {
    fn default() -> Self {
        Self::default_templates()
    }
}

impl TemplateParser {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            registered_templates: Vec::new(),
        })
    }

    pub fn default_templates() -> Self {
        let mut parser = Self::new().expect("Failed to parse default templates!");

        parser.register(IncludeTemplate::new());
        parser.register(ExecTemplate::new());

        parser
    }

    pub fn register<T: Template + 'static>(&mut self, template: T) {
        self.registered_templates.push(Box::new(template));
    }

    pub fn registered_templates(&self) -> &Vec<Box<dyn Template>> {
        &self.registered_templates
    }

    pub fn parse(&self, item: &mut dyn TemplateItem) -> anyhow::Result<()> {
        for template in &self.registered_templates {
            let parsed = item.parse_template(&self, &template)?;
        }

        Ok(())
    }
}
