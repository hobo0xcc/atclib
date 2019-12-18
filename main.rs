#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::cmp::max;
#[allow(unused_imports)]
use std::cmp::min;

#[allow(unused_macros)]
macro_rules! get {
    ($i:ident) => {
        $i = String::new();
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

#[allow(unused_macros)]
macro_rules! min {
    ($v:ident) => {
        $v.iter().max().unwrap()
    };
}

#[allow(unused_macros)]
macro_rules! sort {
    ($v:ident) => {
        $v.as_mut_slice().sort()
    };
}

#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {

}
