//! # weblib
//!
// `weblib` is a Rust library for making HTTP requests.
//!
//! ## Examples
//!
//! ```rust
//! use weblib::text;
//!
//! fn main() {
//!     let url = "https://httpbin.org/get";
//!     let response = text(url).unwrap();
//!     println!("{}", response);
//! }
//! ```
//!
//! ## License
//!
//! This project is licensed under the terms of the GPL-3.0 license.
//!
//! ## Author
//!
//! This library was created by Mac Lawson.
use reqwest::blocking::get;
use reqwest::{RequestBuilder, Error};
use reqwest::blocking::{Client, Response};
use std::error::Error as OtherError;
use std::time::Duration;


/// Fetches the contents of a URL and returns them as a `String`.
///
/// # Arguments
///
/// * `url` - A string slice that holds the URL to fetch.
///
/// # Returns
///
/// This function returns a `Result` object that holds a `String` with the
/// contents of the fetched URL, or an error message if the fetch fails.
///
/// # Examples
///
/// ```
/// let url = "https://httpbin.org/ip";
/// match weblib::text(url) {
///     Ok(resp) => println!("{}", resp),
///     Err(e) => panic!("Error: {}", e),
/// }
/// ```
pub fn text(url: &str) -> Result<String, Box<dyn OtherError>> {
    let resp = get(url)?.text()?;
    Ok(resp)
}


/// Fetches the contents of a URL and returns them as a `String`.
///
/// # Arguments
///
/// * `url` - A string slice that holds the URL to fetch.
///
/// # Returns
///
/// This function returns a `Result` object that holds a `String` with the
/// contents of the fetched URL, or an error message if the fetch fails.
///
/// # Examples
///
/// ```
/// let url = "https://httpbin.org/get";
/// let query = "key1=value1&key2=value2";
/// match weblib::query(url, query) {
///     Ok(resp) => println!("{}", resp),
///     Err(e) => panic!("Error: {}", e),
/// }
/// ```
pub fn query(url: &str, query_string: &str) -> Result<String, Box<dyn OtherError>> {
    let url_with_query_string = format!("{}?{}", url, query_string);
    let resp = get(&url_with_query_string)?.text()?;
    Ok(resp)
}


/// Sends a POST request to the specified URL and returns the response as a `String`.
///
/// # Arguments
///
/// * `url` - A string slice that holds the URL to send the request to.
/// * `data` - A string slice that holds the data to send in the request body.
///
/// # Returns
///
/// This function returns a `Result` object that holds a `String` with the
/// contents of the response, or an error message if the request fails.
///
/// # Examples
///
/// ```
/// let url = "https://httpbin.org/post";
/// let data = "key1=value1&key2=value2";
/// match weblib::post(url, data) {
///     Ok(resp) => println!("{}", resp),
///     Err(e) => panic!("Error: {}", e),
/// }
/// ```
pub fn post(url: &str, data: &str) -> Result<String, Box<dyn OtherError>> {
    let client = Client::new();
    let resp = client.post(url).body(data.to_string()).send()?;
    let body = resp.text()?;
    Ok(body)
}

/// Fetches the contents of a URL using HTTP Basic authentication and returns them as a `String`.
///
/// # Arguments
///
/// * `url` - A string slice that holds the URL to fetch.
/// * `username` - A string slice that holds the username for HTTP Basic authentication.
/// * `password` - A string slice that holds the password for HTTP Basic authentication.
///
/// # Returns
///
/// This function returns a `Result` object that holds a `String` with the
/// contents of the fetched URL, or an error message if the fetch fails.
///
/// # Examples
///
/// ```
/// let url = "https://httpbin.org/basic-auth/user/passwd";
/// match weblib::basic_auth(url, "user", "passwd") {
///     Ok(resp) => println!("{}", resp),
///     Err(e) => panic!("Error: {}", e),
/// }
/// ```
pub fn basic_auth(url: &str, username: &str, password: &str) -> Result<String, Box<dyn OtherError>> {
    let client = Client::new();
    let resp = client
        .request(reqwest::Method::GET, url)
        .basic_auth(username, Some(password))
        .send()?;
    let body = resp.text()?;
    Ok(body)
}
/// Send a request to the specified URL with retries and a timeout
///
/// # Arguments
///
/// * url - A string slice that holds the URL to be requested.
/// * retries - The number of retries to perform in case of a request error.
/// * timeout - The duration after which the request should timeout if it is not successful.
///
/// # Returns
///
/// * Result<Response, Error> - A Result type that holds the Response object on success or an Error object on failure.
///
/// # Examples
///
/// 
/// use std::time::Duration; 
/// use weblib::request_with_retries; 
/// let url = "https://example.com";
/// let retries = 3; 
/// let timeout = Duration::from_secs(10);
/// match request_with_retries(url, retries, timeout) { 
/// Ok(response) => println!("Response: {:?}", response), 
/// Err(e) => println!("Error: {:?}", e), 
/// } 
///
pub fn request_with_retries(
    url: &str,
    retries: usize,
    timeout: Duration,
) -> Result<Response, Error> {
    let client = Client::builder()
        .timeout(timeout)
        .build()
        .unwrap();

    let mut response = client.get(url).send();

    for _ in 1..retries {
        match response {
            Ok(res) => return Ok(res),
            Err(e) => response = client.get(url).send(),
        }
    }

    response
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_function_compiles() {
        let url = "https://httpbin.org/ip";
        match get(url) {
            Ok(_) => assert!(true),
            Err(e) => panic!("Error: {}", e),
        }
    }
    #[test]
    fn test_query_function() {
        let url = "https://httpbin.org/get";
        let query_string = "key1=value1&key2=value2";
        let response = query(url, query_string).unwrap();
        assert!(response.contains("\"key1\": \"value1\""));
        assert!(response.contains("\"key2\": \"value2\""));
    }
    #[test]
    fn test_post_function_compiles() {
        let url = "https://httpbin.org/post";
        let body = "test data";
        let response = post(url, body);
        assert!(response.is_ok());
    }
    #[test]
    fn test_basic_auth_function_compiles() {
        let url = "https://httpbin.org/basic-auth/user/passwd";
        let username = "user";
        let password = "passwd";
        let _response = basic_auth(url, username, password);
    }
    #[test]
    fn test_send_request_with_retries_compiles() {
        let url = "https://example.com";
        let retries = 3;
        let timeout = Duration::from_secs(30);
    
        let _result = request_with_retries(url, retries, timeout);
    }
    
    
    

}