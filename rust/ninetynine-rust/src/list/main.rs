#[cfg(any(feature = "p23", feature = "p24", feature = "p25"))]
extern crate rand;
#[cfg(any(feature = "p23", feature = "p24", feature = "p25"))]
use rand::Rng;

use std::cmp::PartialEq;
#[cfg(any(feature = "p07", feature = "p00"))]
use Node::{One, Many};
#[cfg(any(feature = "p11", feature = "p12", feature = "p13"))]
use CT::{One, Many};

#[cfg(any(feature = "p07", feature = "p00"))]
#[derive(Debug)]
enum Node<T> {
    One(T),
    Many(Vec<Node<T>>),
}

//P01 (*) Find the last box of a list.
#[cfg(feature = "p01")]
fn prob1<T>(vec: &[T]) -> Option<&T>{
    vec.last()
}

//P02 (*) Find the last but one box of a list.
#[cfg(feature = "p02")]
fn prob2<T>(vec: &[T]) -> Option<(&T,&T)>{
    match vec.len(){
        x if x < 2 => None,
        x => Some((&vec[x-2], &vec[x-1])),
    }
}

//P03 (*) Find the K'th element of a list.
#[cfg(feature = "p03")]
fn prob3<'a, T>(vec: &'a [T], counter: &usize) -> Option<&'a T>{
    match vec.len(){
        x if x < *counter-1 => None,
        _ => Some(&vec[*counter-1]),
    }
}

//P04 (*) Find the number of elements of a list.
#[cfg(feature = "p04")]
fn prob4<'a, T:PartialEq>(vec: &'a [T], item: &T) -> usize{
    vec.iter().filter(|&x| *x == *item).fold(0, |sum, i| sum + 1)
}

//P05 (*) Reverse a list.
#[cfg(any(feature = "p05", feature = "p06"))]
fn prob5<T: Clone>(vec: &Vec<T>) -> Vec<T>{
    let mut x = (*vec).clone();
    x.reverse();
    x
}

//P06 (*) Find out whether a list is a palindrome.
#[cfg(feature = "p06")]
fn prob6<T: Clone+PartialEq>(vec: &Vec<T>) -> bool{
    let rev = prob5(vec);
    *vec == rev
}

#[cfg(feature = "p05")]
fn main() {
    let vec = vec!(1,2,3,4,3,2,1);
    let item = prob5(&vec);
    println!("{}", item[0]);
}

#[cfg(feature = "p06")]
fn main() {
    let vec = vec!(1,2,3,4,3,2,1);
    let item = prob6(&vec);
    println!("{}", item);
}

//P07 (**) Flatten a nested list structure.
#[cfg(feature = "p07")]
fn prob7<T>(vec: Node<T>) -> Vec<T>{
    match vec{
        One(x) => vec!(x),
        Many(x) => x.into_iter().flat_map(|item| prob7(item).into_iter()).collect(),
    }
}

#[cfg(feature = "p07")]
fn main() {
    let items = Many(vec![Many(vec![One(1i32),One(2)]),One(3),One(4)]);
    let flatten = prob7(items);
    println!("{:?}", flatten);
}

//P08 (**) Eliminate consecutive duplicates of list elements.
#[cfg(feature = "p08")]
fn prob8<T: PartialEq>(vec: Vec<T>) -> Vec<T>{
    vec.into_iter().fold(Vec::new(), |mut sum, x| {
        if sum.len() == 0{
            sum.push(x);
            sum
        } else if sum[sum.len()-1] == x {
            sum
        } else {
            sum.push(x);
            sum
        }
    })
}

#[cfg(feature = "p08")]
fn main() {
    let vec = vec!(1,1,1,1,2,2,2,3,3,3,3,4,4,4);
    let packed = prob8(vec);
    println!("{:?}", packed);
}

//P09 (**) Pack consecutive duplicates of list elements into sublists.
#[cfg(feature = "p09")]
fn prob9<T: PartialEq>(vec: Vec<T>) -> Vec<Vec<T>>{
    vec.into_iter().fold(Vec::new(), |mut vec, x| {
        let len = vec.len();
        if len == 0 {
            vec.push(vec![x]);
        } else if vec[len-1][0] == x {
            vec[len-1].push(x)
        } else{
            vec.push(vec![x]);
        }
        vec
    })
}

#[cfg(feature = "p09")]
fn main() {
    let vec = vec!(1,1,1,1,2,2,2,3,3,3,3,4,4,4);
    let packed = prob9(vec);
    println!("{:?}", packed);
}

