use std::{fs::read_to_string, io};
fn main() -> Result<(), io::Error> {
    let input = read_to_string("input.txt")?
        .split("\n")
        .map(|e| e.to_string())
        .filter(|e| e != "")
        .collect::<Vec<_>>();

    let mut total = 0;

    for mut line in input {
        let mut idx = 11;
        let mut buf = String::with_capacity(12);
        while idx != 0 {
            let (left, right) = line.split_at(line.len() - idx);

            let max = left.chars().max().unwrap();
            idx -= 1;
            println!("{max} at slot {}: {left}:{right}", line.chars().position(|e| e == max).unwrap());

            buf.push_str(&format!("{}", max));

            let mut new = left.split_at(left.chars().position(|e| e == max).unwrap() + 1).1.to_string();
            new.push_str(right);
            line = new;
        }
        buf.push(line.chars().max().unwrap());
        println!("adding {buf} left is: {line}");
        
        total += buf.parse::<u128>().unwrap();
        
    }


    println!("total: {total}");

    Ok(())
}
