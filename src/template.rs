use std::fmt::Debug;

pub trait Template: Debug {
    fn descriminator(&self) -> &str;
    fn parse(&self, s: &str) -> anyhow::Result<Box<dyn Template>>;
    fn exec(&self) -> anyhow::Result<String>;
}

pub struct TemplateParser {
    registered_templates: Vec<Box<dyn Template>>,
}

impl TemplateParser {
    pub fn new() -> Self {
        Self {
            registered_templates: Vec::new(),
        }
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
