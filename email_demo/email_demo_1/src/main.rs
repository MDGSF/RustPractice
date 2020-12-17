use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn main() {
  let email = Message::builder()
    .from("huangjian@minieye.cc".parse().unwrap())
    .to("1342042894@qq.com".parse().unwrap())
    .subject("Hi, Hello world")
    .body("Hello world.")
    .unwrap();

  let creds = Credentials::new("username".to_string(), "password".to_string());

  // Open a remote connection to gmail
  let mailer = SmtpTransport::relay("smtp.exmail.qq.com")
    .unwrap()
    .credentials(creds)
    .build();

  // Send the email
  let result = mailer.send(&email);

  if result.is_ok() {
    println!("Email sent");
  } else {
    println!("Could not send email: {:?}", result);
  }

  assert!(result.is_ok());
}
