use std::env;
use std::error::Error;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};


pub fn send_mail(reciever: &str, subject: &str, body: String) -> Result<(), Box<dyn Error>> {
    let smtp_email = env::var("SMTP_EMAIL").expect("Missing the SMTP_EMAIL environment variable.");
    let smtp_password = env::var("SMTP_PASSWORD").expect("Missing the SMTP_PASSWORD environment variable.");
    let smtp_server = env::var("SMTP_SERVER").expect("Missing the SMTP_SERVER environment variable.");
    

    let email = Message::builder()
        .from(
            "Info FL <info@fetchland.eu>"
                .parse()
                .unwrap()
        )
        .reply_to(
            "Info FL <info@fetchland.eu>"
                .parse()
                .unwrap()
        )
        .to(
            format!("<{}>", reciever)
                .as_str()
                .parse()
                .unwrap()
        )
        .subject(subject)
        .body(body)
        .unwrap();

    // Connect to a remote server on a custom port
    let mailer = match SmtpTransport::starttls_relay(&smtp_server) {
        Ok(mailer_builder) => mailer_builder
            .credentials(Credentials::new(
                smtp_email, 
                smtp_password
            )).build(),
        Err(e) => return Err(e.into())
    };

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}