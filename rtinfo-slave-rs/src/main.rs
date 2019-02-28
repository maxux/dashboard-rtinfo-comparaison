#![feature(duration_float)]

use redis::Commands;
use serde_json::json;
use std::thread::sleep;
use std::time::{Duration,Instant};
use failure::Error;

const RTINFO_ENDPOINT: &str = "http://clea.maxux.net:8089/json";
const REDIS_TARGET: &str = "redis://127.0.0.1/";


fn redis_init() -> Result<redis::Connection,Error> {
    let client = redis::Client::open(REDIS_TARGET)?;
    Ok(client.get_connection()?)
}

fn rtinfo_fetch() -> Result<String,Error> {
    let now = Instant::now();
    let data = reqwest::get(RTINFO_ENDPOINT)?.text()?;
    println!("\t[-] Downloading took {} ",now.elapsed().as_float_secs());
    Ok(data)
}

fn dashboard_encode(payload: String) -> Result<String,Error> {
    let now = Instant::now();
    // parse json
    let rtinfo: serde_json::Value = serde_json::from_str(&payload)?;
//    if rtinfo["rtinfo"].is_array() == false {
//        panic!("malformed json from rtinfo");
//    }

    let hosts = rtinfo["rtinfo"].as_array().expect("Malformed JSON from rtinfo");
    println!("[+] rtinfo: {} hosts found", hosts.len());

    // encapsulate json into dashboard frame
    let converted = json!({
        "id": "rtinfo",
        "payload": rtinfo
    });

    // dumps jsono
    println!("\t[-] Encoding took {} ",now.elapsed().as_float_secs());
    Ok(converted.to_string())
}

fn redis_publish(con: &redis::Connection, payload: String) -> Result<(),Error> {
    Ok(con.publish("dashboard", payload)?)
}

fn main() -> Result<(),Error> {
    let onesec = Duration::new(1, 0);
    let client = redis_init()?;

    loop {
        println!("[+] rtinfo: fetching");
        let payload = rtinfo_fetch()?;

        let converted = dashboard_encode(payload)?;
        redis_publish(&client, converted)?;

        // waiting next fetch
        sleep(onesec);
    }
}

