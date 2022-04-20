use crate::types::poke_data::PokeData;

#[derive(Clone)]
pub struct PokeArgs {
    pub addr: u64,
    pub data: PokeData,
}
