pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let line = read_line();
    let mut iter = line.split_whitespace();
    let r: f32 = iter.next().unwrap().parse().unwrap();
    let c: f32 = iter.next().unwrap().parse().unwrap();

    let c_r = r - c;

    println!("{:.6}", 100. * (c_r * c_r) / (r * r));
}
