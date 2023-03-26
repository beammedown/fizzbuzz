fn main() {
    for i in 0..100  {
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
    unreadable()
}

fn unreadable() {
    for i in 0..100 {if i%15 == 0 {println!("Fizzbuzz")}else if i%5==0{println!("Buzz")}else if i%3==0{println!("Fizz")}else{println!("{}", i)}}
}