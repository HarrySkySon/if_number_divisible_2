use std::io;
use std::thread;

fn main() {
    // the program takes a user input number
    println!("Enter a number:");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let user_input: u64 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please enter a valid number");
            return;
        }
    };

    // and finds all the numbers in the range from 1 to user_input that can be divided by the user input without a remainder
    println!(
        "There are all numbers in the range from 1 to {} that can be divided by your number without a remainder:",
        user_input
    );

    let num_threads :u64 = num_cpus::get().try_into().unwrap();
    let chunk_size = (user_input + num_threads - 1) / num_threads;
    let mut threads = vec![];

    for chunk_start in (1..=user_input).step_by(chunk_size as usize) {
        let chunk_end = chunk_start + chunk_size - 1;
        let chunk_end = std::cmp::min(chunk_end, user_input);
        let user_input = user_input.clone();
        let thread = thread::spawn(move || {
            for i in chunk_start..=chunk_end {
                if user_input % i == 0 {
                    println!("{}", i);
                }
            }
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
