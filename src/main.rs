use std::env;
use std::error::Error;

use getopts::Options;
use log::{error, info, LevelFilter};

use html_to_email::cmd::HtmlToEmail;

fn opts() -> Options {
    let mut opts = Options::new();
    {
        opts.optopt("f", "from", "Set sender address", "FROM");
        opts.optopt("t", "to", "Set receiver address", "TO");
        opts.optflag("v", "verbose", "Verbose printing");
        opts.optflag("h", "help", "Print this help");
        opts.optflag("", "about", "Show about");
    }
    opts
}

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .init();

    let opts = opts();
    let input: Vec<String> = env::args().collect();
    let args = opts.parse(&input[1..]).expect("parse argument failed");

    match () {
        _ if args.opt_present("about") => {
            println!("{}", "Visit https://github.com/gonejack/html_to_email");
            return;
        }
        _ if args.opt_present("h") => {
            println!("{}", opts.usage("Usage: html_to_email *.html"));
            return;
        }
        _  if args.free.is_empty() => {
            error!(target: "argument", "No .html files given");
            return;
        }
        _ => {}
    }

    let from: String = args.opt_str("from").unwrap_or("sender@exmail.com".to_string());
    let to: String = args.opt_str("to").unwrap_or("receiver@example.com".to_string());

    for html in args.free {
        info!("process {}", html);

        let result = conv(&html, &from, &to);
        if let Err(e) = result {
            error!("parse {} failed: {}", html, e);
        }
    }
}

fn conv(html: &str, from: &str, to: &str) -> Result<(), Box<dyn Error>> {
    HtmlToEmail::new(html, from, to).run()
}
