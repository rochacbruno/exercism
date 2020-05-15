use std::char::from_digit;

const MINE: char = '*';

fn count_mines(char_index: usize, row_index: usize, minefield: &[&str]) -> char {
    let max_row_index = minefield.len() - 1;
    let max_char_index = minefield[row_index].len() - 1;

    let mut counter = 0;

    for y in &minefield[row_index.saturating_sub(1)..=max_row_index.min(row_index + 1)] {
        for x in char_index.saturating_sub(1)..=max_char_index.min(char_index + 1) {
            if y.as_bytes()[x] == MINE as u8 {
                counter += 1;
            }
        }
    }

    match counter {
        0 => ' ',
        _ => from_digit(counter, 10).unwrap(),
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.chars()
                .enumerate()
                .map(|(char_index, c)| match c {
                    MINE => MINE,
                    _ => count_mines(char_index, row_index, minefield),
                })
                .collect::<String>()
        })
        .collect()
}
