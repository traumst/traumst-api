use crate::config;
use serde::{Deserialize, Serialize};
use serde_json;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailRequest {
    pub sender: String,
    pub topic: String,
    pub body: String,
}

pub fn parse_request(json: &str) -> Result<EmailRequest, String> {
    match serde_json::from_str(json) {
        Ok(email) => { Ok(email) },
        _ => { Err("Incorrect json".to_string()) }
    }
}

pub fn send_email(json: EmailRequest) -> Result<String, String> {
    let email = Message::builder()
        .from(json.sender.parse().unwrap())
        .to("dutraumst@gmail.com".parse().unwrap())
        .subject(json.topic)
        .body(json.body.to_string())
        .unwrap();

    let creds = Credentials::new(config::smtp_user(), config::smtp_pass());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .expect("SMTP did not connect")
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok("Email sent successfully!".to_string()),
        Err(e) => Err(format!("Could not send email: {e:?}"))
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