use serde::Deserialize;

#[derive(Deserialize)]
pub struct CongruentQuery {
    pub n1: u32,
    pub n2: u32,
    pub modulo: u32,
}
