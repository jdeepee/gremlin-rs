#[macro_use]
extern crate gremlin_derive;

use gremlin_client::{Map, TryFromGremlinMap};
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Default, TryFromGremlinMap, Debug)]
pub struct Vertice {
    pub property_1: String,
}

fn main() {
    let mut hashmap = HashMap::new();
    hashmap.insert("property_1".to_string(), gremlin_client::GValue::String(String::from("Field 1 value")));
    let map = Map::from(hashmap);
    println!("{:?}", Vertice::try_from_gremlin_map(map.try_into().unwrap()));
}
