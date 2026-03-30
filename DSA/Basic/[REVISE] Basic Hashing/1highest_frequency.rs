use std::collections::HashMap;

fn main() {
    println!("{}", most_frequency_element(&[1, 2, 2, 3, 3, 3]));
}

fn most_frequency_element(a: &[i32]) -> i32 {
    let mut h = HashMap::new();

    for e in a {
        let c = h.get(e);

        if c.is_some() {
            h.insert(e, c.unwrap()+1);
        } else {
            h.insert(e, 1);
        }
    }

    // let mut max_key = h.keys().copied().copied().collect::<Vec<i32>>()[0]; Better approach below
    let mut max_key = **h.keys().next().unwrap();
    for &k in h.keys() {
        if h[k] > h[&max_key] || (h[k] == h[&max_key] && *k < max_key) {
            max_key = *k;
        }
    }
    
    max_key
    // **h.keys().filter(|k| h[*k] == h[&max_key]).min().unwrap()
}


// Genius solution found by AI below. The explanation follows
// fn most_frequency_element(a: &[i32]) -> i32 {
//     let mut h = HashMap::new();

//     // The `&e` pattern matches the reference, meaning `e` is now an `i32`
//     for &e in a {
//         // The entry API handles the if/else logic for you
//         *h.entry(e).or_insert(0) += 1;
//     }

//     // Find the key with the maximum value directly from the iterator
//     h.into_iter()
//         // Compare by count first (ascending), then by val (descending/reversed)
//         .max_by_key(|&(val, count)| (count, std::cmp::Reverse(val)))
//         .map(|(val, _)| val)
//         .unwrap_or(0) // unwrap_or(0) is safer than unwrap() for empty arrays!
// }

/*
Detailed breakdown of this one magical line:

Rust*freq.entry(item).or_insert(0) += 1;
This is the fastest and most idiomatic way to count frequencies in Rust. Let's dissect it step by step, exactly what the compiler sees.
1. freq.entry(item)

HashMap has a method called entry that takes ownership of the key (or borrows it if possible).
It returns an Entry enum (not a value yet!):

pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}
Behind the scenes, it does one hash lookup and decides:
Is the key already in the map? → Occupied
Is the key missing? → Vacant


This is the key to why it's fast — only one hash computation + lookup per element.
2. .or_insert(0)
This is a method on the Entry enum. It does different things depending on the variant:
Rust// Pseudo-code of what or_insert actually does:
match entry {
    Occupied(mut occupied) => occupied.into_mut(),        // returns &mut V (existing value)
    Vacant(vacant)         => vacant.insert(0),           // inserts 0 and returns &mut V
}
What happens in practice:

First time we see a value (e.g. item = 5):
entry → Vacant
or_insert(0) → inserts 5: 0 into the map
returns &mut i32 pointing to that 0

Second time we see the same value (item = 5 again):
entry → Occupied
or_insert(0) → does nothing (doesn't insert again)
returns &mut i32 pointing to the existing 1 (or whatever it was)


So or_insert(0) always gives you a mutable reference (&mut V) to the count, whether the key existed or not.
3. * ... += 1

The * dereferences the &mut i32 we just got.
+= 1 increments it in place.

So the full line is equivalent to:
Rustlet count = freq.entry(item).or_insert(0);  // &mut i32
*count += 1;
Visual walkthrough with example
Start with empty HashMap<i32, i32> and array [1, 2, 2]
Iteration 1: item = 1

entry(1) → Vacant
or_insert(0) → inserts {1: 0} → returns &mut 0
*count += 1 → now {1: 1}

Iteration 2: item = 2

entry(2) → Vacant
or_insert(0) → inserts {1:1, 2:0} → returns &mut 0
*count += 1 → now {1:1, 2:1}

Iteration 3: item = 2

entry(2) → Occupied
or_insert(0) → does nothing
returns &mut 1 (the existing value)
*count += 1 → now {1:1, 2:2}




----------------------------------------------------------------------
Line 44 onwards: Here is what each step is doing, one by one:

1. h.into_iter(): The Conveyor Belt
This takes your HashMap and dumps all its data onto a conveyor belt. Instead of a map, you now have a stream of pairs (tuples) flowing down the line.

If your array was [3, 3, 1, 1], the stream looks like this: (3, 2) and (1, 2).

In these pairs, the first number is the val (the number from your array) and the second is the count (its frequency).

2. .max_by_key(...): The Tournament
This is where the actual logic happens. It looks at every pair on the conveyor belt and tries to find the "maximum" one based on a custom rule you give it.

Let's look at your custom rule: |&(val, count)| (count, Reverse(val))

|&(val, count)|: This just says, "For every pair coming down the belt, temporarily label the first thing val and the second thing count so I can use them."

(count, Reverse(val)): This is the genius part of Rust. You are creating a brand new, temporary tuple just for the sake of comparison. Rust compares tuples strictly left-to-right.

Round 1 (count): It compares the frequencies first. If one frequency is higher (e.g., 5 vs 2), that element wins immediately. The comparison stops.

Round 2 (Reverse(val)): If the counts are a dead tie (e.g., both appeared 2 times), Rust moves to the second item in the tuple to break the tie. Normally, Rust would say 3 > 1. But because you wrapped it in Reverse(), you tricked the compiler! It flips the logic upside down so the compiler thinks Reverse(1) is greater than Reverse(3). This forces the smallest val to win the tie-breaker.

3. .map(|(val, _)| val): The Extraction
By the time the tournament is over, .max_by_key has selected the winning pair—let's say it's (1, 2).

But your function signature says it needs to return an i32 (just the number), not a tuple. .map takes the winning tuple (1, 2), grabs only the val part (1), and throws the count away into the void (represented by the _).

4. .unwrap_or(0): The Safety Net
What if someone passed an empty array [] into your function?

The map would be empty.

The conveyor belt would be empty.

.max_by_key wouldn't find a winner; it would return None.

If you just used .unwrap() on None, your program would violently crash (panic). .unwrap_or(0) is your safety net. It says: "If you have a winning number, give it to me. If you have None, don't panic, just give me 0."
*/