fn main() {
    for i in 1..=100 {
        match (i % 3, i % 5) {               
            // If both `i % 3` and `i % 5` == 0:
            (0, 0) => println!("FizzBuzz"),
            
            // Else if `i % 3` == 0:
            (0, _) => println!("Fizz"),
                
            // Else if `i % 5` == 0:
            (_, 0) => println!("Buzz"),
                
            // Else:
            _ => println!("{}", i),
        }
    }
}
