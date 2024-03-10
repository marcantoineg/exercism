use std::ops::Rem;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let cleaned_code = code.replace(" ", "");
    
    if cleaned_code.len() <= 1 {
        return false
    }

    let mut sum: u32 = 0;

    for (i, c) in cleaned_code.char_indices().rev() {
        let (is_numerical, v) = cast_to_u32(c);
        if !is_numerical {
            return false
        }

        let right_to_left_idx = cleaned_code.len() - i;

        if right_to_left_idx.rem(2) == 0 {
            let mut product = v * 2;

            if product > 9 {
                product -= 9;
            }

            sum += product;
        } else {
            sum += v;
        }
    }


    return sum.rem(10) == 0
}

fn cast_to_u32(c: char) -> (bool, u32) {
    match c.to_digit(10) {
        Some(v) => return (true, v),
        None => return (false, 0),
    }
}
