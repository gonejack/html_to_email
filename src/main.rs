use std::error::Error;

use clap::{App, Arg, ArgMatches};
use log::{info, error, LevelFilter};

use html_to_email::cmd::HtmlToEmail;

fn args() -> ArgMatches<'static> {
    App::new("html_to_email")
        .about("https://github.com/gonejack/html_to_email")
        .args(&[
            Arg::from_usage("-f, --from=[FROM] 'set sender address'"),
            Arg::from_usage("-t, --to=[TO] 'set receiver address'"),
            Arg::from_usage("-v, --verbose 'verbose printing'"),
            Arg::with_name("html").multiple(true)
        ])
        .get_matches()
}

fn conv(html: String, from: String, to: String) -> Result<(), Box<dyn Error>> {
    HtmlToEmail::new(html, from, to).run()
}

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .init();

    let args = args();

    let mut from = args.value_of("from").unwrap_or_default();
    if from.is_empty() {
        from = "sender@example.com"
    }
    let mut to = args.value_of("to").unwrap_or_default();
    if to.is_empty() {
        to = "receiver@example.com"
    }
    let htms = args.values_of("html").unwrap_or(Default::default());

    if htms.to_owned().count() == 0 {
        error!("not html given");
        return;
    }

    for htm in htms {
        info!("process {}", htm);

        let res = conv(htm.to_string(), from.to_string(), to.to_string());
        if res.is_err() {
            error!("parse {} failed: {}", htm, res.err().unwrap());
        }
    }
}

