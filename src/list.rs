use trello::client;
use trello::Error;

#[derive(RustcDecodable, RustcEncodable)]
pub struct List {
    pub id: String,
    pub name: String
}

impl List {
    pub fn list(app_key: &str, token: &str, board_id: &str) -> Result<Vec<Self>, Error> {
        let path = ["boards", board_id, "lists"].join("/");

        client::get(&path, app_key, token)
    }
}
