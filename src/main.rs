use std::io;

fn main() {
    println!("Welcome to the number reverser!");
    
    loop 
    {
      println!("Please enter a positive #!");
      let mut input_num = String::new();

      io::stdin().read_line(&mut input_num)
                 .expect("Failed to read line");

      let input_num: u64 = match input_num.trim().parse()
                           {
                             // return num if valid
                             Ok(num) => num,
                             // keep asking if not valid
                             Err(_) => continue,
                           };
      println!("The reversed # is {}", rev_order(input_num));
    }
}

// takes a positive u64 and returns it reversed
fn rev_order(x: u64) -> u64 {
    // convert int to string
    let mut num: String = x.to_string();

    // reverse string
    num = num.chars().rev().collect();

    // convert string back into int
    let num: u64 = match num.trim().parse()
                   {
                    Ok(num) => num,
                    Err(_) => 0,
                   };    
    // return value
    num
}
