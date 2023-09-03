use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "spider")]
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
        search_term: Option<String>,
    },
    /// List web sites in the spider file
    List {},
}

fn main() {
    let spider = Cli::parse();

    match &spider.command {
        Commands::Web {
            web_site_name,
            search_term,
        } => spider_web_search::web_search(&web_site_name, &search_term),

        Commands::List {} => spider_web_search::list_web_search_urls(),
    }
}
