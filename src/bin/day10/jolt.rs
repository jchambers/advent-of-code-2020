use std::cmp::max;

pub fn get_deltas(adapters: &mut Vec<u32>) -> [u32; 3] {
    assert!(adapters.len() > 0);

    adapters.sort();

    let mut deltas: [u32; 3] = [0; 3];

    for i in 1..adapters.len() {
        let delta: usize = (adapters[i] - adapters[i - 1] - 1) as usize;

        deltas[delta] += 1;
    }

    // Always count the first adapter as a delta from zero
    deltas[(adapters[0] - 1) as usize] += 1;

    // Last adapter -> device always counts as a delta of 3
    deltas[2] += 1;

    deltas
}

pub fn count_adapter_chains(adapters: &mut Vec<u32>) -> u64 {
    assert!(adapters.len() > 0);

    adapters.sort();

    let mut chains: Vec<u64> = vec![0; (adapters.iter().max().unwrap() + 1) as usize];

    chains[0] = 1;

    for i in 0..adapters.len() {
        let start = max(0, adapters[i] as i64 - 3) as usize;
        chains[adapters[i] as usize] = chains[start..(adapters[i] as usize)].iter().sum();
    }

    chains[chains.len() - 1]
}

#[cfg(test)]
mod test {
    use crate::jolt;

    #[test]
    pub fn get_deltas() {
        {
            let mut adapters: Vec<u32> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
            let deltas = jolt::get_deltas(&mut adapters);

            assert_eq!(7, deltas[0]);
            assert_eq!(5, deltas[2]);
        }

        {
            let mut adapters: Vec<u32> = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
            let deltas = jolt::get_deltas(&mut adapters);

            assert_eq!(22, deltas[0]);
            assert_eq!(10, deltas[2]);
        }
    }

    #[test]
    fn count_adapter_chains() {
        {
            let mut adapters: Vec<u32> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
            assert_eq!(8, jolt::count_adapter_chains(&mut adapters));
        }

        {
            let mut adapters: Vec<u32> = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
            assert_eq!(19208, jolt::count_adapter_chains(&mut adapters));
        }
    }
}