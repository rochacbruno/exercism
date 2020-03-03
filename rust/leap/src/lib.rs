pub fn is_leap_year(year: u64) -> bool {
    let year_divisible_by = |factor| year % factor == 0;
    year_divisible_by(4) && !year_divisible_by(100) || year_divisible_by(400)
}
