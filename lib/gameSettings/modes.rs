use std::io;

pub fn getMode() {
    let mut diff: String = String::new();
    println!("What difficulty do you want? (1: Easy, 2: Medium, 3: Hard, 4: Insane.)");
    io::stdin()
        .read_line(&mut diff)
        .expect("Uh oh.");

    let diff: u32 = diff.trim().parse().expect("You didnt enter a number.");

    match diff {
        1 => return 101,
        2 => return 251,
        3 => return 501,
        4 => return 1001,
    }
    println!("You didnt enter a correct number.");
}