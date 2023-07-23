fn parity(x: u32) -> u32 {
    let mut y = x;
    y ^= x >> 16;
    y ^= y >> 8;
    y ^= y >> 4;
    y ^= y >> 2;
    y ^= y >> 1;
    y & 1
}

pub fn round_function(x: u32, _key: u32, _flavor: u8) -> u32 {
    x
}

pub fn feistel(x: u64, left_key: u32, right_key: u32, l_arg: u8, r_arg: u8) -> u64 {
    let mut left: u32 = (x >> 32) as u32;
    let mut right = x as u32;

    left ^= round_function(right, left_key as u32, l_arg);
    right ^= round_function(left, right_key as u32, r_arg);

    ((left as u64) << 32) | (right as u64)
}

pub fn expand_00(key: u64, protocol: u8) -> [u32; 4] {
    let mut exp00: [u32; 4] = [0, 0, 0, 0];
    exp00[0] = (key >> 32) as u32;
    exp00[1] = key as u32;
    exp00[2] = 0x08090a0b;
    exp00[3] = 0x0c0d0e0f;

    let mut chain: u32 = if (protocol & 0x0c) != 0 {
        0x84e5c4e7
    } else {
        0x6aa32b6f
    };
    for i in 0..8 {
        chain = round_function(exp00[i & 3], chain, 0);
        exp00[i & 3] = chain;
    }
    exp00
}

#[cfg(test)]
mod tests {
    use super::{parity, round_function};

    #[test]
    fn calc_parity1() {
        let x: u32 = 0xF0F0F0F0;
        assert_eq!(parity(x), 0);
    }
    #[test]
    fn calc_parity2() {
        let x: u32 = 0x38B80801;
        assert_eq!(parity(x), 1);
    }
    #[test]
    fn round1() {
        let p = round_function(1010, 0, 0);
        println!("{}", p);
        assert_eq!(p, 978197038);
    }
    #[test]
    fn round2() {
        let p = round_function(99999999, 99999999, 1);
        println!("{}", p);
        assert_eq!(p, 1922248979);
    }
}
