pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let line = read_line();
    let mut last_char = ' ';
    let mut output = String::with_capacity(line.len());

    for c in line.chars() {
        if c != last_char {
            last_char = c;
            output.push(c);
        }
    }
    println!("{}", output);
}
