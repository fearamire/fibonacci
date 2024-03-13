use std::io;

fn main() {
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut c: u32 = 0;
    let mut i = 2;

    loop {
        let mut max = String::new();

        // Get index from user
        print!("Please enter the index of the fibonacci number you would like.\n");
        print!("(Use 0 indexing.)\n");

        io::stdin()
        .read_line(&mut max)
        .expect("Failed to read line.");

        let max:u32 = match max.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("That wasn't an acceptable number.\n\n");
                continue
            }
        };

        print!("\n\n");

        // Cases for indexes 0 and 1, else loop until correct index.
        if max == a {
            print!("The {max} index Fibonacci number is: {a}")
        } else if max == b {
            print!("The {max} index Fibonacci number is: {b}")
        } else {
            // iterate through and find the requested value
            while i <= max {
                c = a + b;
                a = b;
                b = c;

                i += 1;
            };
            print!("The {max} index Fibonacci number is: {b}");
        };

        println!("\n\nPress Enter to Exit...");
        let mut exit = String::new();
        io::stdin().read_line(&mut exit).unwrap();
        break
    }
    
}
