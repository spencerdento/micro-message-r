use lettre::{SmtpTransport, Transport, smtp::response::Response};

pub fn send_email(to: &str, name: &str, domain: &str, username: &str, password: &str) -> anyhow::Result<Response> {
    let email = lettre_email::EmailBuilder::new()
      .to((to, name))
      .from(("micromessager@gmail.com", "Test Email"))
      .subject("Sent from my uMessage-r")
      .text("What did you do today?")
      .build()?;

    let mut mailer = make_smtp_transport(domain, username, password)?;
    
    let result = mailer.send(email.into())?;
    Ok(result)
}

fn make_smtp_transport(domain: &str, username: &str, password: &str) -> anyhow::Result<SmtpTransport> {
    let mailer = lettre::SmtpClient::new_simple(domain)?;
    let mailer = mailer.credentials(lettre::smtp::authentication::Credentials::new(username.into(), password.into())).transport();

    Ok(mailer)
}