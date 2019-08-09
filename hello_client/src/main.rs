fn main() {
    let mut response = reqwest::get("http://localhost:8000").expect("Unable to retrieve response");
    println!("{:?}", response);
    let body = response.text().expect("Unable to get body");
    println!("{}", body);
}
