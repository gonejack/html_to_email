use std::env;

use getopts::Options;
use log::{error, info, LevelFilter};

use html_to_email::cmd::HtmlToEmail;

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let mut opts = Options::new();
    {
        opts.optopt("f", "from", "Set sender address", "FROM");
        opts.optopt("t", "to", "Set receiver address", "TO");
        opts.optflag("v", "verbose", "Verbose printing");
        opts.optflag("h", "help", "Print this help");
        opts.optflag("", "about", "Show about");
    }

    let args_raw: Vec<String> = env::args().collect();
    let args = opts.parse(&args_raw[1..]).expect("parse argument failed");

    match () {
        _ if args.opt_present("about") => {
            println!("{}", "Visit https://github.com/gonejack/html_to_email");
            return;
        }
        _ if args.opt_present("h") => {
            println!("{}", opts.usage("Usage: html_to_email *.html"));
            return;
        }
        _ if args.free.is_empty() => {
            error!(target: "argument", "No .html files given");
            return;
        }
        _ => {}
    }

    let from = args
        .opt_str("from")
        .unwrap_or("sender@example.com".to_string());
    let to = args
        .opt_str("to")
        .unwrap_or("receiver@example.com".to_string());

    for html in args.free {
        info!("process {}", html);

        if let Err(e) = HtmlToEmail::new(&html, &from, &to).run() {
            error!("parse {} failed: {}", html, e);
        }
    }
}
