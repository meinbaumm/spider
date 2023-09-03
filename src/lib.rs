use std::collections::HashMap;
use std::fs;

use regex::Regex;

const SPIDER_ENV_VARIABLE: &str = "SPIDER_FILE";
const DEFAULT_SPIDER_FILE_PATH: &str = "web-search-urls.spider";

fn get_spider_file(name: &str) -> String {
    let default_file_path = DEFAULT_SPIDER_FILE_PATH.to_string();
    std::env::var(name).unwrap_or(default_file_path)
}

/// A struct wrapper for the web search url.
struct URL {
    /// The URL to search for
    path: String,
}

impl URL {
    fn new(url: String) -> URL {
        URL { path: url }
    }

    fn format(&self, query: &str) -> String {
        format!("{}{}", self.path, query)
    }

    fn main_page(&self) -> &str {
        let pattern = r"^(https?://[^/]+/)";
        let regex = Regex::new(pattern).unwrap();

        regex.find(&self.path).unwrap().as_str()
    }
}

/// Collection of web search urls.
struct WebSearchURLs {
    /// A HashMap with the name of the search site and the URL to search for
    urls: HashMap<String, URL>,
    /// Parsed file content. It is necessary for further filling of the `urls` HashMap
    file_content: String,
}

impl WebSearchURLs {
    fn new() -> WebSearchURLs {
        WebSearchURLs {
            urls: HashMap::new(),
            file_content: "".to_string(),
        }
    }

    fn read_urls_file(mut self, web_urls_file_name: &str) -> WebSearchURLs {
        let file = fs::read_to_string(web_urls_file_name);
        match file {
            Ok(file_content) => {
                self.file_content = file_content;
                self
            }
            Err(err) => panic!("Error opening file: {}", err),
        }
    }

    fn split_and_flesh_out(mut self) -> WebSearchURLs {
        self.file_content.lines().for_each(|line| {
            let mut split = line.split_whitespace();
            let name = split.next().unwrap();
            let url = split.next().unwrap();
            self.urls
                .insert(name.to_string(), URL::new(url.to_string()));
        });

        self
    }

    fn get(&self, name: &str) -> Option<&URL> {
        self.urls.get(name)
    }
}

fn open_url(url: &str) {
    match open::that(url) {
        Ok(()) => println!("Opened '{}'.", url),
        Err(err) => panic!("An error occurred when opening '{}': {}", url, err),
    }
}

fn read_spider_file() -> WebSearchURLs {
    WebSearchURLs::new()
        .read_urls_file(&get_spider_file(SPIDER_ENV_VARIABLE))
        .split_and_flesh_out()
}

pub fn web_search(web_site_name: &str, search_term: &Option<String>) {
    let web_search_urls = read_spider_file();

    let url = {
        let url = web_search_urls.get(&web_site_name);

        match url {
            Some(url) => url,
            None => {
                println!("Unknown website '{}'.", web_site_name);
                return;
            }
        }
    };

    match search_term {
        Some(search_term) => open_url(&url.format(&search_term)),
        None => open_url(&url.main_page()),
    }
}

pub fn list_web_search_urls() {
    let web_search_urls = read_spider_file();

    let max_len = web_search_urls
        .urls
        .iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap();

    web_search_urls.urls.iter().for_each(|(name, url)| {
        println!("{:width$} {}", name, url.main_page(), width = max_len + 1)
    });
}
