#[allow(unused_imports)]
use std::io;

#[allow(unused_macros)]
macro_rules! get {
    ($i:ident) => {
        let mut $i = String::new();
        io::stdin().read_line(&mut $i).ok();
    };
}

#[allow(unused_macros)]
macro_rules! to_vec {
    ($s:ident, $t:ty) => {
        $s.split_whitespace().map(|i| i.parse().ok().unwrap()).collect::<Vec<$t>>()
    }
}

#[allow(unused_macros)]
macro_rules! parse {
    ($s:ident, $t:ty) => {
        $s.trim().parse::<$t>().ok().unwrap()
    }
}

#[allow(unused_macros)]
macro_rules! max {
    ($v:ident) => {
        $v.iter().max().unwrap()
    };
}

#[allow(unused_mut)]
fn main() {

}
