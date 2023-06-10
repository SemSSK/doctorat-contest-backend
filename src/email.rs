use lettre::{transport::smtp::authentication::Credentials, *};
pub fn send_email(subject: String, content: String) {
    let email = Message::builder()
        .to("<ssksem015@gmail.com>".parse().unwrap())
        .from("<me@hello.com>".parse().unwrap())
        .subject(subject)
        .body(content)
        .unwrap();
    let creds = Credentials::new(
        "doctoratemanager@gmail.com".to_string(),
        "ibjphxunpqkobtly".to_string(),
    );
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully"),
        Err(_) => println!("Failed to send email"),
    }
}
