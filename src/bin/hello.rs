use std::collections::HashSet;

fn main() {
    let mut vikings = HashSet::new();

    let list1 = vec!["Texts.Language".to_string(), "Texts.Text".to_string(), "Texts.SystemTextCollection".to_string()];
    let list2 = vec!["Texts.SystemText".to_string(), "Texts.Text".to_string()];

    vikings.insert("Test.Text".to_string());
    list1.into_iter().map(|x|  vikings.insert(x)).collect::<HashSet<_>>().into_iter();

    println!("hello there {:?}", vikings);
}