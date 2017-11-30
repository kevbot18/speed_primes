/*
	Author: Kevin Ingalls (kevbot18)
	Date: 11/29/17
	Project: speed_primes
	Description: command line prime number generator. By far the fastest one I have ever made.
*/

/*
	translation of python code from
	https://www.quora.com/How-many-prime-numbers-are-there-between-1-and-1000/answers/15501891
	with optimizations:
		marks all evens (except 2) as not prime (false)
		skips ALL evens when multiplying
*/

/*
	In working on this, I innitially used a simple Vector of booleans.
	When testing, however, I noticed that it seemed to be using 8x more memory than it should.
	After scowering over the code for any vector copying and coming up empty, I researched the problem.
	Turns out that Rust and some other languages store booleans as a byte because that is the smallest unit of data that can be requested.
	To combat the memory usage I:
		store 8 consecutive numbers in an unsigned 8 bit integer
		store an array of the u8's
		rely on optimizations of number/(power of 2)
		remove the excess numbers from the end after completion (max of 7 extra numbers)
		spend too much time trying to get the math right and actually bring it together
*/

/*
	Bottlenecks:
		- println!() for every number is slow
			- quick tests with buffers resulted in no gains
*/

use std::env;


fn main() {
	// takes user imput and puts it in a vector
	let input: Vec<String> = env::args().collect();
	// takes first element in input vector and converts it to a u32 for prime number program
	let num: u32 = input[1].parse::<u32>().unwrap();
	// defaults to NOT showing all primes, and only counting them
	let list_primes = if input.len() > 2 { // checks if there are any more args
		match input[2].as_ref() {	// if the args match, show all primes
			"-l" => true,			// else, don't
			"l" => true,
			_ => false,
		}
	} else { // if no additional args were given
		false // don't show primes
	};
	// prints total primes
	println!("\n{} primes", num_primes(num, list_primes));
}

fn num_primes(num: u32, show_primes: bool) -> u32 {
	let mut count: u32 = 0;
	let elements = if num % 8 == 0 { num / 8 } else { num / 8 + 1 };
	let mut is_prime: Vec<u8> = Vec::with_capacity(elements as usize);
	// println!("{}", elements); // used for debugging the vector length

	// create new "data type" to fix bool=byte storage issue (reduce mem usage by 8)
	is_prime = vec![0b11111111; elements as usize]; // assigns 1's to every digit
	// makes all evens NOT prime
	for e in 0..elements as usize {
		let mut index = 0; // keeps track of place value
		while index <= 8 { // using while to count by 2's (why, Rust, why?)
			is_prime[e] &= !(2u8.pow(index) as u8); // &'s input with inverse of 2^(digit place)
			index += 2;
		}
	}
	is_prime[0] |= 1u8 << 2; // OR'ing sets bit to high (00110010 OR 00000001 = 00110011)
	is_prime[0] &= !3u8; // AND NOT'ing sets bit to low (00110010 AND !00000010 = 00110000)

	// checks all odds up to the square
	let root = isqrt(num);
	for i in 3..(root + 1) as usize { // only check to the square
		if is_prime[i / 8] >> (i % 8) & 1 == 1 { // checks if bit of current number (i) is high
			let mut k: usize = i;
			while k * i <= num as usize { // only check to 
				is_prime[(i * k) / 8] &= !(1u8 << ((i * k) % 8) as u8); // sets bit to low using AND NOT (see above)
				k += 2;
			}
		}
	}

	// remove ones above given number (sets bits to low so they are ignored when counting)
	for i in (num + 1)..elements * 8 { // only worry about values greater than input num
		is_prime[(elements - 1) as usize] &= !(2u8.pow(i % 8) as u8); // // sets bit to low using AND NOT (see above)
	}

	// counts the primes
	if show_primes { // prints out all primes (very slow and may crash terminal)
		for i in 0..is_prime.len() { // goes through every number
			for e in 0..8 { // goes through every bit in number
				if (is_prime[i as usize] >> e) & 1u8 == 1 { // if bit is high
					count += 1; // increase prime count
					print!("{}, ", i * 8 + e); // print out the number
				}
			}
		}
		println!();
	} else { // only counts primes
		for &e in &is_prime {
			count += e.count_ones(); // function counts all 1's in the number
		}
	}
	return count;
}


// recursive integer square root function found on Wikipedia
fn isqrt(n: u32) -> u32 {
	if n < 2 {
		return n;
	} else {
		let small = isqrt(n >> 2) << 1;
		let large = small + 1;
		if large * small > n {
			return small;
		} else {
			return large;
		}
	}
}
