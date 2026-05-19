fn main() {
    let chrs: [char; 5] = ['1', '2', '3', '4', '5'];

    let mut txt: String = String::with_capacity(5);

    for chr in chrs {
        txt.push(chr);
    }

    let num: u32 = txt.parse().unwrap();

    println!("{}", num);

}
