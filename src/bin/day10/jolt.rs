pub fn get_deltas(jolts: &mut Vec<u32>) -> [u32; 3] {
    assert!(jolts.len() > 0);

    jolts.sort();

    let mut deltas: [u32; 3] = [0; 3];

    for i in 1..jolts.len() {
        let delta: usize = (jolts[i] - jolts[i - 1] - 1) as usize;

        deltas[delta] += 1;
    }

    // Always count the first adapter as a delta from zero
    deltas[(jolts[0] - 1) as usize] += 1;

    // Last adapter -> device always counts as a delta of 3
    deltas[2] += 1;

    deltas
}

#[cfg(test)]
mod test {
    use crate::jolt;

    #[test]
    pub fn get_deltas() {
        {
            let mut jolts: Vec<u32> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
            let deltas = jolt::get_deltas(&mut jolts);

            assert_eq!(7, deltas[0]);
            assert_eq!(5, deltas[2]);
        }

        {
            let mut jolts: Vec<u32> = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
            let deltas = jolt::get_deltas(&mut jolts);

            assert_eq!(22, deltas[0]);
            assert_eq!(10, deltas[2]);
        }
    }
}