//P10 (*) Run-length encoding of a list.
#[cfg(any(feature = "p10", feature = "p11", feature="p12"))]
fn prob10<T: PartialEq>(vec: Vec<T>) -> Vec<(usize, T)>{
    vec.into_iter().fold(Vec::new(), |mut vec, x| {
        let len = vec.len();
        if len == 0 {
            vec.push((1, x));
        } else if vec[len-1].1 == x {
            vec[len-1] = (vec[len-1].0 + 1, x);
        } else{
            vec.push((1, x));
        }
        vec
    })
}

#[cfg(feature = "p10")]
fn main() {
    let vec = vec!(1,1,1,1,2,2,2,3,3,3,3,4,4,4,5);
    let packed = prob10(vec);
    println!("{:?}", packed);
}

//P11 (*) Modified run-length encoding.
#[cfg(any(feature = "p11", feature = "p12"))]
fn prob11<T: PartialEq>(input: Vec<T>) -> Vec<CT<T>>{
    let x = prob10(input);
    x.into_iter().map(|item| match item{
        (1, x) => One(x),
        (a, x) => Many((a, x)),
    }).collect()
}

#[cfg(any(feature = "p11", feature = "p12", feature = "p13"))]
#[derive(Debug)]
enum CT<T> {
    One(T),
    Many((usize, T)),
}

#[cfg(feature = "p11")]
fn main() {
    let vec = vec!(1,1,1,1,2,2,2,3,3,3,3,4,4,4,5);
    let packed = prob11(vec);
    println!("{:?}", packed);
}

#[cfg(feature = "p12")]
fn prob12<T: PartialEq+Clone>(input: Vec<CT<T>>) -> Vec<T>{
    input.into_iter().flat_map(|item| match item{
        One(x) => vec!(x),
        Many((n, x)) => {
            vec![x.clone(); n]
        },
    }).collect()
}

#[cfg(feature = "p12")]
fn main() {
    let vec = vec!(1,1,1,1,2,2,2,3,3,3,3,4,4,4,5);
    let packed = prob11(vec);
    let unpacked = prob12(packed);
    println!("{:?}", unpacked);
}

//P13 (**) Run-length encoding of a list (direct solution).
#[cfg(feature = "p13")]
fn prob13<T: PartialEq+Clone>(input: Vec<T>) -> Vec<CT<T>>{
    let mut retVec: Vec<CT<T>> = Vec::new();

    for x in input.into_iter(){
        let w = match retVec.last(){
            Some(&One(ref a)) if *a == x => Some(Many((2, x.clone()))),
            Some(&Many((n, ref a))) if *a == x => Some(Many((n+1, x.clone()))),
            _ => None
        };

        match w{
            None => retVec.push(One(x.clone())),
            Some(z) => *retVec.last_mut().unwrap() = z,
        }
    }

    retVec
}

#[cfg(feature = "p13")]
fn main() {
    let vec = vec!(1,1,1,1,2,2,2,3,3,3,3,4,4,4,5);
    let packed = prob13(vec);
    println!("{:?}", packed);
}

//P14 (*) Duplicate the elements of a list.
#[cfg(feature = "p14")]
fn prob14<T: PartialEq+Clone>(input: Vec<T>) -> Vec<T>{
    input.into_iter().flat_map(|item| vec!(item.clone(), item.clone())).collect()
}

#[cfg(feature = "p14")]
fn main() {
    let vec = vec!(1,2,2,3,3,3,4,4,4,4);
    let packed = prob14(vec);
    println!("{:?}", packed);
}

//P15 (**) Replicate the elements of a list a given number of times.
#[cfg(feature = "p15")]
fn prob15<T: PartialEq+Clone>(input: Vec<T>, counter: usize) -> Vec<T>{
    input.into_iter().flat_map(|item| vec!(item.clone();counter)).collect()
}

#[cfg(feature = "p15")]
fn main() {
    let vec = vec!(1,2,3,4);
    let packed = prob15(vec, 3);
    println!("{:?}", packed);
}

//P16 (**) Drop every N'th element from a list.
#[cfg(feature = "p16")]
fn prob16<T: PartialEq+Clone>(input: Vec<T>, counter: usize) -> Vec<T>{
    input.into_iter().enumerate().filter_map(|item| {
        if (item.0 + 1) % counter != 0 { Some(item.1) }
        else { None }
    }).collect()
}

#[cfg(feature = "p16")]
fn main() {
    let vec = vec!(1,2,3,4,5,6,7,8,9,10);
    let packed = prob16(vec, 3);
    println!("{:?}", packed);
}

