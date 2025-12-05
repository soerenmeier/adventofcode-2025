use std::ops::Range;

const INPUT: &str = include_str!("../inputs/day5.txt");

struct FreshIngredients {
	inner: Vec<Range<u64>>,
}

impl FreshIngredients {
	fn count_fresh(&self, ids: impl Iterator<Item = u64>) -> u64 {
		ids.filter(|id| self.is_fresh(*id)).count() as u64
	}

	fn is_fresh(&self, id: u64) -> bool {
		self.inner.iter().any(|r| r.contains(&id))
	}
}

fn parse_input(input: &str) -> (FreshIngredients, impl Iterator<Item = u64>) {
	let mut lines = input.trim().lines();

	let mut fresh = Vec::new();

	for line in &mut lines {
		// fresh is done
		if line.is_empty() {
			break;
		}

		let mut parts = line.split('-');

		fresh.push(Range {
			start: parts.next().unwrap().parse::<u64>().unwrap(),
			end: parts.next().unwrap().parse::<u64>().unwrap() + 1,
		});
	}

	(
		FreshIngredients { inner: fresh },
		lines.map(|l| l.parse::<u64>().unwrap()),
	)
}

fn part1() -> u64 {
	let (fresh, available) = parse_input(INPUT);

	fresh.count_fresh(available)
}

fn main() {
	let p1 = part1();
	println!("Part 1: {p1}");
	assert_eq!(p1, 513);
}

#[test]
fn test_part1() {
	let (fresh, available) = parse_input(
		"\
3-5
10-14
16-20
12-18

1
5
8
11
17
32",
	);

	assert_eq!(fresh.count_fresh(available), 3);
}
