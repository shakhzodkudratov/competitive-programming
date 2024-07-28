// https://codeforces.com/problemset/problem/4/A
use std::io::BufRead;

fn read_line() -> Result<String, std::io::Error> {
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_line()?.trim().parse::<i32>()?;
    let answer = if input > 2 && input % 2 == 0 {
        "YES"
    } else {
        "NO"
    };
    print!("{}", answer);
    Ok(())
}
