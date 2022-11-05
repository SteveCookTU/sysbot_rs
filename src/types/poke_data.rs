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
            string += &format!("{:0>2X}", datum);
        }
        string
    }
}

#[cfg(test)]
mod test {
    use crate::types::PokeData;

    #[test]
    fn should_print_be() {
        let data = PokeData::new(0x12345678_u32.to_be_bytes().to_vec());
        assert_eq!("0x12345678".to_string(), data.to_string())
    }

    #[test]
    fn should_print_le() {
        let data = PokeData::new(0x12345678_u32.to_le_bytes().to_vec());
        assert_eq!("0x78563412".to_string(), data.to_string())
    }
}
