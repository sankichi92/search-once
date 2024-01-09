use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use percent_encoding::NON_ALPHANUMERIC;
use serde::{Deserialize, Serialize};

fn main() -> Result<()> {
    let args = Cli::parse();

    let config_path = args.config.unwrap_or(
        confy::get_configuration_file_path(env!("CARGO_PKG_NAME"), None)
            .context("could not determine the default config path")?,
    );

    println!("Config: {}", config_path.display());
    let config: Config = confy::load_path(config_path).context("could not load the config file")?;

    if config.sites.len() <= 1 {
        eprintln!("WARNING: Only one site configured. Please add more sites to your config file.");
    }

    let encoded_query =
        percent_encoding::utf8_percent_encode(&args.query, NON_ALPHANUMERIC).to_string();

    for site in config.sites {
        let url = site.url.replace("%s", &encoded_query);
        println!("{}:\t{}", site.name, url);
        open::that(url).context("could not open the URL")?;
    }

    Ok(())
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    query: String,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    sites: Vec<Site>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sites: vec![Site {
                name: "GitHub".to_string(),
                url: "https://github.com/search?q=%s".to_string(),
            }],
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Site {
    name: String,
    url: String,
}
