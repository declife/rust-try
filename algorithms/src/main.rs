use crate::collection::plus_one;
use std::option::Option::Some;
use std::io::ErrorKind;
use std::io;
use std::fs::File;
use std::net::IpAddr;

mod collection;


fn main(){
    // let home = "127.0.0.1".parse();
    // let home: IpAddr = home.unwrap();
    // collection::plus_one(vec![1,0,0]);
    // plus_one(vec![9,9,9]);
    // collection::string_dis();
    // collection::map_foo();
    // let value = Some(5);
    // if let Some(b) = value {
    //     println!("{}", "PPP");
    // }
    // match value {
    //     Some(value) => println!("{}", "Right"),
    //     None => println!("{}", "False")
    // }
    // let f = File::open("hello.txt");
    //
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };
    let mut val = 1;
    {
        let mut val = 2;
        print!("{}", val);
    }
    print!("{}", val);
    let foo = Some(String::new());
    let bar = Some(12);
    generic_type(&foo);
    generic_type(&bar);

}


fn generic_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

/// 冒泡排序
fn sort(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        for j in 0..v.len() - i - 1 {
            if v[j] > v[j + 1] {
                let tmp = v[j];
                v[j] = v[j + 1];
                v[j + 1] = tmp;
            }
        }
    }
    println!("{}", v.len());
}

/// 1的个数
fn hamming_weight(n: u32) -> i32 {
    let mut ans = 0;
    let mut n = n;
    while n > 0 {
        n &= n - 1;
        ans += 1;
    }
    ans
}

/// 数据类型
#[test]
fn data_type() {
    let _ch: char = '₮';

    let big_str = String::from("world");
    if true {
        // let mut big_str_copy = big_str;
        // big_str_copy.push_str(", copy");
        println!("{}", big_str);
    }
    // println!("{}", big_str);//borrowed value is forever?
}

/// 有序数组去重
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() < 2 {
        nums.len() as i32
    } else {
        let mut count = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[count - 1] {
                nums[count] = nums[i];
                count += 1;
            }
        }
        count as i32
    }
}

/// 合并两有序数组
fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if n == 0 {
        return;
    }
    let m: usize = m as usize;
    let n: usize = n as usize;
    for i in 0..nums2.len() {
        nums1[m + i] = nums2[i];
    }
    if m == 0 {
        return;
    }

    for i in m..m + n {
        for j in (1..i + 1).rev() {
            if nums1[j] < nums1[j - 1] {
                let tmp = nums1[j];
                nums1[j] = nums1[j - 1];
                nums1[j - 1] = tmp;
            } else {
                break;
            }
        }
    }
}
fn merge_by_ans(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    // 转为i32, 避免0_usize - 1溢出
    let mut i = m - 1;
    let mut j = n - 1;
    let mut k = nums1.len() as i32 - 1;
    while j >= 0 {
        // 注意i < 0的情况
        if i < 0 || nums1[i as usize] <= nums2[j as usize] {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        } else {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        }
        k -= 1;
    }
}

///查找needle在haystack中的起始位置
fn str_str_by_api(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(index) => index as i32,
        None => -1
    }
}
fn str_str(haystack: String, needle: String) -> i32 {
    println!("{}", haystack);
    println!("{}", needle);
    if needle.is_empty() || haystack.len() < needle.len() {
        0
    } else {
        for index in 0..haystack.len() - needle.len() + 1 {
            println!("{}", index);
            if equal(&haystack[index..index + needle.len()], &needle) {
                return index as i32;
            }
        }
        -1
    }
}
fn equal(ichi: &str, ni: &str) -> bool {
    if ichi.len() != ni.len() {
        false
    } else {
        for ich in 0..ichi.len() {
            if ichi.as_bytes()[ich]!= ni.as_bytes()[ich] {
                return false;
            }
        }
        true
    }

}

/// 查找插入位置
fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left :usize = 0;
    let mut right = nums.len() - 1;
    if nums[right] < target {
        return (right + 1) as i32;
    }
    while left < right {
        let mid = right - (right - left) / 2;
        if nums[mid] == target {
            return mid as i32;
        }
        if nums[mid] > target {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    if nums[left] >= target {
        left as i32
    } else {
        (left + 1) as i32
    }
}
fn search_insert_simplify(nums: Vec<i32>, target: i32) -> i32 {
    let mut left :usize = 0;
    let mut right = nums.len() - 1;
    let mut ans = nums.len();

    while left <= right {
        let mid = (right - left) >> 1 + left;
        if nums[mid] == target {
            return mid as i32
        } else if nums[mid] > target {
            ans = mid;
            right = mid - 1;
        } else {
           left = mid + 1;
        }
    }
    ans as i32
}

/// 第n个外观数列
fn count_and_say(n: i32) -> String {
   if n <= 0 {
        String::from("0")
    } else if n == 1 {
        String::from("1")
    } else {
        let mut ans = String::from("1");
        for _i in 1..n {
            let mut tmp = String::new();
            let mut pre = ans.as_bytes()[0];
            let mut count = 1;
            for ch in 1..ans.len() {
                if ans.as_bytes()[ch] != pre {
                    tmp.push(std::char::from_digit(count as u32, 10).unwrap());
                    tmp.push(pre as char);
                    count = 1;
                    pre = ans.as_bytes()[ch];
                } else {
                    count += 1;
                }
            }
            tmp.push(std::char::from_digit(count as u32, 10).unwrap());
            tmp.push(pre as char);
            ans = tmp;
        }
        ans
    }
}


/// 写过的
#[test]
fn remove_element() {
    let mut nums = vec![4, 5, 6, 10, 12, 90, 12];
    let val = 12;
    let mut count = 0;
    for value in 0..nums.len() {
       if val != nums[value] {
           nums[count] = nums[value];
           count += 1;
       }
    }
    // count as i32
    println!("{}", count)
}

/// 最后一个单词的长度
#[test]
fn length_of_last_word() {
    let mut s = String::from("k     ");

    // s.trim_end();
    let mut last_index = s.len();
    for char in s.chars().rev() {
        if char == ' ' {
            last_index -= 1;
        } else {
            break;
        }
    }

    let str = &s[..last_index];
    if str.len() == 0 {
        println!("{}", 0);
        // 0
    } else {
        let mut count = 0;
        for char in str.chars().rev() {
            if char == ' ' {
                break;
            }
            count += 1;
        }
        println!("{}", count);
        // count
    }
}