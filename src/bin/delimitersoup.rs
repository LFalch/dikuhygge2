pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let _ = read_line();
    let line = read_line();
    let mut opening_delmiters = Vec::new();

    let mut error = None;

    for (i, c) in line.char_indices() {
        match c {
            '(' => {
                opening_delmiters.push(c);
            }
            '[' => {
                opening_delmiters.push(c);
            }
            '{' => {
                opening_delmiters.push(c);
            }
            ')' => {
                if opening_delmiters.pop() != Some('(') {
                    error = Some((c, i));
                    break;
                }
            }
            ']' => {
                if opening_delmiters.pop() != Some('[') {
                    error = Some((c, i));
                    break;
                }
            }
            '}' => {
                if opening_delmiters.pop() != Some('{') {
                    error = Some((c, i));
                    break;
                }
            }
            _ => (),
        }
    }

    match error {
        Some((c, i)) => println!("{} {}", c, i),
        None => println!("ok so far"),
    }
}
