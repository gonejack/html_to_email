use clap::Parser;

use html_to_email::cmd::HtmlToEmail;
use nu_ansi_term::Color;
use time::macros::format_description;
use time::UtcOffset;
use tracing::{error, info};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::{FormatTime, OffsetTime};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::EnvFilter;

struct MyFormatter;

impl<S, N> FormatEvent<S, N> for MyFormatter
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
        let timer = OffsetTime::new(offset,format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"));
        let meta = event.metadata();

        let level = *meta.level();
        let level_str = match level {
            tracing::Level::ERROR => Color::Red.bold().paint("ERRO"),
            tracing::Level::WARN => Color::Yellow.bold().paint("WARN"),
            tracing::Level::INFO => Color::Green.bold().paint("INFO"),
            tracing::Level::DEBUG => Color::Blue.bold().paint("DBUG"),
            tracing::Level::TRACE => Color::Purple.bold().paint("TRAC"),
        };

        write!(writer, "[")?;
        timer.format_time(&mut writer)?;
        write!(writer, "][{}][{}] ", level_str, meta.target())?;
        ctx.format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

/// Convert .html files to .eml files
#[derive(Parser)]
#[command(
    name = "html_to_email",
    about = "This command line converts .html file to .eml file."
)]
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

    let directives = if cli.verbose {
        "debug,hyper=info,hyper_util=info,h2=info,rustls=info,reqwest=info"
    } else {
        "info"
    };
    let filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .parse_lossy(directives);
    tracing_subscriber::fmt()
        .event_format(MyFormatter)
        .with_env_filter(filter)
        .init();

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
