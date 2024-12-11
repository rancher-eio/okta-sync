use std::str::FromStr;

fn main() {
  for arg in std::env::args().skip(1) {
    eprintln!("arg = {arg}");
    let email_address = email_address::EmailAddress::from_str(&arg).expect("error (email_address)");
    println!("email_address = {email_address:#?}");
    let display_part = email_address.display_part();
    println!("\tdisplay_part = {display_part}");
    let domain = email_address.domain();
    println!("\tdomain = {domain}");
    let email = email_address.email();
    println!("\temail = {email}");
    let local_part = email_address.local_part();
    println!("\tlocal_part = {local_part}");
    let uri = email_address.to_uri();
    println!("\turi = {uri}");
  }
}
