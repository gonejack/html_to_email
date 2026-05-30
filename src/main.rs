use clap::Parser;
use tracing::{error, info, Level};

use html_to_email::cmd::HtmlToEmail;

/// Convert .html files to .eml files
#[derive(Parser)]
#[command(name = "html_to_email", about = "This command line converts .html file to .eml file.")]
struct Cli {
    /// Set sender address
    #[arg(short, long, default_value = "sender@example.com")]
    from: String,

    /// Set receiver address
    #[arg(short, long, default_value = "receiver@example.com")]
    to: String,

    /// Verbose printing
    #[arg(short, long)]
    verbose: bool,

    /// Show about
    #[arg(long)]
    about: bool,

    /// HTML files to convert
    #[arg(value_name = "FILE")]
    files: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt().with_max_level(level).init();

    if cli.about {
        println!("Visit https://github.com/gonejack/html_to_email");
        return;
    }

    if cli.files.is_empty() {
        error!(target: "argument", "No .html files given");
        return;
    }

    for html in &cli.files {
        info!("process {}", html);

        if let Err(e) = HtmlToEmail::new(html, &cli.from, &cli.to).run() {
            error!("parse {} failed: {}", html, e);
        }
    }
}
