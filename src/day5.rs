use std::{collections::VecDeque, mem, ops::Range};

const INPUT: &str = include_str!("../inputs/day5.txt");

struct FreshIngredients {
	inner: Vec<Range<u64>>,
	dedup: bool,
}

impl FreshIngredients {
	fn dedup(&mut self) {
		let mut nv = VecDeque::<Range<u64>>::with_capacity(self.inner.len());

		let mut ranges = mem::take(&mut self.inner);

		'range: while let Some(mut r) = ranges.pop() {
			for i in 0..nv.len() {
				let nr = nv[i].clone();

				if r.start <= nr.start {
					if r.end <= nr.start {
						// we are completely before
						nv.insert(i, r);
						continue 'range;
					}

					// we overlap at the start
					nv[i].start = r.start;

					if r.end <= nr.end {
						// we are completely inside
						continue 'range;
					}

					ranges.push(Range {
						start: nr.end,
						end: r.end,
					});

					continue 'range;
				}

				assert!(r.start > nr.start);

				if r.start < nr.end {
					if r.end <= nr.end {
						// we are completely inside
						continue 'range;
					}

					// we overlap at the end
					r.start = nr.end;
				}
			}

			nv.push_back(r);
		}

		// todo we could reduce the ranges
		self.inner = nv.into_iter().collect();
		self.dedup = true;
	}

	fn count_fresh(&self) -> u64 {
		assert!(self.dedup);

		self.inner.iter().map(|r| r.end - r.start).sum()
	}

	fn count_fresh_from_ids(&self, ids: impl Iterator<Item = u64>) -> u64 {
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
		FreshIngredients {
			inner: fresh,
			dedup: false,
		},
		lines.map(|l| l.parse::<u64>().unwrap()),
	)
}

fn part1() -> u64 {
	let (fresh, available) = parse_input(INPUT);

	fresh.count_fresh_from_ids(available)
}

fn part2() -> u64 {
	let (mut fresh, _) = parse_input(INPUT);

	fresh.dedup();
	fresh.count_fresh()
}

fn main() {
	let p1 = part1();
	println!("Part 1: {p1}");
	assert_eq!(p1, 513);

	let p2 = part2();
	println!("Part 2: {p2}");
	assert_eq!(p2, 339668510830757);
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

	assert_eq!(fresh.count_fresh_from_ids(available), 3);
}

#[test]
fn test_part2() {
	let (mut fresh, _) = parse_input(
		"\
3-5
10-14
16-20
12-18",
	);

	fresh.dedup();
	eprintln!("Fresh ranges: {:?}", fresh.inner);
	assert_eq!(fresh.count_fresh(), 14);
}

#[test]
fn test_overlap_check() {
	let (mut fresh, _) = parse_input(
		"\
5-15
15-20
0-5
10-20
4-21
10-25
30-40
",
	);

	fresh.dedup();
	assert_eq!(fresh.count_fresh(), 37);
}
