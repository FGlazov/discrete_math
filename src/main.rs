use std::collections::HashSet;

const ORDER: usize = 3;

fn main() {
    let first_entries = [[1, 2, 4], [2, 4, 1], [4, 1, 2]];
    let second_entries = [[1, 2, 4], [4, 1, 2], [2, 4, 1]];

    let first_latin_square = LatinSquare { entries: first_entries };
    let second_latin_square = LatinSquare { entries: second_entries };

    println!("{}", first_latin_square.orthogonal_to(second_latin_square));
}

struct LatinSquare {
    entries: [[u16; ORDER]; ORDER]
}

impl LatinSquare {
    fn orthogonal_to(&self, other: LatinSquare) -> bool {
        let mut set = HashSet::with_capacity(ORDER * ORDER);
        for i in 0..ORDER {
            for j in 0..ORDER {
                let first_entry = self.entries[i][j];
                let second_entry = other.entries[i][j] << ORDER;
                let to_put = first_entry ^ second_entry;
                let duplicate = !set.insert(to_put);
                if duplicate {

                    return false;
                }
            }
        }

        true
    }
}