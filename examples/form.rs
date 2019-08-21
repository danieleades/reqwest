#![feature(async_await)]

#[tokio::main]
async fn main() {
    reqwest::Client::new()
        .post("http://www.baidu.com")
        .form(&[("one", "1")])
        .send()
        .unwrap();
}
