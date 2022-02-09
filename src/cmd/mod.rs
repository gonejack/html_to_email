use std::fs;
use std::ops::Add;
use std::path::Path;

use lettre::message::header::{ContentTransferEncoding, ContentType};
use lettre::message::SinglePartBuilder;
use lettre::Message;
use log::warn;
use visdom::types::{BoxDynError, Elements};
use visdom::Vis;

pub struct HtmlToEmail<'a> {
    html: &'a str,
    from: &'a str,
    to: &'a str,
}

impl<'a> HtmlToEmail<'a> {
    pub fn new(html: &'a str, from: &'a str, to: &'a str) -> Self {
        Self { html, from, to }
    }

    pub fn run(&self) -> Result<(), BoxDynError> {
        let output = self.html.trim_end_matches(".html").to_string().add(".eml");

        if Path::new(&output).exists() {
            warn!("target {} exist", output);
            return Ok(());
        }
        let data = fs::read_to_string(self.html)?;
        let doc = Vis::load(&data)?;

        self.clean_doc(&doc);
        let title = doc.find("title").text().to_string();
        let mut pb = SinglePartBuilder::new();
        {
            pb = pb.header(ContentTransferEncoding::Base64);
            pb = pb.header(ContentType::TEXT_HTML);
        }
        let part = pb.body(doc.html());

        let mut mb = Message::builder();
        {
            mb = mb.from(self.from.parse()?);
            mb = mb.to(self.to.parse()?);
            mb = mb.subject(title);
        }
        let eml = mb.singlepart(part)?;

        fs::write(output, eml.formatted())?;

        Ok(())
    }

    pub fn clean_doc(&self, doc: &Elements) {
        doc.find(r#"div:contains("ads from inoreader")"#)
            .closest("center")
            .remove();
        doc.find(r#"img[src='https://img.solidot.org//0/446/liiLIZF8Uh6yM.jpg']"#)
            .remove();

        doc.find("iframe").remove();
        doc.find("link").remove();
        doc.find("script").remove();
        doc.find("button").remove();
        doc.find("input").remove();

        doc.find("*[contenteditable=true]")
            .remove_attr("contenteditable");
    }
}
