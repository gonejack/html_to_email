use std::error::Error;
use std::fs;
use std::path::Path;

use lettre::Message;
use lettre::message::header::{ContentTransferEncoding, ContentType};
use lettre::message::SinglePartBuilder;
use log::warn;
use visdom::types::Elements;
use visdom::Vis;

pub struct HtmlToEmail {
    html: String,
    from: String,
    to: String,
}

impl HtmlToEmail {
    pub fn new(html: String, from: String, to: String) -> Self {
        Self {
            html,
            from,
            to,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let output = self.html.trim_end_matches(".html").to_string() + ".eml";

        if Path::new(&output).exists() {
            warn!("target {} exist", output);
            return Ok(());
        }

        let data = fs::read_to_string(self.html.clone())?;
        let doc = Vis::load(data.as_str())?;

        self.clean_doc(&doc);

        let title = doc.find("title").text().to_string();
        let body = SinglePartBuilder::new()
            .header(ContentTransferEncoding::Base64)
            .header(ContentType::TEXT_HTML)
            .body(doc.html());
        let email = Message::builder()
            .from(self.from.parse()?)
            .to(self.to.parse()?)
            .subject(title)
            .singlepart(body)?;

        fs::write(output, email.formatted())?;

        Ok(())
    }

    pub fn clean_doc(&self, doc: &Elements) {
        doc.find(r#"div:contains("ads from inoreader")"#).closest("center").remove();
        doc.find(r#"img[src='https://img.solidot.org//0/446/liiLIZF8Uh6yM.jpg']"#).remove();

        doc.find("iframe").remove();
        doc.find("link").remove();
        doc.find("script").remove();
        doc.find("button").remove();
        doc.find("input").remove();

        doc.find("*[contenteditable=true]").remove_attr("contenteditable");
    }
}
