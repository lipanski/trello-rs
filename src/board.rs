use Client;
use Error;

#[derive(RustcDecodable, RustcEncodable, Debug)]
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

#[cfg(all(test, feature = "mocks"))]
mod tests {
    use mockito;
    use Client;
    use Board;
    use Error;

    #[test]
    fn test_boards() {
        mockito::mock("GET", "/1/members/me/boards?key=app_key&token=token").respond_with_file("tests/mocks/list_boards.http");

        let client = Client::new("app_key".to_string(), "token".to_string());

        let boards_result = Board::list(&client);
        assert!(boards_result.is_ok());

        let boards = boards_result.unwrap();
        assert_eq!(1, boards.len());

        let board = boards.get(0).unwrap();
        assert_eq!("trello", board.name);
        assert_eq!("123456789abcdefghijklmno", board.id);

        mockito::reset();
    }
}
