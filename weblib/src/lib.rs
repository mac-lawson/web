use reqwest::blocking::get;
use reqwest::blocking::Client;
use std::error::Error;

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
pub fn text(url: &str) -> Result<String, Box<dyn Error>> {
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
pub fn query(url: &str, query_string: &str) -> Result<String, Box<dyn Error>> {
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
pub fn post(url: &str, data: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let resp = client.post(url).body(data.to_string()).send()?;
    let body = resp.text()?;
    Ok(body)
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
}