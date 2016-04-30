extern crate hyper;

use self::hyper::Client;
use self::hyper::header::Connection;

use std::io::Read;

pub fn download(url: &str) -> String {
    let client = Client::new();
    let mut res = client.get(url)
        //.header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    return body
}
