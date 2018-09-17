use std::collections::HashSet;

const ORDER: usize = 6;

fn main() {
    let mut state = [[0u16; 6]; 6];
}

struct LatinSquare {
    entries: [[u16; ORDER]; ORDER]
}

impl LatinSquare {
    fn orthogonal_to(&self, other : LatinSquare) -> bool {
        let mut set = HashSet::with_capacity(ORDER * ORDER);
        for i in 0..ORDER {
            for j in 0..ORDER {
                let first_entry = self.entries[i][j];
                let second_entry = self.entries[i][j] << 6;
                let to_put = first_entry ^ second_entry;

                let duplicate = set.insert(to_put);
                if duplicate {
                    return false
                }
            }
        }

        true
    }
}