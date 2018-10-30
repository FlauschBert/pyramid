extern crate rand;

use rand::distributions::{Distribution, Uniform};
use std::io;

fn get_input () -> i64 {
  loop {
    // Reading from stdin failed in general
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input)
    {
      println!("Keine Zeichen gelesen. Fehler {0}", e);
      continue;
    }
    // Conversion to integer succeeded
    if let Ok(n) = input.trim().parse::<i64>()
    {
      return n;
    }
    // Conversion to integer failed
    else
    {
      println!("Keine Zahl gelesen. Nochmal");
      continue;
    }
  }
}

fn main () {
  let mut rng = rand::thread_rng();
  let low_bound = 2;
  let up_bound = 10;

  println!("Lass uns rechnen in den Pyramiden von {0} bis {1}:", low_bound, up_bound);
  println!("-------------------------------------------------");

  loop {
    let pyramid_range = Uniform::new_inclusive(low_bound, up_bound);
    let pyramid = pyramid_range.sample(&mut rng);
    let sample_range = Uniform::new_inclusive(0,pyramid);
    let sample = sample_range.sample(&mut rng);

    println!("Berechne: {0} + {1}", sample, pyramid - sample);

    match get_input() == pyramid {
      true => println!("Richtig"),
      false => println!("Falsch. {0} ist richtig", pyramid),
    }
    println!("---------------------");
  }
}