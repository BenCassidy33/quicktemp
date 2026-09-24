crate::regex!(REGEX, r"(?s)\{{\{{(.*?)\}}\}}");

pub fn preprocess(mut file: String) -> String {
    REGEX
        .replace_all(&file, |caps: &regex::Captures| {
            let inner = caps[1].replace('\n', "");
            format!("{{{{{inner}}}}}")
        })
        .into_owned()
}
