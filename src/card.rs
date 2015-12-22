use client;
use Error;

#[derive(RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
pub struct Card {
    pub id: Option<String>,
    pub idList: String,
    pub name: String,
    pub desc: Option<String>,
    pub pos: Option<u64>,
    pub url: Option<String>
}

impl Card {
    pub fn list(app_key: &str, token: &str, list_id: &str) -> Result<Vec<Self>, Error> {
        let path = ["lists", list_id, "cards"].join("/");

        client::get(&path, app_key, token)
    }

    pub fn create(app_key: &str, token: &str, list_id: &str, name: &str) -> Result<Self, Error> {
        let card = Card {
            id: None,
            idList: list_id.to_string(),
            name: name.to_string(),
            desc: None,
            pos: None,
            url: None
        };

        client::post("/cards", card, app_key, token)
    }

    pub fn delete(app_key: &str, token: &str, card_id: &str) -> Result<(), Error> {
        let path = ["cards", card_id].join("/");

        client::delete(&path, app_key, token)
    }
}
