use reqwest::blocking::get;
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
/// match web::text(url) {
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
/// match web::query(url, query) {
///     Ok(resp) => println!("{}", resp),
///     Err(e) => panic!("Error: {}", e),
/// }
/// ```
pub fn query(url: &str, query_string: &str) -> Result<String, Box<dyn Error>> {
    let url_with_query_string = format!("{}?{}", url, query_string);
    let resp = get(&url_with_query_string)?.text()?;
    Ok(resp)
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
}
