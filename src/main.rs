fn main() {
    let chr1: char = '1';
    let chr2: char = '2';
    let chr3: char = '3';

    let mut sum: u16 = 0;

    for chr in [chr1, chr2, chr3] {
        sum += chr.to_digit(10).unwrap() as u16;
    }
    println!("{}", sum);
}