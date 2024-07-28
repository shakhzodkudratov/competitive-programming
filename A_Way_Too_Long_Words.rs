// https://codeforces.com/problemset/problem/71/A
use std::io::BufRead;

fn read_line() -> Result<String, std::io::Error> {
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = read_line()?.trim().parse::<i32>()?;
    (0..n)
        .into_iter()
        .map(|_| read_line().expect("-"))
        .map(|s| {
            if s.len() > 10 {
                format!(
                    "{}{}{}",
                    s.chars().next().unwrap(),
                    s.len() - 2,
                    s.chars().next_back().unwrap()
                )
            } else {
                s
            }
        })
        .for_each(|s| println!("{}", s));
    Ok(())
}
