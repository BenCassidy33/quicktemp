use std::path::PathBuf;

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
