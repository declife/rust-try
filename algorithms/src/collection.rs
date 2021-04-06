use std::collections::HashMap;

/// 集合：string
pub fn string_dis() {
    let mut s = String::new();
    s.push('d');
    let _data = "initial contents";
    let s3 = String::from("lol");
    let s = format!("{}-{}-{}", s, _data, s3);
    println!("{}::{}", String::from("konnichiwa"), s);
}


/// 集合： HashMap
pub fn map_foo() {
   let mut foo_map = HashMap::new();
    foo_map.insert(String::from("foo"), String::from("bar"));
}