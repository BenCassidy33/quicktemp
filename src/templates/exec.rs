use std::{path::PathBuf, str::FromStr, sync::LazyLock};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{options::TemplateOptions, template::Template};

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = format!(r"^\s*(exec)\s+([^,]+?)\s*(?:::\s*(.+))?\s*$");
    Regex::new(&pattern).expect("invalid regex pattern")
});

enum ExecLanguage {
    C,
    Cpp,
    Rust,
    JavaScript,
    TypeScript,
    Python,
}

impl FromStr for ExecLanguage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "c" => Ok(Self::C),
            "cpp" | "c++" => Ok(Self::Cpp),
            "rust" | "rs" => Ok(Self::Rust),
            "javascript" | "js" => Ok(Self::JavaScript),
            "typescript" | "ts" => Ok(Self::TypeScript),
            "python" | "py" => Ok(Self::Python),

            _ => Err(()),
        }
    }
}

impl ExecLanguage {
    pub fn default_binary(&self) -> PathBuf {
        match self {
            ExecLanguage::C => PathBuf::from("/usr/bin/gcc"),
            ExecLanguage::Cpp => PathBuf::from("/usr/bin/g++"),
            ExecLanguage::Rust => PathBuf::from("/usr/bin/rustc"),
            ExecLanguage::JavaScript => PathBuf::from("/usr/bin/node"),
            ExecLanguage::TypeScript => PathBuf::from("/usr/bin/ts-node"), // this will have to change
            ExecLanguage::Python => PathBuf::from("/usr/bin/python"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ExecOptions {
    language: String,
    binary: String,
}

impl Default for ExecOptions {
    fn default() -> Self {
        Self {
            language: "python".to_string(),
            binary: "/usr/bin/python3".to_string(),
        }
    }
}

impl TemplateOptions for ExecOptions {}

#[derive(Debug)]
pub struct ExecTemplate {
    code: String,
    options: ExecOptions,
    args: Vec<String>,
}

impl ExecTemplate {
    pub fn new() -> ExecTemplate {
        Self {
            code: String::new(),
            options: ExecOptions::default(),
            args: Vec::new(),
        }
    }
}

impl Template for ExecTemplate {
    fn descriminator(&self) -> &str {
        "exec"
    }

    fn parse(&self, s: &str) -> anyhow::Result<Box<dyn Template>> {
        let captures = REGEX
            .captures(s)
            .ok_or(anyhow::anyhow!("Invalid exec template structure"))?;

        let mut captures = captures.iter();
        captures.next();

        let _desc = captures.next();
        let code = captures
            .next()
            .ok_or(anyhow::anyhow!("Could not get paths for include statment!"))?
            .unwrap()
            .as_str()
            .to_string();

        let opts = if let Some(opts) = captures.next() {
            let opts = opts.unwrap().as_str();
            ExecOptions::parse(opts)?
        } else {
            ExecOptions::default()
        };

        Ok(Box::new(Self {
            code,
            options: opts,
            args: Vec::new(),
        }))
    }

    fn exec(&self) -> anyhow::Result<String> {
        todo!()
    }
}
