#[cfg(test)]
mod tests {

    use std::char;


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn add_binary_test() {

        let a = "100".to_string();
        let b = "110010".to_string();
        // let ans = add_binary(a, b);
        // assert_eq!(ans, "11010".to_string());
        let bb = add_binary_op(a, b);
        assert_eq!(bb, "110110".to_string());
    }

    /// 字符串过长后会有问题
    fn add_binary(a: String, b: String) -> String {

        format!("{:b}", u32::from_str_radix(&a, 2).unwrap() + u32::from_str_radix(&b, 2).unwrap())
    }

    /// 优化版本
    fn add_binary_op(a: String, b: String) -> String {
        let mut carry = 0;
        let mut ans = Vec::new();
        // let a_vec.len() = a.len();
        // let b_vec.len() = b.len();
        let a_vec = a.into_bytes();
        let b_vec = b.into_bytes();
        let (prefix, a_suffix, b_suffix) = if a_vec.len() > b_vec.len() {
            let len = a_vec.len() - b_vec.len();
            (&a_vec[..len], &a_vec[len..], &b_vec[..])
        } else {
            let len = b_vec.len() - a_vec.len();
            (&b_vec[..len], &a_vec[..], &b_vec[len..])
        };
        for index in (0..a_suffix.len()).rev() {
            let cur = a_suffix[index] as u32 - 48 + b_suffix[index] as u32 - 48 + carry;
            carry = cur >> 1;
            let out = char::from_digit(cur & 1, 2).unwrap();
            ans.push(zero_or_one(cur & 1));
        }
        for index in (0..prefix.len()).rev() {
            if carry == 0 {
                // for ch in prefix[..=index].iter().rev() {
                //     ans.push(zero_or_one(*ch as u32 - 48));
                // }
                prefix[..=index].iter().rev().for_each(|ch| ans.push(zero_or_one(*ch as u32 - 48))); //看看这个是啥
                // ans = ans + prefix[..=index].iter().rev().collect::<String>();
                // ans = format!("{}{}", &ans, prefix[..=index].iter().rev().collect::<String>());
                break;
            }
            let cur = (prefix[index] as u32 - 48) + carry;
            carry = cur >> 1;
            ans.push(zero_or_one(cur & 1));
        }
        if carry == 1 {
            ans.push('1');
        }
        ans.into_iter().rev().collect()
    }

    fn zero_or_one(n: u32) -> char {
        if n == 1 {
            '1'
        } else {
            '0'
        }
    }

    /// 平方根
    #[test]
    fn my_sqrt_test() {
        assert_eq!(my_sqrt(2147483647), 46340)
    }

    fn my_sqrt(x: i32) -> i32 {
        if x < 0 {
            -1
        } else {
            let mut l = 0;
            let mut r = 46340;

            while l < r {
                let mid = (r - l) / 2 + l;
                let mid_sqr = mid * mid;
                if mid_sqr == x {
                    return mid;
                }
                if mid_sqr < x {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
            if l * l > x {
                l - 1
            } else {
                l
            }
            // for i in 0..=x {
            //     if  i * i == x {
            //         return i;
            //     } else if i * i > x {
            //         return i - 1;
            //     }
            // }
            // -1
        }
    }

    /// 牛顿迭代: xi = 0.5(x0+c/x0)...
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0{
            return 0i32;
        }

        let c = x as f64;
        let mut  x0 = c;
        loop{
            let xi = 0.5 * (x0 + c/x0);
            if (x0 - xi) < 1e-7{
                break;
            }
            x0 = xi;
        }

        x0 as i32
    }



}