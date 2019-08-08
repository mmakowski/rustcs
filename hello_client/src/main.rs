fn main() {
    let body = reqwest::get("http://localhost:8000")
        .expect("Unable to retrieve response")
        .text()
        .expect("Unable to get body");
    println!("{}", body);
}
