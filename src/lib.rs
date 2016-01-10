extern crate rustc_serialize;
extern crate hyper;
extern crate mockito;

pub mod client;
pub mod board;
pub mod list;
pub mod card;
pub mod label;

pub type Client = client::Client;
pub type Board = board::Board;
pub type List = list::List;
pub type Card = card::Card;
pub type Label = label::Label;

#[derive(PartialEq, Debug)]
pub enum Error {
    NotFound,
    Unauthorized,
    TooManyRequests,
    InvalidRequest(String),
    Json(String),
    Unknown(String)
}
