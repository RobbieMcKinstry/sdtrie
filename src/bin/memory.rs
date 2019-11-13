use sdtrie::dtrie::RadixTrie;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let mut dlb = RadixTrie::new();
    let dictionary_path = "dictionaries/alphanumeric.txt";
    println!("Opening file…");
    let f = File::open(dictionary_path).unwrap();
    let file = BufReader::new(&f);
    let mut words = Vec::with_capacity(194433);
    println!("Reading dictionary {}", dictionary_path);
    let mut letter_count = 0;
    for line in file.lines() {
        let content = line.unwrap().trim().to_owned();
        letter_count += content.len();
        words.push(content);
    }
    let mut tokens = Vec::with_capacity(194433);
    for val in words.iter() {
        let token = dlb.get_or_intern(val.to_string());
        tokens.push(token);
    }
    let byte_count = dlb.size_of();

    println!("Total file size:\t{} bytes", letter_count);
    println!("Size of trie:\t\t{} bytes", byte_count);
}
