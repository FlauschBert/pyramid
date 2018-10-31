extern crate rand;

use rand::distributions::{Distribution, Uniform};
use std::io;

fn get_input () -> u64 {
  loop {
    // Reading from stdin failed in general
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input)
    {
      println!("Keine Zeichen gelesen. Fehler {0}", e);
      continue;
    }
    // Conversion to integer succeeded
    if let Ok(n) = input.trim().parse::<u64>()
    {
      return n;
    }
    // Conversion to integer failed
    else
    {
      println!("Keine natürliche Zahl gelesen. Nochmal");
      continue;
    }
  }
}

// Pass immutable reference
fn print_line (string : &String) {
  let size = string.len();
  let mut line = String::with_capacity(size);
  for _char in 0..size {
    line.push('-');
  }
  println!("{}", line);
}

struct Counter {
  right : u64,
  wrong : u64,
}

fn main () {
  let mut rng = rand::thread_rng();

  let bounds = (2, 10);
  let pyramid_range = Uniform::new_inclusive(bounds.0, bounds.1);

  let mut counter: Counter = Counter {
    right: 0,
    wrong: 0,
  };

  let title = format!("Lass uns rechnen in den Pyramiden von {0} bis {1}:", bounds.0, bounds.1);
  println!("{}", title);
  print_line(&title);

  loop {
    // Choose pyramid randomly
    let pyramid = pyramid_range.sample(&mut rng);
    // Choose values inside pyramid randomly
    let sample_range = Uniform::new_inclusive(0,pyramid);
    let sample = sample_range.sample(&mut rng);

    println!("Berechne: {0} + {1}", sample, pyramid - sample);

    match get_input() == pyramid {
      true => {
          counter.right += 1;
          println!("Richtig");
      },
      false => {
          counter.wrong += 1;
          println!("Falsch. {0} ist richtig", pyramid);
      },
    }

    let counter_info = format!("Richtig: {0}, Falsch: {1}", counter.right, counter.wrong);
    // Pass immutable reference (no move, std::String does not support copy trait)
    print_line(&counter_info);
    println!("{}", counter_info);
    // Pass immutable reference (no move, std::String does not support copy trait)
    print_line(&counter_info);
  }
}
