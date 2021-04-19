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
    foo_map.entry(String::from("hai")).or_insert(String::from("bar"));
    let default = lambda_fn;
    foo_map.entry(String::from("xxx")).or_insert_with(default);
    foo_map.entry(String::from("xxx.key")).or_insert_with_key(lambda_fn_key);
    println!("{:#?}", foo_map);
}

fn lambda_fn() -> String {
    String::from("i32")
}

fn lambda_fn_key(key: &String) -> String {
    // String::from("key:".to_owned() + key)
    format!("{}{}", "key:", key)
}

/// 最大字串和
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = i32::min_value();
    let mut cur = 0;
    for num in &nums {
        cur = cur + num;

        if cur > max {
            max = cur;
        }
        if cur < 0 {
            cur = 0;
        }
    }
    max
}

/// 非负数字加1
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 1;
    let mut digits = Vec::from(digits);
    let len = digits.len();
    for index in (0..digits.len()).rev() {

        let tmp = digits[index]  + carry;
        carry = tmp / 10;
        digits[index] = tmp % 10;
        if carry == 0 {
            break;
        }
    }
    if carry == 1 {
        let mut digits= vec![1];
        println!("{:?}", digits);
        digits.append(&mut vec![0; len]);
        println!("{:?}", digits);
        return digits;
    }
    digits
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn this_test_will_pass() {
    //     let value = prints_and_returns_10(4);
    //     assert_eq!(10, value);
    // }
    //
    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }
}