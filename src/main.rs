use std::cmp::Ordering;

fn sort_usernames<T: AsRef<str>>(mut usernames: Vec<T>) {
    todo!()
}

fn case_insensitive_sort<T: AsRef<str>>(a: &T, b: &T) -> Ordering {
    String::cmp(&a.as_ref().to_lowercase(), &b.as_ref().to_lowercase())
}

fn main() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];

    println!("unsorted: {:?}", &users);
    users.sort_by(case_insensitive_sort);
    println!("sorted:   {:?}", &users);
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    users.sort_by(case_insensitive_sort);

    assert_eq!(users, sorted);
}
