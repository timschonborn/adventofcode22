
#![allow(dead_code, unused_variables)]

fn part1(s: &str) -> u32 {
    let mut points = 0;
    for line in s.lines() {
        // split to put first part in hashset and second part for lookup
        let (a, b) = line.split_at(line.len() / 2);
        let hs = a.chars().collect::<std::collections::HashSet<_>>();
        for c in b.chars() {
            if hs.contains(&c) {
                if c.is_lowercase() {
                    // 'a' is at pos 97 in ASCII
                    points += c as u32 - 96;
                }
                else {
                    // 'A' is at pos 65 in ASCII + 26 to skip lowercase points
                    // -64 + 26 = -38
                    points += c as u32 - 38;
                }
             
                break;
            }
        }
    }
    points
} 

fn part2(s: &str) -> u32 {
    let lines = s.lines().collect::<Vec<_>>(); // collect to avoid double iteration

    for idx in (0..lines.len()).step_by(3) {
        let a = lines[idx].chars().collect::<std::collections::HashSet<_>>();
        let b = lines[idx].chars().collect::<std::collections::HashSet<_>>();
        let c = lines[idx].chars().collect::<std::collections::HashSet<_>>();
        let inter = a.intersection(&b).collect::<std::collections::HashSet<_>>();
    }

    todo!()
}

fn main() {
    // read input.txt
    let s = include_str!("input3.txt");
    println!("Part 1: {}", part1(s));

}
