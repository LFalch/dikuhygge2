pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let line = read_line();
    let mut iter = line.split_whitespace();
    let _n: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();

    let line = read_line();
    let mut items: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    items.sort_unstable();

    let len = items.len();

    let mut max_count = 0;
    for i in 0..items.len() {
        let mut count = 0;
        let mut max = 0;
        for &item in &items[i..] {
            if max + item > x {
                break
            } else {
                count += 1;
                max = item;
            }
        }

        max_count = count.max(max_count);
    }

    if len == 1 || len == 0 {
        max_count = len;
    }
    println!("{}", max_count);
}
