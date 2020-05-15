/// Check a Luhn checksum.
pub fn is_valid(keycode: &str) -> bool {
    keycode
        .chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(i, acc), c| {
            c.to_digit(10).map(|mut d| { 
                if i % 2 != 0 {
                    d *= 2;
                    if d > 9 {
                        d -= 9;
                    }
                }
                (i + 1, acc + d)
            })
        })
        .map_or(false, |(i, acc)| i > 1 && acc % 10 == 0)
}
