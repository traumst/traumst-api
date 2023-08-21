use crate::config;
use serde::{Deserialize, Serialize};
use serde_json;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailRequest {
    pub sender: String,
    pub topic: String,
    pub body: String,
}

pub fn parse_request(json: &str) -> Result<EmailRequest, String> {
    match serde_json::from_str(json) {
        Ok(email) => { Ok(email) },
        Err(e) => { Err(format!("Bad input json: {e:?}")) }
    }
}

pub fn send_email(json: EmailRequest) -> Result<String, String> {
    let user = config::smtp_user();
    let sender = format!("Email Contact Form <{user}>");
    let receiver = config::email_receiver();
    debug!("sending email from {sender} to {receiver}");
    let email = Message::builder()
        .from(sender.parse().unwrap())
        .to(receiver.parse().unwrap())
        .subject(format!("{} by <{}>", json.topic, json.sender))
        .body(json.body.to_string())
        .unwrap();

    let pass = config::smtp_pass();
    let creds = Credentials::new(user, pass);

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .expect("SMTP did not connect")
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(res) => Ok(format!("Email sent successfully! {res:?}")),
        Err(err) => Err(format!("Failed to send an email, {err:?}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_request() {
        let json = r#"
        {
            "sender": "TestUser",
            "topic": "TestSubject",
            "body": "test body bla bla"
        }
        "#;

        let result = parse_request(json).unwrap();
        assert_eq!(result.sender, "TestUser");
        assert_eq!(result.topic, "TestSubject");
        assert_eq!(result.body, "test body bla bla");
    }

    #[test]
    #[should_panic]
    fn test_bad_parse_request() {
        let json = "wait this is not a json";

        parse_request(json).unwrap();
    }
}