use std::str::FromStr;
use std::fmt::Debug;


pub fn str_to_isize_vec(data: &str) -> Vec<isize> {
    data.trim().split("\n").map(|s| s.parse().unwrap()).collect()
}


pub fn str_to_str_isize_vec(data: &str) -> Vec<(&str, isize)> {
    data.trim().split("\n").map(|s| {
        let mut str_isize = s.split(" ");
        (str_isize.next().unwrap(), str_isize.next().unwrap().parse().unwrap())
    }).collect()
}


pub fn comma_separated_num_to_vec<T>(data: &str) -> Vec<T>
    where T: FromStr,
          <T as FromStr>::Err: Debug
{
    data.trim().split(",").map(|s| s.parse().unwrap()).collect()
}
