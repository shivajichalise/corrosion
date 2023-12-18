# Improvemnets

###### _zachs18 :: forall a. IO a (@pizzapants184 on discord)_

!(original)[https://github.com/shivajichalise/corrosion/commit/f6505edaea39b2e299f67aea7cd989e279b3759f#diff-6e6182021c370680c381ea98bb2a6cd6702c44ed417263f5c926d7e88286840d]

1. I'd recommend using an iterative `loop { if valid { return; } }` pattern instead of a recursive `if !valid { recurse(); }` pattern to parse user input, e.g.

   ```rs
   fn read_vertical_type() -> PatternVerticalType {
     let mut input = String::new();
     loop {
       input.clear();
       println!("Please choose one option.");
       println!("1. Top to bottom");
       println!("2. Bottom to top");
       print!("Enter either 1 or 2: ");

       io::stdout().flush().unwrap();
   io::stdin()
         .read_line(&mut input)
         .expect("Failed to read the input. :(");

       match input.trim().parse::<u32>() {
         Ok(1) => return PatternVerticalType::TopToBottom,
         Ok(2) => return PatternVerticalType::BottomToTop,
         Ok(_) => {
           println!();
           println!("***************************");
           println!("Please enter either 1 or 2.");
           println!("***************************");
           println!();
         }
         Err(_) => {
           println!();
           println!("-----------------------");
           println!("Please give an integer.");
           println!("-----------------------");
           println!();
         }
       };
     }
   }
   ```

   The recursive version would eventually overflow the stack if the user entered a few thousand invalid inputs,
   and it would also make a new `String` to read each input, whereas the `loop` one only makes one string and just clears it before reading the next try
   (If you did the `let mut input = ...;` _inside_ the `loop` then it would make a new string for each iteration,
   but it would drop the old string at the end of the previous iteration, which the recursive version wouldn't do)
   Additionally, it is not necessary in this case to have nested `match`es, you can pattern-match on the `i32` in the `Result<i32, _>` while matching on the `Result`
   (see the `Ok(1)` pattern) Also, I'd recommend specifying what type you are parsing as.
   Rust defaults to integer literals being `i32` so that's what you are parsing as in `take_verticle_type_input` currently.
   You can specify the type generic on the `parse` method using the "turbofish", e.g. `str.parse::<u32>()` to parse as `u32`.

   One thing I didn't modify in my function above is that the parsing routines will infinitely loop (or in your case, infinitely recurse) if stdin gets EOF.
   You may want to handle this case, e.g. by checking if `input` is an empty string (this indicates EOF, since the newline terminator is included otherwise)
