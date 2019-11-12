use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;
use rand_core::SeedableRng;
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
fn test_aaa_case1() {
    let mut dlb = DLB::new();
    let expected1 = String::from("a");
    let expected2 = String::from("aa");
    let expected3 = String::from("aaa");

    let id1 = dlb.get_or_intern(expected1.clone());
    let id2 = dlb.get_or_intern(expected2.clone());
    let id3 = dlb.get_or_intern(expected3.clone());
    let observed1 = dlb.resolve(id1);
    let observed2 = dlb.resolve(id2);
    let observed3 = dlb.resolve(id3);
    assert_eq!(Some(expected1), observed1);
    assert_eq!(Some(expected2), observed2);
    assert_eq!(Some(expected3), observed3);
}

#[test]
fn test_aaa_case2() {
    let mut dlb = DLB::new();
    let expected1 = String::from("a");
    let expected2 = String::from("aaa");
    let expected3 = String::from("aa");

    let id1 = dlb.get_or_intern(expected1.clone());
    let id2 = dlb.get_or_intern(expected2.clone());
    let id3 = dlb.get_or_intern(expected3.clone());
    let observed1 = dlb.resolve(id1);
    let observed2 = dlb.resolve(id2);
    let observed3 = dlb.resolve(id3);
    assert_eq!(Some(expected1), observed1);
    assert_eq!(Some(expected2), observed2);
    assert_eq!(Some(expected3), observed3);
}

#[test]
fn test_aaa_case3() {
    let mut dlb = DLB::new();
    let expected1 = String::from("aa");
    let expected2 = String::from("a");
    let expected3 = String::from("aaa");

    let id1 = dlb.get_or_intern(expected1.clone());
    let id2 = dlb.get_or_intern(expected2.clone());
    let id3 = dlb.get_or_intern(expected3.clone());
    let observed1 = dlb.resolve(id1);
    let observed2 = dlb.resolve(id2);
    let observed3 = dlb.resolve(id3);
    assert_eq!(Some(expected1), observed1);
    assert_eq!(Some(expected2), observed2);
    assert_eq!(Some(expected3), observed3);
}

#[test]
fn test_aaa_case4() {
    let mut dlb = DLB::new();
    let expected1 = String::from("aa");
    let expected2 = String::from("aaa");
    let expected3 = String::from("a");

    let id1 = dlb.get_or_intern(expected1.clone());
    let id2 = dlb.get_or_intern(expected2.clone());
    let id3 = dlb.get_or_intern(expected3.clone());
    let observed1 = dlb.resolve(id1);
    let observed2 = dlb.resolve(id2);
    let observed3 = dlb.resolve(id3);
    assert_eq!(Some(expected1), observed1);
    assert_eq!(Some(expected2), observed2);
    assert_eq!(Some(expected3), observed3);
}

#[test]
fn test_aaa_case5() {
    let mut dlb = DLB::new();
    let expected1 = String::from("aaa");
    let expected2 = String::from("a");
    let expected3 = String::from("aa");

    let id1 = dlb.get_or_intern(expected1.clone());
    let id2 = dlb.get_or_intern(expected2.clone());
    let id3 = dlb.get_or_intern(expected3.clone());
    let observed1 = dlb.resolve(id1);
    let observed2 = dlb.resolve(id2);
    let observed3 = dlb.resolve(id3);
    assert_eq!(Some(expected1), observed1);
    assert_eq!(Some(expected2), observed2);
    assert_eq!(Some(expected3), observed3);
}

#[test]
fn test_aaa_case6() {
    let mut dlb = DLB::new();
    let expected1 = String::from("aaa");
    let expected2 = String::from("aa");
    let expected3 = String::from("a");

    let id1 = dlb.get_or_intern(expected1.clone());
    let id2 = dlb.get_or_intern(expected2.clone());
    let id3 = dlb.get_or_intern(expected3.clone());
    let observed1 = dlb.resolve(id1);
    let observed2 = dlb.resolve(id2);
    let observed3 = dlb.resolve(id3);
    assert_eq!(Some(expected1), observed1);
    assert_eq!(Some(expected2), observed2);
    assert_eq!(Some(expected3), observed3);
}

#[test]
fn test_reads_dictionary() {
    let mut rng: ChaCha8Rng = SeedableRng::seed_from_u64(100);
    let mut dlb = DLB::new();
    let f = File::open("dictionaries/alphanumeric.txt").unwrap();
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
        // println!("Gen token {}", token);
    }
    tokens.as_mut_slice().shuffle(&mut rng);

    for (idx, token) in tokens.into_iter().enumerate() {
        let expected = Some(words[idx].clone());
        let observed = dlb.resolve(token);
        if expected != observed {
            println!(
                "Found \"{}\" with token {} when looking for \"{}\"",
                observed.clone().unwrap(),
                token,
                expected.clone().unwrap(),
            );
        }
        assert_eq!(expected, observed);
    }
}

#[test]
fn test_contains_dictionary() {
    let mut dlb = DLB::new();
    let f = File::open("dictionaries/alphanumeric.txt").unwrap();
    let file = BufReader::new(&f);
    let mut words = Vec::with_capacity(466551);
    println!("Reading file.");
    for line in file.lines().take(22000) {
        let content = line.unwrap().trim().to_owned();
        words.push(content);
    }

    for val in words.iter() {
        dlb.get_or_intern(val.to_string());
    }

    for (idx, word) in words.iter().enumerate() {
        let expected = true;
        let observed = dlb.contains(word.to_string());
        if expected != observed {
            println!("Failed to recover word \"{}\" on step {}", word, idx);
        }
        assert_eq!(expected, observed);
    }
}
