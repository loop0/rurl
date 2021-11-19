use reqwest;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Write};

#[macro_use]
extern crate clap;

#[derive(Deserialize, Debug)]
struct RequestConfig {
    url: String,
    method: String,
    headers: Option<Map<String, Value>>,
    body: Option<String>,
}

fn read_request_config(path: String) -> Result<RequestConfig, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}

fn rurl(config: RequestConfig, verbose: bool) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    let mut request;

    // select correct method
    match config.method.as_str() {
        "get" => request = client.get(config.url),
        "post" => request = client.post(config.url),
        _ => panic!("Unsupported http method"),
    }

    // set headers
    if let Some(headers) = config.headers {
        for (k, v) in headers {
            if let Some(value) = v.as_str() {
                request = request.header(&k, value);
            }
        }
    }
    // set body
    if let Some(body) = config.body {
        request = request.body(body);
    }
    // execute the request
    let response = request.send()?;
    if verbose {
        for (key, value) in response.headers() {
            let output = format!("{}: {:?}\n", key, value);
            stderr.write_all(output.as_bytes())?;
        }
    }
    stdout.write_all(&response.bytes()?)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = clap_app!(app =>
        (name: "rurl")
        (version: "0.1.0")
        (author: "Bruno Ribeiro da Silva <bruno.devpod@gmail.com>")
        (about: "Like curl but with configuration by json files")
        (@arg VERBOSE: -v --verbose "Enables verbose mode")
        (@arg CONFIG: +required "Sets the input file to use for the request configuration")
    )
    .get_matches();

    let verbose = args.is_present("VERBOSE");
    let config_path = String::from(args.value_of("CONFIG").unwrap());
    let config = read_request_config(config_path)?;
    rurl(config, verbose)?;
    Ok(())
}
