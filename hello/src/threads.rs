use std::thread;
// Data races are not allowed
pub fn run(){
    let mut data = 100;
    thread::spawn(|| { data = 500 });
    thread::spawn(|| { data = 1000 });
    println!("{:?}", data);
}