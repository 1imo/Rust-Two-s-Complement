fn main() {
    loop {
        let mut i: String = String::new();
        std::io::stdin().read_line(&mut i).expect("Failed to read line");
        
        // Convert binary string to u32
        let num = match u32::from_str_radix(i.trim(), 2) {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        // Two's complement is bitwise NOT plus 1
        let result = (!num).wrapping_add(1);
        
        // Convert back to binary string
        println!("{:b}", result);
    }
}
