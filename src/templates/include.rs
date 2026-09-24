use crate::{OPTIONS_DELIMITER, Template, info::TemplateInfo, options::TemplateOptions};

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{any, path::PathBuf, range::Range, sync::LazyLock};

crate::regex!(
    REGEX,
    r"^\s*(include)\s+([^,]+?)\s*(?:{OPTIONS_DELIMITER}\s*(.+))?\s*$"
);

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
    info: TemplateInfo,
}

impl IncludeTemplate {
    pub fn new() -> Self {
        IncludeTemplate {
            paths: Vec::new(),
            opts: IncludeOptions::default(),
            info: TemplateInfo::default(),
        }
    }
}

impl Template for IncludeTemplate {
    fn info(&self) -> &TemplateInfo {
        &self.info
    }

    fn descriminator(&self) -> &str {
        return "include";
    }

    fn parse(&self, s: &str, info: TemplateInfo) -> anyhow::Result<Option<Box<dyn Template>>> {
        let Some(captures) = REGEX.captures(s) else {
            return Ok(None);
        };

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

        Ok(Some(Box::new(Self {
            paths: paths.collect(),
            opts,
            info,
        })))
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
