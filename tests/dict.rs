use sdtrie::dtrie::DLB;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[test]
fn test_reads_dictionary() {
    let mut dlb = DLB::new();
    let f = File::open("tests/words.txt").unwrap();
    let file = BufReader::new(&f);
    let mut words = Vec::with_capacity(466551);
    println!("Reading file.");
    for line in file.lines().take(5) {
        let content = line.unwrap().trim().to_owned();
        words.push(content);
    }
    let mut tokens = Vec::with_capacity(466551);
    for val in words.iter() {
        let token = dlb.get_or_intern(val.to_string());
        tokens.push(token);
        println!("Gen token {}", token);
    }

    for (idx, token) in tokens.into_iter().enumerate() {
        let expected = Some(words[idx].clone());
        let observed = dlb.resolve(token);
        assert_eq!(expected, observed);
        println!("Found \"{}\" with token {}", observed.unwrap(), token);
    }
}
