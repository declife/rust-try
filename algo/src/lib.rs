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

    fn add_binary_op(a: String, b: String) -> String {
        let mut carry = 0;
        let mut ans = Vec::new();
        let alen = a.len();
        let blen = b.len();
        let a_vec = a.into_bytes();
        let b_vec = b.into_bytes();
        let (prefix, a_suffix, b_suffix) = if alen > blen {
            let len = alen - blen;
            (&a_vec[..len], &a_vec[len..], &b_vec[..])
        } else {
            let len = blen - alen;
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
}
