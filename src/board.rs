use Client;
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
    pub fn list(client: &Client) -> Result<Vec<Self>, Error> {
        client.get("members/me/boards")
    }
}

#[cfg(test)]
mod tests {
    use mockito::{self, server};
    use Client;
    use Board;
    use Error;

    #[test]
    fn test_something() {
        server::instance();

        mockito::mock("GET", "/1/members/me/boards?key=app_key&token=token").respond_with("HTTP/1.1 200 OK\n\nHello world");

        let client = Client::new("app_key".to_string(), "token".to_string());
        let boards = Board::list(&client);

        assert!(boards.is_err());
        assert_eq!(boards.err().unwrap(), Error::Json("Hello world".to_string()));

        mockito::reset();
    }
}
