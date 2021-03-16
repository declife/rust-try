

fn main() {
    let mut v = vec![488,43,564,23];
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
