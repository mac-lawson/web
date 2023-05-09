use weblib::text;


fn main() {
    match text("https://httpbin.org/ip") {
        Ok(resp) => println!("{}", resp),
        Err(e) => eprintln!("Error: {}", e),
    }
    let url = "https://httpbin.org/get";
    let query_string = "key1=value1&key2=value2";
    match weblib::query(url, query_string) {
     Ok(resp) => println!("{}", resp),
     Err(e) => panic!("Error: {}", e),
 }
}
