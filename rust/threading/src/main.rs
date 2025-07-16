use core::time;
use std::thread;

fn main() {
    let numbers: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7];

    /*
    let th = thread::spawn(move || numbers.iter().sum::<usize>() / numbers.len());

    match th.join() {
        Ok(res) => println!("Result : {}", res),
        Err(err) => println!("Error : {:?}", err),
    };
    */

    let res = thread::scope(|scoped| {
        scoped
            .spawn(|| {
                println!("2:{:?}", thread::current().id());
                thread::sleep(time::Duration::from_secs(120));
                numbers.iter().sum::<usize>() / numbers.len()
            })
            .join()
            .unwrap()
    });

    println!("{:?}", res);
}
