pub fn get_loop_size(subject_number: u64, public_key: u64) -> u64 {
    let mut key = 1;
    let mut loop_number = 0;

    while key != public_key {
        key *= subject_number;
        key %= 20201227;
        loop_number += 1;
    }

    loop_number
}

pub fn get_encryption_key(public_key: u64, loop_size: u64) -> u64 {
    let mut encryption_key = 1;

    for _ in 0..loop_size {
        encryption_key *= public_key;
        encryption_key %= 20201227;
    }

    encryption_key
}

#[cfg(test)]
mod test {
    use crate::door;

    #[test]
    pub fn get_loop_size() {
        assert_eq!(8, door::get_loop_size(7, 5764801));
        assert_eq!(11, door::get_loop_size(7, 17807724));
    }

    #[test]
    pub fn get_encryption_key() {
        assert_eq!(14897079, door::get_encryption_key(17807724, 8));
        assert_eq!(14897079, door::get_encryption_key(5764801, 11));
    }
}
