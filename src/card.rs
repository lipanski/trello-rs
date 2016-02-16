use Client;
use Error;

#[derive(RustcDecodable, RustcEncodable, Debug)]
#[allow(non_snake_case)]
pub struct Card {
    pub id: Option<String>,
    pub idList: String,
    pub name: String,
    pub desc: Option<String>,
    pub pos: Option<f32>,
    pub url: Option<String>
}

impl Card {
    pub fn list(client: &Client, list_id: &str) -> Result<Vec<Self>, Error> {
        let path = ["lists", list_id, "cards"].join("/");

        client.get(&path)
    }

    pub fn find(client: &Client, card_id: &str) -> Result<Self, Error> {
        let path = ["cards", card_id].join("/");

        client.get(&path)
    }

    pub fn create(client: &Client, list_id: &str, name: &str) -> Result<Self, Error> {
        let card = Card {
            id: None,
            idList: list_id.to_string(),
            name: name.to_string(),
            desc: None,
            pos: None,
            url: None
        };

        client.post("/cards", card)
    }

    pub fn delete(client: &Client, card_id: &str) -> Result<(), Error> {
        let path = ["cards", card_id].join("/");

        client.delete(&path)
    }
}
