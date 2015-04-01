use std::old_io::File;
use std::rand::random;
use std::rand::{thread_rng, Rng};




fn main() {
    let path = Path::new("list.txt");
    let mut file = File::open(&path);

    // returns Vector of Bytes (Vec<u8>)
    let data = file.read_to_end().unwrap();

    // To print the contents of the file
    let filestr = String::from_utf8(data).unwrap();

    // put every line in an array
    let mut result: Vec<String> = Vec::new();
    for line in filestr.lines_any() {
        result.push(line.to_string());
    }

    //get random line from the array
    let mut rng = thread_rng();

    //using random number
    let m: usize = rng.gen_range(0, result.len());
    println!("{}", result[m]);

    //using array shuffle
    rng.shuffle(&mut result);
    println!("{}", result[0]);

}
