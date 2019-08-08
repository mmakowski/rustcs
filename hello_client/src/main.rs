fn main() {
    let mut response = match reqwest::get("http://localhost:8000") {
        Ok(resp) => resp,
        Err(error) => {
            panic!("Unable to retrieve response: {:?}", error)
        }
    };
    let body = match response.text() {
        Ok(text) => text,
        Err(error) => {
            panic!("Unable to get body: {:?}", error)
        }
    };
    println!("{}", body);
}
