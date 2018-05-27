use std::fs::File;
use std::io::Write;
use std::io::Read;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::cmp::PartialEq;
const PRIME: i64 = 961748941;

#[derive(Copy, Clone)]
pub struct ModInteger (i64);

impl PartialEq for ModInteger {
    fn eq(self: &ModInteger, other: &ModInteger) -> bool {
        return self.0 == other.0;
    }
}

impl Add for ModInteger {
    type Output = ModInteger;

    fn add(self, other: ModInteger) -> ModInteger {
        ModInteger((self.0 + other.0) % PRIME)
    }
}

impl Sub for ModInteger {
    type Output = ModInteger;
    fn sub(self, other: ModInteger) -> ModInteger {
        ModInteger(
            if self.0 > other.0 { self.0 - other.0 } else { self.0 + PRIME - other.0 }
        )
    }
}

impl Mul for ModInteger {
    type Output = ModInteger;
    fn mul(self, other: ModInteger) -> ModInteger {
        ModInteger(
            (self.0 * other.0) % PRIME
        )
    }
}


fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        writeln!(std::io::stderr(),
                 "Usage: rkgrep PATTERN DOC_FILE")
            .unwrap();
        std::process::exit(1);
    }

    let mut doc_file = File::open(&args[2])?;
    let mut buffer = String::new();

    doc_file.read_to_string(&mut buffer);

    let doc = buffer.into_bytes();
    let pattern = "wor".to_string().into_bytes();
    let (index, count) = match rk_substring_match(&pattern, &doc) {
        (Some(i), c) => (i, c),
        (None, _) => (0, 0)
    };
    println!("FIRST INDEX: {}, COUNT: {}", index, count);
    Ok(())
}

fn naive_substring_match(pattern: &Vec<u8>, doc: &Vec<u8>) -> (Option<usize>, i64) {
    let mut first_match_index: Option<usize> = None;
    let mut match_count = 0;
    for (i, _c) in doc[0..doc.len() - pattern.len()].iter().enumerate() {
        if is_substr(pattern, &doc[i..i + pattern.len()]) {
            if first_match_index == None {
                first_match_index = Some(i);
            }
            match_count = match_count + 1;
        }
    }
    (first_match_index, match_count)
}

fn rk_substring_match(pattern: &[u8], doc: &[u8]) -> (Option<usize>, i64) {
    let mut first_match_index: Option<usize> = None;
    let mut match_count = 0;
    let (pattern_hash, _) = rk_init(&pattern);
    println!("PATTERN HASH: {}", pattern_hash.0);
    let (mut doc_hash, base_to_m) = rk_init(&doc[0..pattern.len()]);
    println!("{}", doc.len() - pattern.len());
    for (i, _c) in doc[0..doc.len() - pattern.len()].iter().enumerate() {
        println!("DOC HASH: {}", doc_hash.0);
        if pattern_hash == doc_hash {
            if first_match_index == None {
                first_match_index = Some(i);
            }
            match_count = match_count + 1;
        }
        doc_hash = rk_next(doc_hash, base_to_m, doc[i], doc[i + pattern.len()]);
    }
    (first_match_index, match_count)
}


fn is_substr(pattern: &[u8], doc: &[u8]) -> bool {
    pattern == doc
}


fn rk_init(s: &[u8]) -> (ModInteger, ModInteger) {
    let mut hash = ModInteger(0);
    let mut base_to_m = ModInteger(1);
    for c in s {
        hash = hash + (base_to_m * ModInteger(*c as i64));
        base_to_m = ModInteger(256) * base_to_m;
    }
    (hash, base_to_m)
}

fn rk_next(
    curr_hash: ModInteger,
    base_to_m: ModInteger,
    leftmost: u8,
    rightmost: u8
) -> ModInteger {
    return ModInteger(rightmost as i64) + (
        (curr_hash * ModInteger(256)) - (ModInteger(leftmost as i64) * base_to_m)
    )
}
