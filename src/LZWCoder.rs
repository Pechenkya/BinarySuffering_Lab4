const MAX_DICT_SIZE: usize = 0xFFFF;

pub struct LZWCoder {
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

    fn recover_seq_from_dict(&self, mut idx: u16) -> Option<Vec<u8>> {
        let mut seq: Vec<u8> = Vec::new();

        while let Some((char, next_idx)) = self.dict.get(idx as usize) {
            seq.push(*char);
            if let Some(next_idx) = next_idx {
                idx = *next_idx;
            } else {
                break;
            }
        }

        seq.reverse();

        if seq.is_empty() {
            None
        } else {
            Some(seq)
        }
    }

    pub fn encode(input: &[u8]) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        // Create encoder and initialize dictionary
        let mut internal_encoder = LZWCoder {
            dict: Vec::new(),
            max_dict_size: MAX_DICT_SIZE,
        };
        internal_encoder.set_init_dict();

        // let mut S: Vec<u8> = Vec::new();
        let mut I: Option<u16> = None;

        for &byte in input.iter() {
            // S.push(byte);
            if let Some(idx) = internal_encoder.find_seq_in_dict((byte, I)) {
                I = Some(idx);
                // S.push(byte);
            } else {
                output.extend_from_slice(&I.unwrap().to_le_bytes());
                internal_encoder.add_seq_to_dict((byte, I));

                // S.clear();
                // S.push(byte);
                I = Some(byte as u16);  // I -> idx of byte (bytes are filled sequentially)
            }
        }

        output.extend_from_slice(&I.unwrap().to_le_bytes());

        return output;
    }

    pub fn decode(input: &[u8]) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        // Create decoder and initialize dictionary
        let mut internal_decoder = LZWCoder {
            dict: Vec::new(),
            max_dict_size: MAX_DICT_SIZE,
        };
        internal_decoder.set_init_dict();

        // Read first idx
        let I = u16::from_le_bytes(input[0..2].try_into().unwrap());
        let mut S: Vec<u8> = Vec::new();

        // First byte should be always in the dict
        if let Some((fb, _)) = internal_decoder.dict.get(I as usize) {
            S.push(*fb);
            output.push(*fb);   // Send it directly to output
        } else {
            panic!("Corrupted input data: first index not in dictionary");
        }

        let mut old_I: u16 = I;
        // let mut old_S = S;
        // S = Vec::new();

        for chunk in input[2..].chunks(2) {
            // Read next idx
            let I = u16::from_le_bytes(chunk.try_into().unwrap());
            //                           [chunk[0], chunk[1]];

            if let Some(S) = internal_decoder.recover_seq_from_dict(I) {
                output.extend_from_slice(&S);
                internal_decoder.add_seq_to_dict((S[0], Some(old_I)));
                old_I = I;
            } else {
                // Special case (only case when I is not in dict - covering sequences)
                // S = old_S || old_S[0]
                if let Some(old_S) = internal_decoder.recover_seq_from_dict(old_I) {
                    output.extend_from_slice(&old_S);
                    output.push(old_S[0]);

                    // Add this sequence to the dict
                    internal_decoder.add_seq_to_dict((old_S[0], Some(old_I)));
                }
            }
        }

        return output;
    }
}