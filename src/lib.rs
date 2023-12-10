mod p1025;
mod p11;
mod p12;
mod p121;
mod p125;
mod p1266;
mod p13;
mod p1326;
mod p134;
mod p135;
mod p14;
mod p1424;
mod p150;
mod p151;
mod p1662;
mod p167;
mod p169;
mod p17;
mod p18;
mod p189;
mod p191;
mod p2147;
mod p221;
mod p225;
mod p228;
mod p238;
mod p2483;
mod p26;
mod p27;
mod p274;
mod p28;
mod p300;
mod p38;
mod p39;
mod p392;
mod p394;
mod p40;
mod p400;
mod p415;
mod p45;
mod p48;
mod p50;
mod p55;
mod p56;
mod p57;
mod p58;
mod p6;
mod p68;
mod p72;
mod p80;
mod p88;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
