use rand::prelude::*;
use rayon::prelude::*;
use std::collections::HashSet;

pub fn shares_birthday () {
    const N_PEOPLE: usize = 30;
    const TRIALS: usize = 1_000_000;
    const DAYS_IN_YEAR: i32 = 365;
    let share_birthday = (0..TRIALS).into_par_iter().filter(|_|{
        let mut birthdays = HashSet::new();
        (0..N_PEOPLE).map(|_| thread_rng().gen_range(0..DAYS_IN_YEAR)).any
        (|b| !birthdays.insert(b))
    }).count();
    println!(
        "Probability of {} people sharing a birthday is {:.4}%",
        N_PEOPLE,
        share_birthday as f32 / TRIALS as f32 * 100.00
    );
}