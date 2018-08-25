use std::io;

fn main() {

    let mut x = String::new();
    loop {
        io::stdin().read_line(&mut x)
            .expect("?");
        println!("?");
    }
}
