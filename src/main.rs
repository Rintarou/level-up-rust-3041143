use std::collections::HashMap;
use std::hash::Hash;

/*
fn _unique(a: Vec<i32>) -> Vec<i32> {
    let mut lookup: HashMap<i32, Vec<usize>> = HashMap::new();
    a.iter().enumerate().for_each(|(idx, el)| {
        let vex = lookup.get(el);
        match vex {
            Some(vec) => {
                vec.push(idx);
            }
            None => {
                lookup.insert(*el, vec![]);
            }
        };
    });
    vec![]
}*/

fn unique<T>(a: Vec<T>) -> Vec<T>
where
    T: Hash + Eq + Copy,
{
    let mut lookup: HashMap<T, ()> = HashMap::new();
    a.iter()
        .filter_map(|&x| {
            let contained = lookup.get(&x);
            match contained {
                Some(_) => None,
                None => {
                    lookup.insert(x, ());
                    Some(x)
                }
            }
        })
        .collect()
}
fn _unique(a: Vec<i32>) -> Vec<i32> {
    let mut lookup: HashMap<i32, ()> = HashMap::new();
    a.iter()
        .filter_map(|&x| {
            let contained = lookup.get(&x);
            match contained {
                Some(_) => None,
                None => {
                    lookup.insert(x, ());
                    Some(x)
                }
            }
        })
        .collect()
}

// advanced 1: use generic types
// fn unique(a: Vec<T>) -> Vec<T> {
//     todo!();
// }

// advanced 2: keep items in order
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

// advanced 3: use iterators
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

fn main() {
    let input = vec![2, 1, 1];
    let answer = unique(input);
    println!("unique items -> {:?}", answer)
}

#[test]
fn empty_list() {
    let input: Vec<Hash + Eq + Copy> = vec![];
    let expected_output = vec![];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let mut expected_output = vec![1, 4, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}
