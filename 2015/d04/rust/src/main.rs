use md5;

fn main() {
    p1();
    p2();
}

fn p1() {
    let ans = mine_coin("bgvyzdsv", "00000");
    println!("{}", ans);
}

fn p2() {
    let ans = mine_coin("bgvyzdsv", "000000");
    println!("{}", ans);
}

fn mine_coin(key: &str, prefix: &str) -> i32 {
    let mut i = 0;
    loop {
        let mut coin = key.to_string();
        coin.push_str(i.to_string().as_str());
        let digest = md5::compute(coin.as_bytes());
        let hash = format!("{:x}", digest);
        if hash[0..prefix.len()].eq(prefix) {
            break;
        }
        i += 1;
    }
    return i;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(609043, mine_coin("abcdef", "00000"));
        assert_eq!(1048970, mine_coin("pqrstuv", "00000"));
    }
}
