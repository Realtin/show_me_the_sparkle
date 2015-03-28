use std::old_io::File;

fn main() {
    let path = Path::new("list.txt");
    let mut file = File::open(&path);

    // returns Vector of Bytes (Vec<u8>)
    let data = file.read_to_end().unwrap();

    // To print the contents of the file
    let filestr = String::from_utf8(data).unwrap();
    println!("{}", filestr);
}
