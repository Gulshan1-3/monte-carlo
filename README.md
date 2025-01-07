# Birthday Paradox Simulation using Monte Carlo Method

This repository contains a Rust implementation of the **Birthday Paradox** simulation, which calculates the probability of at least two people in a group sharing a birthday. The implementation uses the Monte Carlo method for approximation and demonstrates parallel computation for enhanced performance.

---

## What is the Birthday Paradox?
The Birthday Paradox refers to the counterintuitive probability that in a group of just 23 people, there's about a 50% chance that two people share the same birthday. The probability grows rapidly with the number of people in the group.

### Mathematical Background
To compute the probability mathematically:
1. Assume there are 365 days in a year and birthdays are uniformly distributed.
2. The probability that no two people in a group of `N` share a birthday is given by:

**2. Probability of no shared birthday:**

P(no shared birthday) = (365/365) * (364/365) * (363/365) * ... * ((365 - N + 1)/365)

**3. Probability of at least one shared birthday:**

P(shared birthday) = 1 - P(no shared birthday)

For larger groups, direct computation becomes cumbersome. Here, the Monte Carlo method provides an efficient approximation.

---

## Monte Carlo Method
The Monte Carlo method involves running a large number of random simulations to approximate probabilities. For this problem:
1. Simulate random birthdays for a group of `N` people.
2. Check if at least two birthdays match.
3. Repeat this simulation for many trials.
4. Estimate the probability as:

 ## P(shared birthday) ≈ Number of trials with shared birthdays / Total trials



---

## Code Explanation
Below is the Rust code implementation for simulating the Birthday Paradox using the Monte Carlo method:

```rust
use rand::prelude::*;
use rayon::prelude::*;
use std::collections::HashSet;

pub fn shares_birthday() {
    const N_PEOPLE: usize = 30;      // Number of people in the group
    const TRIALS: usize = 1_000_000; // Number of Monte Carlo trials
    const DAYS_IN_YEAR: i32 = 365;   // Number of days in a year

    // Parallel simulation of the trials
    let share_birthday = (0..TRIALS).into_par_iter()  // Parallel iterator for performance
        .filter(|_| {
            let mut birthdays = HashSet::new();
            // Generate random birthdays and check for duplicates
            (0..N_PEOPLE)
                .map(|_| thread_rng().gen_range(0..DAYS_IN_YEAR))
                .any(|b| !birthdays.insert(b)) // Insert returns false if duplicate
        })
        .count();

    // Calculate and print the probability
    println!(
        "Probability of {} people sharing a birthday is {:.4}%",
        N_PEOPLE,
        share_birthday as f32 / TRIALS as f32 * 100.0
    );
}
```

### Key Components:
1. **Random Birthday Generation**:
   - `thread_rng().gen_range(0..DAYS_IN_YEAR)` generates a random integer representing a birthday.
   - The range is `[0, 364]` corresponding to 365 days in a year.

2. **Duplicate Detection**:
   - A `HashSet` is used to store unique birthdays.
   - The `insert` method of `HashSet` returns `false` if the birthday is already present, indicating a shared birthday.

3. **Monte Carlo Simulation**:
   - The simulation runs for `TRIALS` iterations.
   - `into_par_iter()` enables parallelism using the `rayon` crate, significantly speeding up the computation.

4. **Probability Calculation**:
   - The proportion of trials with at least one shared birthday is computed as:

     \[
     P = \frac{\text{Number of trials with shared birthdays}}{\text{Total trials}} \times 100\%
     \]

---
