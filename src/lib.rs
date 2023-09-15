mod p121;
mod p1326;
mod p169;
mod p189;
mod p225;
mod p228;
mod p2483;
mod p26;
mod p27;
mod p38;
mod p39;
mod p40;
mod p45;
mod p55;
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
