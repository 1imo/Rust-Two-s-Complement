// Two's Complement Calculator
fn main() {
    let mut i: String = String::new();
    std::io::stdin().read_line(&mut i).expect("Failed to read line");

    let i:String = i.trim().to_string();
    let mut on:String = String::new();

    for c in i.chars() {
        if c != '0' && c != '1' {
            println!("Invalid input");
            main();
        } else {
            match c {
                '0' => on.push('1'),
                '1' => on.push('0'),
                _ => main(),
                
            }
        }
    }

    let mut on: Vec<char> = on.chars().collect();
    let l = on.len();

    for i in 0..on.len() {
        if on[l - 1- i] == '0' {
            on[l - 1 - i] = '1';
            break;
        } else {
            on[l - 1 - i] = '0';
            if i + 1 == l {
                on.insert(0, '1');
            }
        }
    }

    println!("{:?}", on);
    main()
}
