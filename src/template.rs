use std::{fmt::Debug, ops::Range};

use crate::templates::{exec::ExecTemplate, include::IncludeTemplate};

pub trait Template: Debug {
    // fn range(&self) -> Range<usize>;
    fn descriminator(&self) -> &str;
    fn parse(&self, s: &str) -> anyhow::Result<Box<dyn Template>>;
    fn exec(&self) -> anyhow::Result<String>;
}

pub struct TemplateParser {
    registered_templates: Vec<Box<dyn Template>>,
}

impl Default for TemplateParser {
    fn default() -> Self {
        Self::default_templates()
    }
}

impl TemplateParser {
    pub fn new() -> Self {
        Self {
            registered_templates: Vec::new(),
        }
    }

    pub fn default_templates() -> Self {
        let mut parser = Self::new();

        parser.register(IncludeTemplate::new());
        parser.register(ExecTemplate::new());

        parser
    }

    pub fn register<T: Template + 'static>(&mut self, template: T) {
        self.registered_templates.push(Box::new(template));
    }

    pub fn parse(&self, s: &str) -> anyhow::Result<Box<dyn Template>> {
        for temp in &self.registered_templates {
            if s.starts_with(temp.descriminator())
                && let Ok(t) = temp.parse(s)
            {
                return Ok(t);
            }
        }

        Err(anyhow::anyhow!("Not a valid template: {s}"))
    }
}
