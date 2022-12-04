fn sum_calories(lines: core::str::Split<char>) -> Vec<u32> {
    let mut totals =Vec::new();
    let mut sum = 0;
    for line in lines {
        if line.is_empty() {
            totals.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap();     
        }
    }
    totals
}
    
fn main() {
    let mut totals = sum_calories(include_str!("input.txt").split('\n'));
    totals.sort_unstable();
    println!("Part 1: {}", totals.last().unwrap());
    println!("Part 2: {:?}", &totals[totals.len() - 3..]);
}