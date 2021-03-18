

fn main() {
    data_type();
    let mut v = vec![0,1,1,1,2,3,4,4,5,5,6,7,7,8];
    remove_duplicates(&mut v);
    sort(&mut v);
    println!("Hello, world!");
}


fn sort(v: &mut Vec<i32>) {

    for i in 0..v.len() {
        let m = i + 1;
        for j in 0..v.len() - i - 1{
            if v[j] > v[j + 1] {
                let tmp = v[j];
                v[j] = v[j + 1];
                v[j + 1] = tmp;
            }
        }
    }
    println!("{}", v.len());
}

fn hamming_weight (n: u32) -> i32 {
    let mut ans = 0;
    let mut n = n;
    while n > 0 {
        n &= n - 1;
        ans += 1;
    }
    ans
}

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
