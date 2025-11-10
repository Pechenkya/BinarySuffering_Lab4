const MAX_DICT_SIZE: usize = 0xFFFF;

pub struct LZWCoder {
    dict: Vec<(u8, Option<u16>)>,
    max_dict_size: usize,
    clear_dict_on_overfill: bool,
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
            if self.clear_dict_on_overfill {
                return;
            } else {
                self.dict.resize(256, (0, None));
            }
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

    fn get_last_dict_index(&self) -> u16 {
        self.dict.len() as u16 - 1
    }

    pub fn encode(input: &[u8], clear_dict_on_overfill: bool) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        // Create encoder and initialize dictionary
        let mut internal_encoder = LZWCoder {
            dict: Vec::new(),
            max_dict_size: MAX_DICT_SIZE,
            clear_dict_on_overfill
        };

        // Store parameters for decoder
        output.push(if clear_dict_on_overfill { 1 } else { 0 });
        output.extend_from_slice(&(internal_encoder.max_dict_size as u16).to_le_bytes());

        internal_encoder.set_init_dict();

        let mut S: Vec<u8> = Vec::new();
        let mut I: Option<u16> = None;

        for &byte in input.iter() {
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

    pub fn decode(input: &[u8]) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        // Read first three bytes to restore parameters of encoder
        let clear_dict_on_overfill = input[0] != 0;
        let last_dict_index = u16::from_le_bytes(input[1..3].try_into().unwrap());

        // Create decoder and initialize dictionary
        let mut internal_decoder = LZWCoder {
            dict: Vec::new(),
            max_dict_size: last_dict_index as usize + 1,    // We store only two bytes to ensure the limitation of max 16 bits for code
            clear_dict_on_overfill
        };
        internal_decoder.set_init_dict();

        // Read first idx
        let I = u16::from_le_bytes(input[3..5].try_into().unwrap());
        let mut S: Vec<u8> = Vec::new();

        // First byte should be always in the dict
        if let Some((fb, _)) = internal_decoder.dict.get(I as usize) {
            S.push(*fb);
            output.push(*fb);   // Send it directly to output
        } else {
            panic!("Corrupted input data: first index not in dictionary");
        }

        let mut old_I: u16 = I;

        for chunk in input[5..].chunks(2) {
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

                    // Set I to newly added sequence
                    old_I = internal_decoder.get_last_dict_index();
                }
            }
        }

        return output;
    }
}