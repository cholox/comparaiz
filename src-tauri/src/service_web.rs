use reqwest::Error;

// Do a request for the given URL with time between requests.
pub fn do_throttled_request(url: &str) -> Result<String, Error> {
    let response = reqwest::blocking::get(url)?;
    response.text()
}