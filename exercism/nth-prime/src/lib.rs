pub fn nth(n: u32) -> u32 {
  let mut primes: Vec<u32> = vec![];
  (2..)
    .filter(|&num| {
      if primes.iter().all(|prime| num % prime != 0) {
        primes.push(num);
        true
      } else {
        false
      }
    })
    .nth(n as usize)
    .unwrap()
}

pub fn my_nth_2(n: u32) -> u32 {
  let mut primes: Vec<u32> = vec![];
  for num in 2.. {
    let mut valid = true;
    for prime in &primes {
      if num % prime == 0 {
        valid = false;
        break;
      }
    }
    if valid {
      primes.push(num);
    }
    if primes.len() == (n + 1) as usize {
      return primes[primes.len() - 1];
    }
  }
  0
}

pub fn nth_2(n: u32) -> u32 {
  let mut primes = vec![2];
  for num in 3.. {
    if primes.iter().all(|x| num % x != 0) {
      primes.push(num);
    }
    if primes.len() > n as usize {
      break;
    }
  }
  primes[n as usize]
}

use std::ops::RangeFrom;

pub fn nth_3(n: u32) -> u32 {
  (RangeFrom { start: 2 })
    .filter(|x| is_prime_3(x))
    .nth(n as usize)
    .unwrap()
}

pub fn is_prime_3(n: &u32) -> bool {
  !(2..*n).any(|x| n % x == 0)
}

pub fn nth_4(n: u32) -> u32 {
  (2..)
    .filter(|&number| is_prime_4(number))
    .nth(n as usize)
    .unwrap()
}

pub fn is_prime_4(n: u32) -> bool {
  !(2..n).any(|factor| n % factor == 0)
}

pub fn is_prime_5(n: u32) -> bool {
  !(2..n / 2 + 1).any(|i| n % i == 0)
}

pub fn is_prime(n: u32) -> bool {
  !(2..n / 2 + 1).any(|factor| n % factor == 0)
}
