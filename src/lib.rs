use std::str::FromStr;
use std::fmt::Debug;


pub fn lines_to_vec<T>(data: &str) -> Vec<T>
    where T: FromStr,
          <T as FromStr>::Err: Debug,
{
    data.lines().map(|s| s.parse().unwrap()).collect()
}


// TODO: Replace with generic version.
pub fn str_to_str_isize_vec(data: &str) -> Vec<(&str, isize)> {
    data.lines().map(|s| {
        let mut str_isize = s.split(' ');
        (str_isize.next().unwrap(), str_isize.next().unwrap().parse().unwrap())
    }).collect()
}


pub fn comma_separated_to_vec<T>(data: &str) -> Vec<T>
    where T: FromStr,
          <T as FromStr>::Err: Debug,
{
    data.trim().split(',').map(|s| s.parse().unwrap()).collect()
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


pub mod math {
    pub fn tri_f64(value: f64) -> f64 {
        value * (value + 1.0) / 2.0
    }

    pub fn tri_base_f64(value: f64, rows: f64) -> f64 {
        tri_f64(value) - tri_f64(value - rows)
    }

    pub fn tri_isize(value: isize) -> isize {
        value * (value + 1) / 2
    }

    pub fn tri_base_isize(value: isize, rows: isize) -> isize {
        tri_isize(value) - tri_isize(value - rows)
    }

    pub fn tri_inv(value: f64) -> f64 {
        (2.0 * value + 1.0/4.0).sqrt() - 1.0/2.0
    }
}