//P17 (*) Split a list into two parts; the length of the first part is given.
#[cfg(feature = "p17")]
fn prob17<T: PartialEq+Clone>(input: Vec<T>, counter: usize) -> Vec<Vec<T>>{
    input.into_iter().enumerate().fold(vec!(vec!(), vec!()), |mut sum, item| {
        if item.0 < counter { sum[0].push(item.1); sum }
        else { sum[1].push(item.1); sum }
    })
}

#[cfg(feature = "p17")]
fn main() {
    let vec = vec!(1,2,3,4);
    let packed = prob17(vec, 2);
    println!("{:?}", packed);
}

//P18 (**) Extract a slice from a list.
#[cfg(feature = "p18")]
fn prob18<T: PartialEq+Clone>(input: Vec<T>, start: usize, end: usize) -> Vec<T>{
    input.into_iter().enumerate().filter_map(|item| {
        if item.0 >= start - 1 && item.0 <= end - 1 { Some(item.1) }
        else { None }
    }).collect()
}

#[cfg(feature = "p18")]
fn main() {
    let vec = vec!(1,2,3,4);
    let packed = prob18(vec, 2, 3);
    println!("{:?}", packed);
}

//P19 (**) Rotate a list N places to the left.
#[cfg(feature = "p19")]
fn prob19<T: PartialEq+Clone>(a: Vec<T>, n: usize) -> Vec<T> {
    let mut r = vec![];
    let (v1, v2) = a.split_at(n);
    r.extend_from_slice(v2);
    r.extend_from_slice(v1);
    r
}

#[cfg(feature = "p19")]
fn main() {
    let vec = vec!(1,2,3,4);
    let packed = prob19(vec, 2);
    println!("{:?}", packed);
}

//P20 (*) Remove the K'th element from a list.
#[cfg(any(feature = "p20", feature = "p23", feature = "p24", feature = "p25"))]
fn prob20<T: PartialEq+Clone>(input: Vec<T>, n: usize) -> Vec<T> {
    input.into_iter().enumerate().filter_map(|item| {
        if item.0 != n { Some(item.1) }
        else { None }
    }).collect()
}

#[cfg(feature = "p20")]
fn main() {
    let vec = vec!(1,2,3,4);
    let packed = prob20(vec, 2);
    println!("{:?}", packed);
}

//P21 (*) Insert an element at a given position into a list.
#[cfg(feature = "p21")]
fn prob21<T: PartialEq+Clone>(insert_item: T, input: Vec<T>, n: usize) -> Vec<T> {
    input.into_iter().enumerate().flat_map(|item| {
        if item.0 == n { vec!(insert_item.clone(), item.1) }
        else { vec!(item.1) }
    }).collect()
}

#[cfg(feature = "p21")]
fn main() {
    let vec = vec!(1,2,3,4);
    let packed = prob21(35, vec, 2);
    println!("{:?}", packed);
}

//P22 (*) Create a list containing all integers within a given range.
#[cfg(any(feature = "p22", feature = "p24"))]
fn prob22(start: usize, end: usize) -> Vec<usize> {
    let mut r: Vec<usize> = Vec::new();
    for x in start..end+1{
        r.push(x);
    }
    r
}

#[cfg(feature = "p22")]
fn main() {
    let vec = prob22(4, 9);
    println!("{:?}", vec);
}

//P23 (**) Extract a given number of randomly selected elements from a list.
#[cfg(any(feature = "p23", feature = "p24", feature = "p25"))]
fn sub_prob23<T: PartialEq+Clone+Copy>(val: &mut Vec<T>, item: Vec<T>, counter: usize) -> Vec<T> {
    if counter == 0 { return (*val).clone() }
    let random = rand::thread_rng().gen_range(0, item.len());
    (*val).push(item[random]);
    sub_prob23(val, prob20(item, random), counter - 1)
}

#[cfg(any(feature = "p23", feature = "p24", feature = "p25"))]
fn prob23<T: PartialEq+Clone+Copy>(item: Vec<T>, n: usize) -> Vec<T> {
    sub_prob23(&mut Vec::new(), item, n)
}

#[cfg(feature = "p23")]
fn main() {
    let vec = vec!(1,2,3,4,5,6,7,8,9,10);
    let random = prob23(vec, 4);
    println!("{:?}", random);
}

//P24 (*) Lotto: Draw N different random numbers from the set 1..M.
#[cfg(feature = "p24")]
fn prob24(num: usize, size: usize) -> Vec<usize> {
    let source = prob22(1, size);
    prob23(source, num)
}

