fn main() {
    euclides(2345678765, 2049);
}

fn euclides(mut a: i64, mut b: i64) { 
    let mut rest: i64;
    while b != 0 { // start a loop that will end when b is zero.
        rest = a % b; // a mod b
        a = b;
        b = rest;
        println!("{}", a); // print a
    }   
}