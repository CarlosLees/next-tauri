fn main() {
    let uri = "https://api.ipify.org/?format=text";
    let response =  reqwest::blocking::get(uri).expect("Failed to send request");
    //println!(" response.status:{}", response.status());
    if let Ok(body) = response.text() {
        //println!("Response body:\n{}", body);
        println!("{:?}", body)
    }
}