use client;
use Error;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Board {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub pinned: Option<bool>,
    pub starred: Option<bool>,
    pub url: String
}

impl Board {
    pub fn list(app_key: &str, token: &str) -> Result<Vec<Self>, Error> {
        client::get("members/me/boards", app_key, token)
    }
}
