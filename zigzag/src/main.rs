impl Solution {
    // I could use a Vec<Vec<char>> approach but I'm feeling spicy today.
    // Given the String is just being remapped to a sawtooth shape, the placement of the characters can be determined by a mathematical function and moved from there instead of having to do expensive mapping and remapping.
    // Suffice to say this is gonna use modulos out the wazoo
    pub fn convert(s: String, num_rows: i32) -> String {
        // The sawtooth pattern means y axis placement is dictated my the modulo of `2r-2`, where `r` is the number of desired rows.
        // When operated with, the remainder determines its effective placement in the y dimension (without a matrix).
        // For `a % b = c` where , `a` is the index of the current char and `b` is the below divisor,

        // where `c < r`, where `r` is the number of desired rows (or `(b + 2)/2`)...
        // ...`c` is the same as its effective y axis position...
        // ...and its x axis position is equal to `(a / b) * b / 2` using integer division, truncating down.
        // Note that (b / 2) will never round as it will always be even.

        // Else where `r <= c`, the effective y axis position is equal to `r-(c % r + 2)`...
        // ...and its x axis position is equal to `((a / b) * b / 2)+((c + 1) % b)`
        let divisor = (num_rows as usize * 2) - 2;    

        // Stores the new coordinates for each old index.
        let mut new_coords: Vec<(usize, [usize; 2])> = Vec::new();

        for i in 0..s.len() {
            new_coords.push((
                i,
                get_new_coords(i, divisor, num_rows as usize)
            ));
        }

        // Now new_coords has the new coordinates for each letter
        new_coords.sort_by(|a, b| {
            if a.1[1] == b.1[1] { // if y is the same
                a.1[0].cmp(&b.1[0])
            } else {
                a.1[1].cmp(&b.1[1])
            }
        });

        let mut res = String::new();

        for i_coords in new_coords {
            match s.chars().nth(i_coords.0) {
                Some(c) => {res.push(c)},
                _ => {}
            }
        }

        res
    }
}

fn get_new_coords(a: usize, b: usize, r: usize) -> [usize; 2]{
    let c: usize = a % b;

    if c < r {
        return[(r * (a / b)), c];
    } else {
        return[((r * (a / b)) + ((c + 1) % b)), (r - ((c % r) + 2))];
    }
}
