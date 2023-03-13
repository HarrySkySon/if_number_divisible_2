# if_number_divisible_2
This program founds all numbers that are devide user input number without reminder. It does it in parallel mode!

This program prompts the user to enter a number, reads the input as a string, converts it to an unsigned 64-bit integer using the parse() method
It checks if the user input number is divisible without a remainder using the modulo operator %. If the number is divisible,
it prints the list of all numbers to the console that are dividers without a remainder.
The parallelize the loop that checks for divisibility using Rust's std::thread.
First, we import the std::thread module. We then determine the number of CPU cores available using the num_cpus::get() function from the num_cpus crate. We calculate the chunk size (i.e., the number of values each thread should check) as chunk_size = (user_input + num_threads - 1) / num_threads. We then create a vector to hold the thread handles and iterate over the chunks, creating a new thread for each chunk. Each thread takes a clone of the user_input variable and checks all values in the current chunk for divisibility. The results are printed to the standard output stream. Finally, we wait for all threads to finish by calling join() on each thread handle.
