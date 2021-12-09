use std::str::FromStr;
use std::fmt::Debug;


pub fn lines_to_vec<T>(data: &str) -> Vec<T>
    where T: FromStr,
          <T as FromStr>::Err: Debug,
{
    data.trim().split("\n").map(|s| s.parse().unwrap()).collect()
}


// TODO: Replace with generic version.
pub fn str_to_str_isize_vec(data: &str) -> Vec<(&str, isize)> {
    data.trim().split("\n").map(|s| {
        let mut str_isize = s.split(" ");
        (str_isize.next().unwrap(), str_isize.next().unwrap().parse().unwrap())
    }).collect()
}


pub fn comma_separated_to_vec<T>(data: &str) -> Vec<T>
    where T: FromStr,
          <T as FromStr>::Err: Debug,
{
    data.trim().split(",").map(|s| s.parse().unwrap()).collect()
}


pub fn grid<T>(data: &str) -> Vec<Vec<T>>
    where T: FromStr,
          <T as FromStr>::Err: Debug,
{
    data
    .lines()
    .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
    .collect()
}