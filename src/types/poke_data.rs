#[derive(Clone)]
pub struct PokeData {
    data: Vec<u8>,
}

impl PokeData {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl ToString for PokeData {
    fn to_string(&self) -> String {
        let mut string = String::with_capacity(self.data.len() + 2);
        string += "0x";
        for datum in &self.data {
            string += &format!("{:0<2X}", datum);
        }
        string
    }
}
