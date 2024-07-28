// https://codeforces.com/problemset/problem/231/A
use std::io::BufRead;

fn read_line() -> Result<String, std::io::Error> {
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = read_line()?.trim().parse::<i32>()?;
    let result = (0..n)
        .into_iter()
        .map(|_| {
            read_line()
                .expect("-")
                .split(" ")
                .map(|s| s.parse::<i32>().expect("-"))
                .fold(0, |r, v| r + v)
        })
        .filter(|v| *v > 1)
        .count();
    println!("{}", result);
    Ok(())
}