#[cfg(feature = "p24")]
fn main() {
    let random = prob24(500, 1000);
    println!("{:?}", random);
}

//P25 (*) Generate a random permutation of the elements of a list.
#[cfg(feature = "p25")]
fn prob25<T: PartialEq+Clone+Copy>(item: Vec<T>) -> Vec<T> {
    let count = item.len().clone();
    prob23(item, count)
}

#[cfg(feature = "p25")]
fn main() {
    let vec = vec!(1,2,3,4,5,6,7,8,9,10);
    let random = prob25(vec);
    println!("{:?}", random);
}

//P26 (**) Generate the combinations of K distinct objects chosen from the N elements of a list
#[cfg(any(feature = "p26", feature = "p27"))]
fn prob26<T: PartialEq+Clone+Copy>(item: Vec<T>, counter: usize) -> Vec<Vec<T>> {
    sub_prob26(Vec::new(), item, counter)
}

#[cfg(any(feature = "p26", feature = "p27"))]
fn sub_prob26<T: PartialEq+Clone+Copy>(joint: Vec<T>, rest_item: Vec<T>, counter: usize) -> Vec<Vec<T>> {
    if counter == 0 { return vec!(joint) }
    let rest_item_backup = rest_item.clone();
    let max = rest_item.len().clone();
    let candidates = rest_item.into_iter().enumerate().filter(|item| {
        item.0 + counter <= max
    });

    let mut r: Vec<Vec<T>> = Vec::new();
    for item in candidates{
        let mut header = joint.clone();
        header.push(item.1);
        let next_candidates = rest_item_backup.clone().split_off(item.0 + 1);
        for answer in sub_prob26(header, next_candidates, counter - 1){
            r.push(answer)
        }
    }
    r
}

#[cfg(feature = "p26")]
fn main() {
    let vec = vec!(1,2,3,4,5,6,7,8,9,10);
    let random = prob26(vec, 5);
    println!("{:?}", random);
}

//P27 (**) Group the elements of a set into disjoint subsets.
#[cfg(feature = "p27")]
fn combination_and_rest<T: PartialEq+Clone+Copy>(src: &Vec<T>, group_size: usize) -> Vec<(Vec<T>, Vec<T>)> {
    let combination = prob26(src.clone(), group_size);
    combination.into_iter().map(|combi|{
        let rest = src.clone().into_iter().filter(|item|{
            !combi.contains(&item)
        }).collect();
        (combi, rest)
    }).collect()
}

#[cfg(feature = "p27")]
fn rest_two<T: PartialEq+Clone+Copy>(combi: (Vec<T>, Vec<T>)) -> Vec<(Vec<T>, Vec<T>, Vec<T>)>{
    let new_combi = combination_and_rest(&combi.1, 3);
    new_combi.into_iter().map(|item|{
        (combi.0.clone(), item.0, item.1)
    }).collect()
}

#[cfg(feature = "p27")]
fn prob_27(){
    let vec = vec!(1,2,3,4,5,6,7,8,9);
    let first_two = combination_and_rest(&vec, 2);
    let hoge: Vec<(Vec<usize>, Vec<usize>, Vec<usize>)> = first_two.into_iter().flat_map(|combi|{
        rest_two(combi)
    }).collect();
    println!("hoge {:?}", hoge);
}

#[cfg(feature = "p27")]
fn main() {
    prob_27();
}

//P28 (**) Sorting a list of lists according to length of sublists
#[cfg(feature = "p28")]
use std::cmp::Ordering;

#[cfg(feature = "p28")]
fn prob_28(){
    let vec0 = vec!(9,8);
    let vec1 = vec!(1,2,3);
    let vec2 = vec!(2,3);
    let vec3 = vec!(1,2);
    let vec4 = vec!(1,2,3,4);
    let vec5 = vec!(1,2,3,4,5);
    let vec6 = vec!(5);
    let mut vec: Vec<Vec<usize>> = Vec::new();
    vec.push(vec0);
    vec.push(vec1);
    vec.push(vec2);
    vec.push(vec3);
    vec.push(vec4);
    vec.push(vec5);
    vec.push(vec6);
    vec.sort_by(|a, b|{
        if a.len() < b.len() { return Ordering::Less }
        else if a.len() > b.len() { return Ordering::Greater }
        else{
            a.cmp(b)
        }
    });

    println!("sort {:?}", vec);
}

#[cfg(feature = "p28")]
fn main() {
    prob_28();
}