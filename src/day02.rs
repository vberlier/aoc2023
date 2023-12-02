#![allow(dead_code)]

const INPUT: &str = include_str!("day02.txt");

fn parse_game<'a>(s: &'a str) -> Option<(&'a str, impl Iterator<Item = [u32; 3]> + 'a)> {
    let (prefix, game) = s.split_once(":")?;
    let it = game.split([';', ',']).filter_map(|s| {
        let (n, color) = s[1..].split_once(' ')?;
        let n = n.parse::<u32>().ok()?;
        match color {
            "red" => Some([n, 0, 0]),
            "green" => Some([0, n, 0]),
            "blue" => Some([0, 0, n]),
            _ => None,
        }
    });
    Some((prefix, it))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let solution = INPUT
            .lines()
            .filter_map(|s| {
                let (prefix, mut cubes) = parse_game(s)?;
                if cubes.all(|[r, g, b]| r <= 12 && g <= 13 && b <= 14) {
                    prefix.split_once(' ')?.1.parse::<u32>().ok()
                } else {
                    None
                }
            })
            .sum::<u32>();
        insta::assert_snapshot!(solution.to_string(), @"2679");
    }

    #[test]
    fn part2() {
        let solution = INPUT
            .lines()
            .filter_map(|s| {
                let mut rgb = [0, 0, 0];
                for cube in parse_game(s)?.1 {
                    for (x, n) in rgb.iter_mut().zip(cube) {
                        *x = (*x).max(n)
                    }
                }
                Some(rgb.iter().product::<u32>())
            })
            .sum::<u32>();
        insta::assert_snapshot!(solution.to_string(), @"77607");
    }
}
