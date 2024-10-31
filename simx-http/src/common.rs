use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::{Map, Value};
use std::str::FromStr;

pub fn hashmap_to_headerMap(map: &Map<String, Value>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    // 将 HashMap 转换为 HeaderMap
    for (key, value) in map.iter() {
        headers.insert(HeaderName::from_str(key.as_str()).unwrap(), HeaderValue::from_str(value.as_str().unwrap()).unwrap());
    }
    headers
}