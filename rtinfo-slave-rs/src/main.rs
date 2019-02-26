use redis::Commands;
use serde_json::json;
use std::{thread, time};

const RTINFO_ENDPOINT: &str = "http://clea.maxux.net:8089/json";
const REDIS_TARGET: &str = "redis://127.0.0.1/";

fn redis_init() -> redis::Connection {
    let client = redis::Client::open(REDIS_TARGET).unwrap();
    return client.get_connection().unwrap();
}

fn rtinfo_fetch() -> std::string::String {
    reqwest::get(RTINFO_ENDPOINT).unwrap().text().unwrap()
}

fn dashboard_encode(payload: std::string::String) -> std::string::String {
    // parse json
    let rtinfo: serde_json::Value = serde_json::from_str(&payload).unwrap();
    if rtinfo["rtinfo"].is_array() == false {
        panic!("malformed json from rtinfo");
    }

    let hosts = rtinfo["rtinfo"].as_array().unwrap();
    println!("[+] rtinfo: {} hosts found", hosts.len());

    // encapsulate json into dashboard frame
    let converted = json!({
        "id": "rtinfo",
        "payload": rtinfo
    });

    // dumps json
    converted.to_string()
}

fn redis_publish(con: &redis::Connection, payload: std::string::String) {
    let _: () = con.publish("dashboard", payload).unwrap();
}

fn main() {
    let onesec = time::Duration::new(1, 0);
    let client = redis_init();

    loop {
        println!("[+] rtinfo: fetching");
        let payload = rtinfo_fetch();

        let converted = dashboard_encode(payload);
        redis_publish(&client, converted);

        // waiting next fetch
        thread::sleep(onesec);
    }
}

