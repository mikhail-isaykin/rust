fn main() {
    let chr1: char = 'a';
    let chr2: char = 'b';
    let chr3: char = 'c';

    let mut txt: String = chr1.to_string();

    txt.push(chr2);
    txt.push(chr3);

    println!("{}", txt);
}