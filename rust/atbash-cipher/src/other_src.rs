use std::iter::once;

fn get_iterator(input: &str) -> impl Iterator<Item = char> + '_ {
    input.chars()
}

fn encode(input: &str) -> String {
    get_iterator(input)
        .enumerate()
        .flat_map(|(idx, c)| {
            if idx % 5 == 0 && idx > 0 {
                Some(' ')
            } else {
                None
            }
            .into_iter()
            .chain(once(c))
        })
        .collect()
}

fn main() {
    println!("{}", encode("helloworldthisisalongstring"));
}
