fn get_number(n: u8) -> Option<u8> {
    return if n % 2 == 0 {
        Some(n)
    } else {
        None
    };
}

fn main() {
    println!("{:?}",get_number(7).unwrap_or(1))
}
