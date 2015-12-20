extern crate rustc_serialize;
extern crate hyper;

pub mod client;
pub mod board;
pub mod list;
pub mod card;

pub type Board = board::Board;
pub type List = list::List;
pub type Card = card::Card;

pub enum Error {
    Unauthorized,
    TooManyRequests,
    InvalidRequest(String),
    Json(String),
    Unknown(String)
}
