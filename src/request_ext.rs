
use std::collections::HashMap;
use reqwest::Request;

pub enum HttpMethod {
    GET,
    PUT,
    DELETE,
    POST,
}

impl HttpMethod {
    fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
        }
    }

    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}


pub fn response_to_hashmap(
    method: HttpMethod,
    response: &reqwest::Response,
) -> HashMap<String, String> {
    let mut map_resp: HashMap<String, String> = HashMap::new();
    map_resp.insert("Status".to_string(), response.status().to_string());
    map_resp.insert("Method".to_string(), method.to_string());
    map_resp.insert("URL".to_string(), response.url().to_string());
    for (key, value) in response.headers() {
        map_resp.insert(key.to_string(), value.to_str().unwrap().to_string());
    }
    map_resp
}

pub fn headers_to_hashmap(headers: &reqwest::header::HeaderMap) -> HashMap<String, String> {
    let mut map_headers: HashMap<String, String> = HashMap::new();
    for (key, value) in headers {
        map_headers.insert(key.to_string(), value.to_str().unwrap().to_string());
    }
    map_headers
}

pub fn request_to_hashmap(request: &Request) -> HashMap<String, String> {
    let mut map_req: HashMap<String, String> = HashMap::new();
    map_req.insert("Method".to_string(), request.method().to_string());
    map_req.insert("URL".to_string(), request.url().to_string());
    for (key, value) in request.headers() {
        map_req.insert(key.to_string(), value.to_str().unwrap().to_string());
    }
    map_req
}

