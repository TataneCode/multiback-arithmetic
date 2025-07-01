use serde::Serialize;

#[derive(Serialize)]
pub struct Congruent {
    pub n1_remaining: u32,
    pub n2_remaining: u32,
    pub is_congruent: bool,
}
