pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
pub fn test(left: u64, right: u64, expected: u64) {
    assert_eq!(add(left, right), expected);
}
