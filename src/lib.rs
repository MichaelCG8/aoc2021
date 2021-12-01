pub fn str_to_isize_vec(data: &str) -> Vec<isize> {
    data.trim().split("\n").map(|s| s.parse().unwrap()).collect()
}
