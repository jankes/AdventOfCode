fn main() {
    let mut mem = Memory::new_with_values(&[4u16, 10, 4, 1, 8, 4, 9, 14, 5, 1, 14, 15, 0, 15, 3, 5]);    
    let mut seen_configs = Vec::<Memory>::new();
    seen_configs.push(mem);

    mem = Memory::redistribute(&mem);    
    let mut count = 1;
    for part in 1..=2 {
        while !seen_configs.contains(&mem) {
            seen_configs.push(mem);
            mem = Memory::redistribute(&mem);
            count += 1;
        }
        println!("part {}: {} redistribution cyles", part, count);
        seen_configs.clear();
        seen_configs.push(mem);
        mem = Memory::redistribute(&mem);
        count = 1;
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Memory {
    banks: [u16; 16]
}

impl Memory {
    fn new_with_values(values: &[u16; 16]) -> Memory {
        Memory {
            banks: *values
        }
    }

    fn redistribute(mem: &Memory) -> Memory {
        let mut updated = mem.clone();
        let mut index = updated.most_full_bank_index();
        let mut blocks = updated.banks[index];
        updated.banks[index] = 0;
        while blocks > 0 {
            index += 1;
            if index >= updated.banks.len() {
                index = 0;
            }
            updated.banks[index] += 1;
            blocks -= 1;
        }
        updated
    }

    fn most_full_bank_index(&self) -> usize {
        let mut max_count = 0u16;
        let mut max_index = 0usize;
        for (i, &block_count) in self.banks.iter().enumerate() {
            if block_count > max_count {
                max_count = block_count;
                max_index = i;
            }
        }
        max_index
    }
}