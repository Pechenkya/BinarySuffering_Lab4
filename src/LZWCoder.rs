const MAX_DICT_SIZE: usize = 0xFFFF;

struct LZWCoder {
    dict: Vec<(u8, Option<u16>)>,
    max_dict_size: usize,
}

impl LZWCoder {
    fn set_init_dict(&mut self) {
        self.dict.clear();
        for i in 0..256 {
            self.dict.push((i as u8, None));
        }
    }

    fn find_seq_in_dict(&self, (char, idx): (u8, Option<u16>)) -> Option<u16> {
        if let Some(idx) = self.dict.iter().position(|&entry| entry == (char, idx)) {
            Some(idx as u16)
        } else {
            None
        }
    }

    fn add_seq_to_dict(&mut self, (char, idx): (u8, Option<u16>)) {
        if self.dict.len() > self.max_dict_size {
            self.dict.resize(256, (0, None));
        }
        
        self.dict.push((char, idx));
    }

    pub fn encode(input: &[u8]) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        // Create encoder and initialize dictionary
        let mut internal_encoder = LZWCoder {
            dict: Vec::new(),
            max_dict_size: MAX_DICT_SIZE,
        };
        internal_encoder.set_init_dict();

        let mut S: Vec<u8> = Vec::new();
        let mut I: Option<u16> = None;

        for &byte in input.iter() {
            S.push(byte);
            if let Some(idx) = internal_encoder.find_seq_in_dict((byte, I)) {
                I = Some(idx);
                S.push(byte);
            } else {
                output.extend_from_slice(&I.unwrap().to_le_bytes());
                internal_encoder.add_seq_to_dict((byte, I));

                S.clear();
                S.push(byte);
                I = Some(byte as u16);  // I -> idx of byte (bytes are filled sequentially)
            }
        }

        output.extend_from_slice(&I.unwrap().to_le_bytes());

        return output;
    }
}