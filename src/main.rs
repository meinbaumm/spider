use std::collections::HashMap;
use std::fs;

use clap::{Parser, Subcommand};

const DEFAULT_WEB_URLS_FILE_NAME: &str = "web-search-urls";

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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search on the web
    Web {
        /// The name of the web site to search
        #[arg(value_parser = clap::builder::NonEmptyStringValueParser::new())]
        web_site_name: String,
        /// The search term
        #[arg(value_parser = clap::builder::NonEmptyStringValueParser::new())]
        search_term: String,
    },
}

fn open_url(url: &str) {
    match open::that(url) {
        Ok(()) => println!("Opened '{}' successfully.", url),
        Err(err) => panic!("An error occurred when opening '{}': {}", url, err),
    }
}

fn main() {
    let spider = Cli::parse();

    match &spider.command {
        Commands::Web {
            web_site_name,
            search_term,
        } => {
            let web_search_urls = WebSearchURLs::new()
                .read_urls_file(DEFAULT_WEB_URLS_FILE_NAME)
                .split_and_flesh_out();

            let url = web_search_urls.get(&web_site_name).unwrap();

            open_url(&url.format(&search_term));
        }
    }
}
