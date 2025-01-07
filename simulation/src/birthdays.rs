use rand::prelude::*;
use rayon::prelude::*;
use std::collections::HashSet;

//pub fn shares_birthday () {
  //  const N_PEOPLE: usize = 30;
    //const TRIALS: usize = 1_000_000;
     // const DAYS_IN_YEAR: i32 = 365;
    //let share_birthday = (0..TRIALS).into_par_iter().filter(|_|{
     //   let mut birthdays = HashSet::new();
    //    (0..N_PEOPLE).map(|_| thread_rng().gen_range(0..DAYS_IN_YEAR)).any
    //    (|b| !birthdays.insert(b))
  //  }).count();
  //  println!(
  //      "Probability of {} people sharing a birthday is {:.4}%",
    //    N_PEOPLE,
   //     share_birthday as f32 / TRIALS as f32 * 100.00
   // );
//}

use std::time::Instant;


const TRIALS: usize = 1_000_000;  // Adjust this as needed
const N_PEOPLE: usize = 30;       // Number of people
const DAYS_IN_YEAR: usize = 365;  // Days in a year

pub fn shares_birthday() {
    // Measure time for parallel version
    let start_parallel = Instant::now();
    
   let share_birthday_parallel = (0..TRIALS).into_par_iter()
      .filter(|_| {
           let mut birthdays = HashSet::new();
            (0..N_PEOPLE)
               .map(|_| rand::thread_rng().gen_range(0..DAYS_IN_YEAR))
               .any(|b| !birthdays.insert(b)) // Insert returns false if duplicate
        })
       .count();
    
   let duration_parallel = start_parallel.elapsed();
   println!("Parallel simulation time: {:?}", duration_parallel);

    // Measure time for serial version
    let start_serial = Instant::now();
    
    let share_birthday_serial = (0..TRIALS).into_iter()
        .filter(|_| {
            let mut birthdays = HashSet::new();
            (0..N_PEOPLE)
                .map(|_| rand::thread_rng().gen_range(0..DAYS_IN_YEAR))
                .any(|b| !birthdays.insert(b)) // Insert returns false if duplicate
        })
        .count();
    
    let duration_serial = start_serial.elapsed();
    println!("Serial simulation time: {:?}", duration_serial);

    // Optionally, you can compare the results
  
  println!("Results: Parallel = {}, Serial = {}", share_birthday_parallel as f32 / TRIALS as f32 * 100.00 , share_birthday_serial as f32 / TRIALS as f32 * 100.00);
}
