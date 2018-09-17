use std::collections::HashSet;

const ORDER: usize = 6;


// This is a computational "proof" that there is no projective plane of order 6.
// We do this by showing that there are no two pairwise orthogonal latin squares of order 6.
// Our alphabet consists of 1, 2, 4, 8, 16, 32 - in order to be able to make optimizations using bit masks.
fn main() {
    let reduced_latin_squares = generate_all_reduced_latin_squares();

    // We Assume that the first square is in reduced form.
    // And the second is in semi-reduced form (i.e. the first row is 1,2,4,8,16,32 but the column can be anything)
    // This can be assumed w.l.o.g., since otherwise you can rename the symbols s.t. the first row is 1,2,4,8,16,32
    // in both squares, and then permeate the rows of both squares until the first square is in reduced form.

    for reduced_square in &reduced_latin_squares {
        println!("Starting square");
        for semi_reduced_square in &reduced_latin_squares {
            let mut base = semi_reduced_square.entries.clone();

            let found = generate_semi_reduced_and_orth_check(ORDER, &mut base, &reduced_square);
            if found {
                println!("Found orthogonal squares!");
                return;
            }
        }
    }
}

fn generate_semi_reduced_and_orth_check(n: usize, semi_reduced_square: &mut [[u16; ORDER]; ORDER], reduced_square: &LatinSquare) -> bool {
    if n == 1 {
        let latin_semi_reduced = LatinSquare { entries: semi_reduced_square.clone() };
        let result = reduced_square.orthogonal_to(latin_semi_reduced);
        return result;
    } else {
        let mut result = false;
        for i in 0..n - 1 {
            result = result || generate_semi_reduced_and_orth_check(n - 1, semi_reduced_square, reduced_square);
            if n % 2 == 0 {
                let first_value = semi_reduced_square[i];
                let second_value = semi_reduced_square[n-1];

                semi_reduced_square[n-1] = first_value;
                semi_reduced_square[i] = second_value;
            } else {
                let first_value = semi_reduced_square[0];
                let second_value = semi_reduced_square[n-1];

                semi_reduced_square[n-1] = first_value;
                semi_reduced_square[0] = second_value;
            }
        }
        result = result || generate_semi_reduced_and_orth_check(n-1, semi_reduced_square, reduced_square);
        return result;
    }
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

    generate_next_values(base)
}

fn generate_next_values(current_square: [[u16; ORDER]; ORDER]) -> HashSet<LatinSquare> {
    // Find the next 0.
    match get_next_allowed(&current_square) {

        // We found the next 0 and its allowed values.
        Ok(allowed_values) => {
            let mut result = HashSet::new();

            // Use every allowed value, and then start again with the next zero.
            for bit in get_bits(allowed_values.allowed) {
                let mut extended_square = current_square.clone();
                extended_square[allowed_values.x][allowed_values.y] = bit;

                for latin_square in generate_next_values(extended_square) {
                    result.insert(latin_square);
                }
            }

            return result;
        }

        Err(latin_square) => {
            // There are no more 0s - we built a latin square.
            if latin_square {
                let mut result = HashSet::with_capacity(1);
                result.insert(LatinSquare { entries: current_square });
                return result;

                // There is a 0 - but it doesn't have any valid values. It can't be extended to a latin square.
            } else {
                return HashSet::with_capacity(0);
            }
        }
    }
}

fn get_bits(bit_mask: u16) -> HashSet<u16> {
    let mut result = HashSet::with_capacity(ORDER - 1);
    for i in 0..ORDER {
        result.insert((1 << i) & bit_mask);
    }
    result.remove(&0);
    result
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


#[derive(Eq, PartialEq, Hash, Debug)]
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