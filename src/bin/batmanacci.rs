pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn batman()

fn main() {
    let line = read_line();
    let mut iter = line.split_whitespace();
    let n: u64 = iter.next().unwrap().parse().unwrap();
    let k: u64 = iter.next().unwrap().parse().unwrap();
}
