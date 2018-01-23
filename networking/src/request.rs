#[no_mangle]
extern crate reqwest;
use std::io::Read;

pub fn request() -> String {
  let mut resp = reqwest::get("https://www.rust-lang.org")?;
  assert!(resp.status().is_success());
  let body = resp.text()?;
  return body;
}
