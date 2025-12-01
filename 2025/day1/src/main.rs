use std::{fs::read_to_string, io};
fn main() -> Result<(), io::Error> {
    let input = read_to_string("input.txt")?
        .split("\n")
        .map(|e| e.to_string())
        .filter(|e| e != "")
        .collect::<Vec<_>>();

    let rots: Vec<i32> = input.iter().map(|w| w.split_at(1)).map(|p| 
        match p.0 {
            "L" => { -p.1.parse::<i32>().unwrap() },
            "R" => { p.1.parse::<i32>().unwrap() },
            _ => { panic!("Shouldn't happen, not LR"); }
        }).collect();

    let mut pointer = 50;
    let mut total1 = 0;

    for rotation in &rots {
        pointer = (pointer + rotation) % 100;
        if pointer < 0 {
            pointer = 100 + pointer;
        }


        if pointer == 0 {
            total1 += 1;
        }
    }

    let mut pointer = 50;
    let mut total2 = 0;

    for rotation in rots {
        for _ in 0..rotation.abs() {
            if rotation > 0 {
                pointer += 1;
            } else {
                pointer -= 1;
            }

            if pointer == -1 {
                pointer = 99;
            }

            if pointer == 100 {
                pointer = 0;
            }

            if pointer == 0 {
                total2 += 1;
            }
        }
    }

    println!("total1: {} total2: {}", total1, total2);

    Ok(())
}
