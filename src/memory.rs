pub mod memory {
    // import modules
    use std::collections::HashMap;

    // memory struct, contains segments of memory made using a HashMap
    pub struct Memory {
        // each segment contains a key and a vec of words
        segments: HashMap<u32, Vec<u32>>,
        next_key: u32,
    }

    // memory functions
    impl Memory {
        // create new memory segments
        pub fn new() -> Self {
            Memory {
                segments: HashMap::new(),
                next_key: 0,
            }
        }

        // map segment
        pub fn map_segment(&mut self, words: Vec<u32>) -> u32 {
            // get the key
            let segment_key = self.next_key;
            // update key counter value
            self.next_key += 1;
            // insert segment into segments hashmap
            self.segments.insert(segment_key, words);
            segment_key
        }

        // unmap segment
        pub fn unmap_segment(&mut self, segment_key: u32) {
            // remove segment from hashmap
            self.segments.remove(&segment_key);
        }

        // load individual word
        pub fn load_word(&self, segment_key: u32, offset: usize) -> u32 {
            // get the word from segment hashmap's offset
            self.segments.get(&segment_key).unwrap()[offset]
        }

        // store individual word
        pub fn store_word(&mut self, segment_key: u32, offset: usize, value: u32) {
            // get the word from segment hashmap's offset and store the value
            self.segments.get_mut(&segment_key).unwrap()[offset] = value;
        }
        
        // function that loads a program
        pub fn load_program(&mut self, src_segment_key: u32, dest_segment_key: u32, offset: usize, program_counter: &mut usize,) {
            // if register B = 0, do not clone segment
            if src_segment_key != 0 {
                // duplicate m[reg_B] and replace m[0]
                *self.segments.get_mut(&dest_segment_key).unwrap() = self.segments.get(&src_segment_key).unwrap().clone()
            }
            // set program counter to m[0][reg_C]
            *program_counter = offset;
        }

    }
}