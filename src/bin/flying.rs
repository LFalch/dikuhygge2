pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let n = read_line().trim().parse().unwrap();

    for _ in 0..n {
        let line = read_line();
        let mut iter = line.split_whitespace();
        let n: u64 = iter.next().unwrap().parse().unwrap();
        let m: u64 = iter.next().unwrap().parse().unwrap();

        for _ in 0..m {
            read_line();
        }

        println!("{}", n - 1);
    }
}
