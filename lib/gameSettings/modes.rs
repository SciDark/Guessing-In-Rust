use std::io;

pub fn get_mode() -> u32{
    let mut diff: String = String::new();
    println!("What difficulty do you want? (1: Easy, 2: Medium, 3: Hard, 4: Insane.)");
    io::stdin()
        .read_line(&mut diff)
        .expect("Uh oh.");
    let diff: u32 = diff.trim().parse().expect("You didnt enter a number.");

    return diff;
}