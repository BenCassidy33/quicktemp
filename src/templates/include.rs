use crate::{OPTIONS_DELIMITER, options::TemplateOptions, template::Template};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{any, path::PathBuf, range::Range, sync::LazyLock};

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = format!(r"^\s*(include)\s+([^,]+?)\s*(?:{OPTIONS_DELIMITER}\s*(.+))?\s*$");
    Regex::new(&pattern).expect("invalid regex pattern")
});

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IncludeOptions {
    pretty: bool,
    allow_subdir: bool,
}

impl TemplateOptions for IncludeOptions {}

#[derive(Debug, Clone)]
pub struct IncludeTemplate {
    paths: Vec<PathBuf>,
    opts: IncludeOptions,
}

impl IncludeTemplate {
    pub fn new() -> Self {
        IncludeTemplate {
            paths: Vec::new(),
            opts: IncludeOptions::default(),
        }
    }
}

impl Template for IncludeTemplate {
    fn descriminator(&self) -> &str {
        return "include";
    }

    fn parse(&self, s: &str) -> anyhow::Result<Box<dyn Template>> {
        let captures = REGEX
            .captures(s)
            .ok_or(anyhow::anyhow!("Invalid include template structure"))?;

        let mut captures = captures.iter();

        captures.next();
        let _desc = captures.next();
        let paths = captures
            .next()
            .ok_or(anyhow::anyhow!("Could not get paths for include statment!"))?
            .unwrap();

        let paths = paths.as_str().split(" ").collect::<Vec<&str>>();
        let paths = paths.iter().map(PathBuf::from);

        let opts = if let Some(opts) = captures.next() {
            let opts = opts.unwrap().as_str();
            IncludeOptions::parse(opts)?
        } else {
            IncludeOptions::default()
        };

        Ok(Box::new(Self {
            paths: paths.collect(),
            opts,
        }))
    }

    fn exec(&self) -> anyhow::Result<String> {
        let mut s = String::new();

        for path in &self.paths {
            if !path.exists() {
                return Err(anyhow::anyhow!(
                    "Path {} does not exist!",
                    path.to_string_lossy()
                ));
            }

            let file = std::fs::read_to_string(path)?;
            s.push_str(&file);
        }

        Ok(s)
    }
}
