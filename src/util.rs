use regex::Regex;

pub struct SearchHighlighter {
    regex: Regex
}

impl SearchHighlighter {
    pub fn new(search_param: &str) -> SearchHighlighter {
        SearchHighlighter {
            regex: Regex::new(&format!("(.*?)({})(.*?)", search_param)).unwrap()
        }
    }

    pub fn highlight(&self, input: &str) -> String {
        self.regex.replace(input, "${1}__${2}__${3}").to_string()
    }
}