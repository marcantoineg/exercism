use std::{collections::HashMap, sync::mpsc::{self, Sender}, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if worker_count <= 0 {
        panic!("invalid worker count!")
    }

    let (tx, rx) = mpsc::channel::<char>();
    let chunks = input.chunks(input.len() / worker_count).clone();
    let mut handles = vec![];

    for chunk in chunks {
        let txc = tx.clone();
        let handle = thread::spawn(move || {
            process_chunk(chunk, txc);
        });
        handles.push(handle);
    }

    let mut final_map = HashMap::new();

    for c in rx {
        final_map.entry(c)
        .and_modify(|count| { *count += 1 })
        .or_insert(1);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    return final_map;
}

fn process_chunk(chunk: &[&str], tx: Sender<char>) {
    // let mut char_count: HashMap<char, usize> = HashMap::new();

    for str in chunk {
        str.chars().for_each(|c| {
            tx.send(c);
            // char_count.entry(c)
            // .and_modify(|count| { *count += 1 })
            // .or_insert(1);
        });
    }

    // return char_count;
}
