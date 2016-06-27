// google-direction -- Google Direction API wrapper for Rust.
// Copyright 2016 Pocket7878 <poketo7878@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.
use models::RouteResult;
use std::io::Read;
use rustc_serialize::json;
use hyper::client;
use hyper::Url;

pub struct Client {
    api_key: String
}

impl Client {
    pub fn new(key: String) -> Client {
        return Client { api_key: key };
    }

    pub fn send(&self, origin: String, destination: String) -> RouteResult {
        let client = client::Client::new();
        let mut route_api_url = Url::parse("https://maps.googleapis.com/maps/api/directions/json").unwrap();
        route_api_url.query_pairs_mut()
            .append_pair("origin", &origin)
            .append_pair("destination", &destination)
            .append_pair("key", &self.api_key);

        let mut res = client.get(route_api_url).send().unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        let route_result: RouteResult = json::decode(&buffer).unwrap();
        return route_result;
    }
}
