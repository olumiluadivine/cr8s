use lettre::message::header::ContentType;
use lettre::transport::smtp::{authentication::Credentials, response::Response};
use lettre::{Message, SmtpTransport, Transport};
use tera::Context;

pub struct HtmlMailer {
    pub credentials: Credentials,
    pub smtp_host: String,
    pub template_engine: tera::Tera,
}

impl HtmlMailer {
    pub fn send(
        self,
        to: String,
        template_name: &str,
        context: &Context,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let html_body = self
            .template_engine
            .render(template_name, &context)
            .unwrap();

        let message = Message::builder()
            .subject("Cr8s digest")
            .from("Cr8s <cr8s@cr8s.com>".parse().unwrap())
            .to(to.parse().unwrap())
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .unwrap();

        let mailer = SmtpTransport::relay(&self.smtp_host)
            .unwrap()
            .credentials(self.credentials)
            .build();

        mailer.send(&message).map_err(|e| e.into())
    }
}
