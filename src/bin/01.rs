use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;

fn main() {
    // File hosts must exist in current path before this produces output
    let numbers= read_lines("./inputs/input1")
      .expect("File read error")
      .map(|s|s.expect("io error").parse::<i32>().expect("string parse error"));

    let number_of_increases = numbers.tuple_windows()
      .filter(|(prev,next)| next > prev)
      .count();
    println!("Answer to part 1 = {}", number_of_increases);

    // Part 2
    let numbers= read_lines("./inputs/input1")
    .expect("File read error")
    .map(|s|s.expect("io error").parse::<i32>().expect("string parse error"));

    let number_of_increases = numbers.tuple_windows()
      .map(|(x,y,z)| x+y+z)
      .tuple_windows()
      .filter(|(prev,next)| next > prev)
      .count();
    println!("Answer to part 2 = {}", number_of_increases);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
