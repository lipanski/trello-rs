use Client;
use Error;

#[derive(RustcDecodable, RustcEncodable, Debug)]
#[allow(non_snake_case)]
pub struct Label {
    id: String,
    idBoard: String,
    name: String,
    color: String,
    uses: u32
}

impl Label {
    pub fn list(client: &Client, board_id: &str) -> Result<Vec<Self>, Error> {
        let path = ["boards", board_id, "labels"].join("/");

        client.get(&path)
    }
}
