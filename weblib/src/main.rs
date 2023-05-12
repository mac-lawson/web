use weblib::{self, text};


fn main() {
//     match weblib::text("https://httpbin.org/ip") {
//         Ok(resp) => println!("{}", resp),
//         Err(e) => eprintln!("Error: {}", e),
//     }
//     let url = "https://httpbin.org/get";
//     let query_string = "key1=value1&key2=value2";
//     match weblib::query(url, query_string) {
//      Ok(resp) => println!("{}", resp),
//      Err(e) => panic!("Error: {}", e),
//  }
//   let url = "https://httpbin.org/post";
//   let data = "key1=value1&key2=value2";
//   match weblib::post(url, data) {
//       Ok(resp) => println!("{}", resp),
//       Err(e) => panic!("Error: {}", e),
//   }
    retrytester();
}

fn testText() {
      let url = "https://httpbin.org/get";
      let response = text(url).unwrap();
      println!("{}", response);
}

fn authtester() {
 let url = "https://httpbin.org/basic-auth/user/passwd";
 match weblib::basic_auth(url, "user", "passwd") {
     Ok(resp) => println!("{}", resp),
     Err(e) => panic!("Error: {}", e),
 }
}

fn retrytester() {
  use std::time::Duration; 
  use weblib::request_with_retries; 
  let url = "https://example.com";
  let retries = 3; 
  let timeout = Duration::from_secs(10);
  match request_with_retries(url, retries, timeout) { 
  Ok(response) => println!("Response: {:?}", response), 
  Err(e) => println!("Error: {:?}", e), 
  } 
}