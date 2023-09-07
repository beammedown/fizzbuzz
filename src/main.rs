use std::time::{Duration, Instant};
use std::thread;

fn main() {
    let st1 = Instant::now();
    ifniceformatted();
    let ft1 = st1.elapsed();
    let st2 = Instant::now();
    unreadable();
    let ft2 = st2.elapsed();
    let st3 = Instant::now();
    matchmaker();
    let ft3 = st3.elapsed();
    thread::sleep(Duration::from_secs(1));
    println!("Mode one: {} \n Mode two: {} \n Mode three: {}", Duration::as_micros(&ft1), Duration::as_micros(&ft2), Duration::as_micros(&ft3));
}

fn ifniceformatted() {
    for i in 0..1001  {
        if i%15 == 0 {
            println!("Fizzbuzz");
        } else if i%5 == 0 {
            println!("Buzz");
        } else if i%3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", i);
        }
    }
}

fn unreadable() {
    for i in 0..1001 {if i%15 == 0 {println!("Fizzbuzz")}else if i%5==0{println!("Buzz")}else if i%3==0{println!("Fizz")}else{println!("{}", i)}}
}

fn matchmaker() {
    for i in 0..1001 {
        match (i%3, i%5) {
            (0, 0) => println!("Fizzbuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i)
        }
    }
}