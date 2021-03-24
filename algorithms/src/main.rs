fn main() {
    for i in 0..-1 {
        println!("{}", i);
    }
    let n = str_str(String::from("refgll"), String::from("llaaaaaaa"));
    data_type();
    let mut v = vec![0, 1, 1, 1, 2, 3, 4, 4, 5, 5, 6, 7, 7, 8];
    remove_duplicates(&mut v);
    sort(&mut v);
    println!("Hello, world!");
}

/// 冒泡排序
fn sort(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        let m = i + 1;
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
fn data_type() {
    let _ch: char = '₮';
    let str = "hello";

    let big_str = String::from("world");
    if true {
        let mut big_str_copy = big_str;
        big_str_copy.push_str(", copy");
        //println!("{}", big_str);
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
