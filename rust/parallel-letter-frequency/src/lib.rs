use std::collections::HashMap;
use std::thread;

fn do_the_count(input: &[String]) -> HashMap<char, usize> {
    let mut counter = HashMap::new();

    for phrase in input.iter() {
        for c in phrase.chars().filter(|l| l.is_alphabetic()) {
            let c_count = counter.entry(c.to_ascii_lowercase()).or_insert(0);
            *c_count += 1;
        }
    }
    counter
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut counters = HashMap::new();
    if input.is_empty() {
        return counters;
    }

    let mut handlers = vec![];
    let chunk_size = input.len() / worker_count + 1;

    for chunk in input.chunks(chunk_size) {
        let temp_data = chunk
            .iter()
            .map(|s| (*s).to_string())
            .collect::<Vec<String>>();
        let handler = thread::spawn(move || do_the_count(&temp_data));
        handlers.push(handler);
    }

    for handler in handlers {
        for (c, count) in handler.join().unwrap() {
            let c_count = counters.entry(c).or_insert(0);
            *c_count += count;
        }
    }

    counters
}
