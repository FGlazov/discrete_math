use std::collections::HashSet;

const ORDER: usize = 3;

fn main() {
    generate_all_reduced_latin_squares();

//    let first_entries = [[1, 2, 4], [2, 4, 1], [4, 1, 2]];
//    let second_entries = [[1, 2, 4], [4, 1, 2], [2, 4, 1]];
//
//    let first_latin_square = LatinSquare { entries: first_entries };
//    let second_latin_square = LatinSquare { entries: second_entries };
//
//    println!("{}", first_latin_square.orthogonal_to(second_latin_square));
}

fn generate_all_reduced_latin_squares() -> HashSet<LatinSquare> {
    // We start with the Square

    //  [  1      2 4 ... 2^ORDER]
    //  [  2      0 0 ...    0   ]
    //  [  4      0 0 ...    0   ]
    //  ..........................
    //  [ 2^ORDER 0 0  ... 0     ]

    // And then recursively fill in all possible values for the 0s.

    let mut base = [[0; ORDER]; ORDER];
    for i in 0..ORDER {
        base[0][i] = 1 << (i as u16);
        base[i][0] = 1 << (i as u16);
    }

    println!("{:?}", get_next_allowed(&base));


    HashSet::new()
}

// Returns AllowedValues if there is a 0 inside the square and it can take values.
// Returns Err(True) if there are no 0s remaining - the square is already a latin square.
// Returns Err(False) if there is a 0 remaining, but it can take no value,
//                    the square can not be extended to a latin square.
fn get_next_allowed(current_square: &[[u16; ORDER]; ORDER]) -> Result<AllowedValues, bool> {
    match scan_for_first_zero(current_square) {
        None => { return Err(true); }
        Some((x, y)) => {
            // Assume all are allowed to begin with
            let mut allowed_values = (1 << (ORDER)) - 1;
            
            // And then remove all the values already taken in that row and column
            for i in 0..ORDER {
                allowed_values = allowed_values & !current_square[x][i];
                allowed_values = allowed_values & !current_square[i][y];
            }

            if allowed_values == 0 {
                return Err(false);
            } else {
                return Ok(AllowedValues { x, y, allowed: allowed_values });
            }
        }
    }
}

// Scan by x then and then y coordinates. Find the first 0.
fn scan_for_first_zero(current_square: &[[u16; ORDER]; ORDER]) -> Option<(usize, usize)> {
    for i in 1..ORDER {
        for j in 1..ORDER {
            if current_square[i][j] == 0 {
                return Option::from((i, j));
            }
        }
    }
    Option::None
}

#[derive(Debug)]
struct AllowedValues {
    x: usize,
    y: usize,
    allowed: u16,
}


#[derive(Eq, PartialEq, Hash)]
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