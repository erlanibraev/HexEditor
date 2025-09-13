use rand::Rng;
fn main() {
    println!("Hello, world!");
    let mut buff: Vec<[u8; 16]> = Vec::new();
    buff.push(random_bytes16());
    buff.push(random_bytes16());
    buff.push(random_bytes16());
    print_buff(buff, 0)
}

fn print_buff(buff: Vec<[u8; 16]>, start: u64) {
    let mut address: u64 = start;
    let start_index = (start / 16) as usize;
    let end_index = start_index + 25;
    for i in start_index..end_index {
        if(i < buff.len()) {
            let row: &[u8; 16] = &buff[i];
            print!("{:08X?}: ", address);

            for j in 0..15 {
                print!("{:02X?} ", &row[j])
            }

            print!(" - ");

            for j in 0..15 {
                print!("{}", char::from(row[j]))
            }
        }
        println!("");

        address = address + 16
    }
}
fn random_bytes16() -> [u8; 16] {
    let mut bytes = [0u8; 16];
    for i in 0..16 {
        bytes[i] = rand::random::<u8>();
    }
    bytes
}
