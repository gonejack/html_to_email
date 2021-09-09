use std::env;
use std::error::Error;

use getopts::{Options};
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

fn conv(html: String, from: String, to: String) -> Result<(), Box<dyn Error>> {
    HtmlToEmail::new(html, from, to).run()
}

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .init();

    let opts = opts();

    let input: Vec<String> = env::args().collect();
    let args = match opts.parse(&input[1..]) {
        Ok(m) => { m }
        Err(..) => {
            panic!("parse argument failed")
        }
    };

    if args.opt_present("about") {
        println!("{}", "Visit https://github.com/gonejack/html_to_email");
        return
    }
    if args.opt_present("h") {
        println!("{}", opts.usage("Usage: html_to_email *.html"));
        return;
    }
    if args.free.is_empty() {
        error!("no html file given");
        return;
    }

    let from = args.opt_str("from").unwrap_or("sender@exmail.com".to_string());
    let to = args.opt_str("to").unwrap_or("receiver@example.com".to_string());

    for html in args.free {
        info!("process {}", html);

        let result = conv(html.clone(), from.clone(), to.clone());

        if result.is_err() {
            error!("parse {} failed: {}", html, result.err().unwrap());
        }
    }
}

