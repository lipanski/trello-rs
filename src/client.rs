use hyper::Client as HttpClient;
use hyper::header::{Connection, ContentType};
use hyper::client::response::Response;
use hyper::status::StatusCode;
use rustc_serialize::json;
use rustc_serialize::{Decodable, Encodable};
use mockito::url::Url;

use Error;

use std::io::Read;
use std::string::ToString;

pub const APP_KEY_URL: &'static str = "https://trello.com/app-key";
const BASE_URL: &'static str = "https://api.trello.com/1/";
const DEFAULT_SCOPE: &'static str = "read,write,account";
const DEFAULT_EXPIRATION: &'static str = "30days";

pub struct Client {
    app_key: String,
    token: String
}

impl Client {
    pub fn new(app_key: String, token: String) -> Client {
        Client {
            app_key: app_key,
            token: token
        }
    }

    pub fn authorize_url(app_name: &str, app_key: &str, scope: Option<&str>, expiration: Option<&str>) -> String {
        let mut url = BASE_URL.to_string() + "authorize?response_type=token";
        url = url + "&name=" + app_name;
        url = url + "&key=" + app_key;
        url = url + "&scope=" + scope.unwrap_or(DEFAULT_SCOPE);
        url = url + "&expiration=" + expiration.unwrap_or(DEFAULT_EXPIRATION);

        url
    }

    pub fn get<D: Decodable>(&self, path: &str) -> Result<D, Error> {
        let url = self.url_from_path(&path);

        let client = HttpClient::new();
        let mut res = client.get(Url(&url))
            .header(Connection::close())
            .send()
            .unwrap();

        Self::handle_response(&mut res)
    }

    pub fn post<E: Encodable, D: Decodable>(&self, path: &str, obj: E) -> Result<D, Error> {
        let url  = self.url_from_path(&path);
        let body = json::encode(&obj).unwrap();

        let client  = HttpClient::new();
        let mut res = client.post(&url)
            .header(Connection::close())
            .header(ContentType::json())
            .body(&body)
            .send()
            .unwrap();

        Self::handle_response(&mut res)
    }

    pub fn delete(&self, path: &str) -> Result<(), Error> {
        let url  = self.url_from_path(&path);

        let client  = HttpClient::new();
        let mut res = client.delete(&url)
            .header(Connection::close())
            .header(ContentType::json())
            .send()
            .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        match res.status {
            StatusCode::Ok | StatusCode::Created => Ok(()),
            StatusCode::Unauthorized             => Err(Error::Unauthorized),
            StatusCode::TooManyRequests          => Err(Error::TooManyRequests),
            StatusCode::BadRequest               => Err(Error::InvalidRequest(body)),
            StatusCode::NotFound                 => Err(Error::NotFound),
            _                                    => Err(Error::Unknown(body))
        }
    }

    fn url_from_path(&self, path: &str) -> String {
        let mut url = BASE_URL.to_string() + path;
        url = url + "?key=" + &self.app_key;
        url = url + "&token=" + &self.token;

        url
    }

    fn handle_response<D: Decodable>(response: &mut Response) -> Result<D, Error> {
        let mut body = String::new();
        response.read_to_string(&mut body).unwrap();

        match response.status {
            StatusCode::Ok | StatusCode::Created => Self::handle_json_response(&body),
            StatusCode::Unauthorized             => Err(Error::Unauthorized),
            StatusCode::TooManyRequests          => Err(Error::TooManyRequests),
            StatusCode::BadRequest               => Err(Error::InvalidRequest(body)),
            StatusCode::NotFound                 => Err(Error::NotFound),
            _                                    => Err(Error::Unknown(body))
        }
    }

    fn handle_json_response<D: Decodable>(body: &str) -> Result<D, Error> {
        match json::decode(body) {
            Ok(decodable)      => Ok(decodable),
            Err(_)             => Err(Error::Json(body.to_string()))
        }
    }
}
