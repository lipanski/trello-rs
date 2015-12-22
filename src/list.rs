use Client;
use Error;

#[derive(RustcDecodable, RustcEncodable)]
pub struct List {
    pub id: String,
    pub name: String
}

impl List {
    pub fn list(client: &Client, board_id: &str) -> Result<Vec<Self>, Error> {
        let path = ["boards", board_id, "lists"].join("/");

        client.get(&path)
    }
}
