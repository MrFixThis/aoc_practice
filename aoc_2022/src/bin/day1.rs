use anyhow::Result;
use aoc_2022::read_vec_by_sep;
use itertools::Itertools;

// Part I
fn max_cal() -> Result<usize> {
    Ok(read_vec_by_sep::<usize>("day1_input.txt", "\n\n")?
        .into_iter()
        .map(|v| v.into_iter().sum())
        .max()
        .unwrap())
}

// Part II
fn max_cal_top_3() -> Result<usize> {
    Ok(read_vec_by_sep::<usize>("day1_input.txt", "\n\n")?
        .into_iter()
        .map(|v| v.into_iter().sum::<usize>())
        .sorted_by(|a, b| b.cmp(&a))
        .take(3)
        .sum())
}

fn main() -> Result<()> {
    println!("Part I: {}\nPart II: {}", max_cal()?, max_cal_top_3()?);
    Ok(())
}
