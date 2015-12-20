use hyper::Client;
use hyper::header::{Connection, ContentType};
use hyper::client::response::Response;
use hyper::status::StatusCode;
use rustc_serialize::json;
use rustc_serialize::{Decodable, Encodable};

use Error;

use std::io::Read;
use std::string::ToString;
use std::error::Error as StdError;

const BASE_URL: &'static str = "https://api.trello.com/1/";
const DEFAULT_SCOPE: &'static str = "read,write,account";
const DEFAULT_EXPIRATION: &'static str = "30days";

pub fn authorize_url(app_name: &str, app_key: &str, scope: Option<&str>, expiration: Option<&str>) -> String {
    let mut url = BASE_URL.to_string() + "authorize?response_type=token";
    url = url + "&name=" + app_name;
    url = url + "&key=" + app_key;
    url = url + "&scope=" + scope.unwrap_or(DEFAULT_SCOPE);
    url = url + "&expiration=" + expiration.unwrap_or(DEFAULT_EXPIRATION);

    url
}

pub fn get<D: Decodable>(path: &str, app_key: &str, token: &str) -> Result<D, Error> {
    let url = url_from_path(&path, &app_key, &token);

    let client = Client::new();
    let mut res = client.get(&url)
        .header(Connection::close())
        .send()
        .unwrap();

    handle_response(&mut res)
}

pub fn post<E: Encodable, D: Decodable>(path: &str, obj: E, app_key: &str, token: &str) -> Result<D, Error> {
    let url  = url_from_path(&path, &app_key, &token);
    let body = json::encode(&obj).unwrap();

    let client  = Client::new();
    let mut res = client.post(&url)
        .header(Connection::close())
        .header(ContentType::json())
        .body(&body)
        .send()
        .unwrap();

    handle_response(&mut res)
}

fn url_from_path(path: &str, app_key: &str, token: &str) -> String {
    let mut url = BASE_URL.to_string() + path;
    url = url + "?key=" + app_key;
    url = url + "&token=" + token;

    url
}

fn handle_response<D: Decodable>(response: &mut Response) -> Result<D, Error> {
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    match response.status {
        StatusCode::Ok | StatusCode::Created => handle_json_response(&body),
        StatusCode::Unauthorized             => Err(Error::Unauthorized),
        StatusCode::TooManyRequests          => Err(Error::TooManyRequests),
        StatusCode::BadRequest               => Err(Error::InvalidRequest(body)),
        _                                    => Err(Error::Unknown(body))
    }
}

fn handle_json_response<D: Decodable>(body: &str) -> Result<D, Error> {
    match json::decode(body) {
        Ok(decodable)      => Ok(decodable),
        Err(decoder_error) => Err(Error::Json(decoder_error.description().to_string()))
    }
}
