extern crate rand;

use rand::distributions::{Distribution, Uniform};
use std::io;

fn get_input () -> i64 {
  let mut input = String::new();
  loop {
    if let Err(e) = io::stdin().read_line(&mut input)
    {
      println!("Keine Zeichen gelesen. Fehler {0}", e);
      continue;
    }
    if let Ok(n) = input.trim().parse::<i64>()
    {
      return n;
    }
    else
    {
      println!("Keine Zahl gelesen. Nochmal");
      continue;
    }
  }
}

fn main () {
  let mut rng = rand::thread_rng();

  println!("Lass uns rechnen:");

  loop {
    let pyramid_range = Uniform::new_inclusive(2, 10);
    let pyramid = pyramid_range.sample(&mut rng);
    let sample_range = Uniform::new_inclusive(0,pyramid);
    let sample = sample_range.sample(&mut rng);

    println!("Berechne: {0} + {1}", sample, pyramid - sample);

    match get_input() == pyramid {
      true => {
        println!("Richtig");
        break;
      },
      false => {
        println!("Falsch");
        continue;
      }
    }
  }
}