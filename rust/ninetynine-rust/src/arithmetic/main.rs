use std::cmp::PartialEq;

//P31 (**) Determine whether a given integer number is prime.
#[cfg(any(feature = "p31", feature = "p35"))]
fn prob31(num: usize) -> bool{
    (2..num-1).all(|item| num % item != 0)
}

#[cfg(feature = "p31")]
fn main(){
    println!("3 is {}", prob31(3));
    println!("4 is {}", prob31(4));
    println!("5 is {}", prob31(5));
    println!("6 is {}", prob31(6));
    println!("7 is {}", prob31(7));
    println!("8 is {}", prob31(8));
    println!("9 is {}", prob31(9));
    println!("10 is {}", prob31(10));
    println!("11 is {}", prob31(11));
    println!("12 is {}", prob31(12));
}

//P32 (**) Determine the greatest common divisor of two positive integer numbers.
#[cfg(any(feature = "p32", feature = "p33", feature = "p34"))]
fn prob32(a: usize, b: usize) -> usize{
    if a == b { a }
    else if a > b { prob32(a - b, b) }
    else { prob32(b, b - a) }
}

#[cfg(feature = "p32")]
fn main(){
    println!("the greatest common divisor of 3 and 4 is {}", prob32(3, 4));
    println!("the greatest common divisor of 4 and 12 is {}", prob32(12, 4));
    println!("the greatest common divisor of 128 and 156 is {}", prob32(128, 156));
}

//P33 (*) Determine whether two positive integer numbers are coprime.
#[cfg(any(feature = "p33", feature = "p34"))]
fn prob33(a: usize, b: usize) -> bool{
    prob32(a, b) == 1
}

#[cfg(feature = "p33")]
fn main(){
    println!("{}", prob33(35, 64));
    println!("{}", prob33(4, 64));
}

//P34 (**) Calculate Euler's totient function phi(m).
#[cfg(feature = "p34")]
fn prob34(m: usize) -> usize{
    (1..m).filter(|item|{
        prob33(m, (*item).clone())
    }).count()
}

#[cfg(feature = "p34")]
fn main(){
    println!("{}", prob34(10));
}

//P35 (**) Determine the prime factors of a given positive integer.
#[cfg(feature = "p35")]
fn make_x(m: usize) -> usize{
    let x = (m as f32).sqrt().floor() as usize;
    x + 1
}

#[cfg(feature = "p35")]
fn factorization(n: usize) -> Vec<usize>{
    if prob31(n) == true { return vec![n] }
    let x = (2..n-1).find(|item| n % item == 0).unwrap();
    vec![x, n/x].into_iter().flat_map(|item|{
        factorization(item)
    }).collect()
}

#[cfg(feature = "p35")]
fn prob35(m: usize) -> Vec<usize>{
    factorization(m)
}

#[cfg(feature = "p35")]
fn main(){
    println!("{:?}", prob35(12000));
}

//P36 (**) Determine the prime factors of a given positive integer (2).
//same as prob 13

//skip
