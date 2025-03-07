pub mod response;
pub mod request;
use tabled::Tabled;

#[derive(Tabled)]
pub struct Header {
    pub name: String,
    pub value: String,
}
