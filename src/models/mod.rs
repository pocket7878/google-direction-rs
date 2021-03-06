// google-direction -- Google Direction API wrapper for Rust.
// Copyright 2016 Pocket7878 <poketo7878@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.
#[derive(RustcDecodable, RustcEncodable)]
pub struct GeoCodedWaypoint {
    pub geocoder_status: String,
    pub partial_match: Option<bool>,
    pub place_id: String,
    pub types: Vec<String>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Fare {
    pub currency: String,
    pub value: usize,
    pub text: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct BoundBox {
    pub northeast: Location,
    pub southwest: Location,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Distance {
    pub value: usize,
    pub text: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Duration {
    pub value: usize,
    pub text: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct DurationInTraffic {
    pub value: usize,
    pub text: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Time {
    pub value: String,
    pub text: String,
    pub time_zone: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Location {
    pub lat: f64,
    pub lng: f64
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Polyline {
    pub points: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct TransitStop {
    pub name: String,
    pub location: Location
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct TransitAgency {
    pub name: String,
    pub url: String,
    pub phone: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Line {
    pub name: String,
    pub short_name: String,
    pub color: String,
    pub agencies: Vec<TransitAgency>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct TransitDetails {
    pub arrival_stop: TransitStop,
    pub depature_stop: TransitStop,
    pub arrival_time: Time,
    pub depature_time: Time,
    pub headsign: String,
    pub headway: usize,
    pub num_stops: usize,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Step {
    pub html_instructions: String,
    pub distance: Distance,
    pub duration: Duration,
    pub start_location: Location,
    pub end_location: Location,
    pub polyline: Polyline,
    pub steps: Option<Vec<Step>>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Leg {
    pub steps: Vec<Step>,
    pub distance: Distance,
    pub duration: Duration,
    pub duration_in_traffic: Option<DurationInTraffic>,
    pub arrival_time: Option<Time>,
    pub depature_time: Option<Time>,
    pub start_location: Location,
    pub end_location: Location,
    pub start_address: String,
    pub end_address: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Route {
    pub summary: String,
    pub legs: Vec<Leg>,
    pub waypoint_order: Vec<usize>,
    pub overview_polyline: Polyline,
    pub bounds: BoundBox,
    pub copyrights: String,
    pub warnings: Vec<String>,
    pub fare: Option<Fare>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct RouteResult {
    pub status: String,
    pub geocoded_waypoints: Vec<GeoCodedWaypoint>,
    pub routes: Vec<Route>,
}
