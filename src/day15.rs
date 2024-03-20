pub type HashResult = u8;

pub fn hash(label: &str) -> HashResult {
    label.chars().fold(0, |acc, x| {
        (acc.overflowing_add(x as HashResult).0)
            .overflowing_mul(17)
            .0
    })
    //step.chars().fold(0, |acc, x| ((acc + x as u64) * 17) % 256)
}
