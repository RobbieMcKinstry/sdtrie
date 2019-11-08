use sdtrie::dtrie::DLB;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[test]
fn test_count_nodes1() {
    let mut dlb = DLB::new();

    vec!["foo", "bar", "pop", "lizz"]
        .into_iter()
        .map(|s| String::from(s))
        .for_each(|s| {
            dlb.get_or_intern(s);
        });
    let count = dlb.count_nodes();
    assert_eq!(count, 4);
}

#[test]
fn test_count_nodes2() {
    let mut dlb = DLB::new();

    vec!["fizz", "fang"]
        .into_iter()
        .map(|s| String::from(s))
        .for_each(|s| {
            dlb.get_or_intern(s);
        });
    let count = dlb.count_nodes();
    assert_eq!(count, 3);
}

#[test]
fn test_count_nodes3() {
    let mut dlb = DLB::new();

    vec!["foo", "fuh", "fang"]
        .into_iter()
        .map(|s| String::from(s))
        .for_each(|s| {
            dlb.get_or_intern(s);
        });
    let count = dlb.count_nodes();
    assert_eq!(count, 4);
}

#[test]
fn test_count_nodes4() {
    let mut dlb = DLB::new();

    vec!["foo", "fuh", "fizz", "fang"]
        .into_iter()
        .map(|s| String::from(s))
        .for_each(|s| {
            dlb.get_or_intern(s);
        });
    let count = dlb.count_nodes();
    assert_eq!(count, 5);
}

#[test]
fn test_broken_case1() {
    let mut dlb = DLB::new();
    let expected1 = String::from("1080");
    let expected2 = String::from("10-point");
    let expected3 = String::from("10th");

    let id1 = dlb.get_or_intern(expected1.clone());
    let id2 = dlb.get_or_intern(expected2.clone());
    let id3 = dlb.get_or_intern(expected3.clone());
    let observed1 = dlb.resolve(id1);
    let observed2 = dlb.resolve(id2);
    let observed3 = dlb.resolve(id3);
    assert_eq!(Some(expected1), observed1);
    assert_eq!(Some(expected2), observed2);
    assert_eq!(Some(expected3), observed3);
    // Expecting:
    // (10) -> (80)
    // (10) -> (-point)
    // (10) -> (th)
    //
    // Observed:
    // (10) -> (thth80)
}

#[test]
fn test_reads_dictionary() {
    let mut dlb = DLB::new();
    let f = File::open("tests/words.txt").unwrap();
    let file = BufReader::new(&f);
    let mut words = Vec::with_capacity(466551);
    println!("Reading file.");
    for line in file.lines().take(22000) {
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
