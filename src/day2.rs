use std::collections::BTreeSet;

const INPUT: &str = include_str!("../inputs/day2.txt");

#[derive(Debug)]
struct PatternRepeater {
	// ex: 1, 10, 100
	pat: u64,
	digits: u32,
	repeat: u32,
	finished: bool,
}

impl PatternRepeater {
	fn new(pat: u64, repeat: u32) -> Self {
		Self {
			pat,
			digits: pat.ilog10() + 1,
			repeat,
			finished: false,
		}
	}

	fn add(&mut self) {
		let n_pat = self.pat + 1;
		if n_pat >= 10u64.pow(self.digits) {
			self.finished = true;
		} else {
			self.pat = n_pat;
		}
	}

	fn value(&self) -> u64 {
		let mut val = self.pat;

		for _ in 1..self.repeat {
			val = val * 10u64.pow(self.digits) + self.pat;
		}

		val
	}
}

impl Iterator for PatternRepeater {
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item> {
		if self.finished {
			return None;
		}

		let val = self.value();
		self.add();
		Some(val)
	}
}

#[derive(Debug, Clone, Copy)]
struct IdRange {
	start: u64,
	end: u64,
}

impl IdRange {
	fn invalid_ids(&self) -> BTreeSet<u64> {
		let start_digits = self.start.ilog10() + 1;
		let end_digits = self.end.ilog10() + 1;

		// / 2 because we need to repeat at least twice
		let min_pat_len = (start_digits) / 2;
		let max_pat_len = (end_digits) / 2;

		let mut invalid_ids = BTreeSet::new();

		for pat_len in min_pat_len..=max_pat_len {
			if pat_len == 0 {
				continue;
			}

			let mut pat = 10u64.pow(pat_len - 1);

			// in the first pat_len
			// the pattern must be bigger then start
			if pat_len == min_pat_len {
				pat = self.start / 10u64.pow(pat_len);
			}

			let mut repeater = PatternRepeater::new(pat, 2);

			for pattern in &mut repeater {
				if pattern > self.end {
					break;
				}

				if pattern >= self.start {
					invalid_ids.insert(pattern);
				}
			}
		}

		invalid_ids
	}
}

fn parse_input(input: &str) -> impl Iterator<Item = IdRange> {
	input.trim().split(',').map(|r| {
		let mut parts = r.trim().split('-');
		IdRange {
			start: parts.next().unwrap().parse().unwrap(),
			end: parts.next().unwrap().parse().unwrap(),
		}
	})
}

fn part1() -> u64 {
	let input = parse_input(INPUT);

	input.map(|r| r.invalid_ids().iter().sum::<u64>()).sum()
}

fn main() {
	let p1 = part1();
	println!("Part 1: {p1}");
	assert_eq!(p1, 19219508902);
}

/*
   11-22 has two invalid IDs, 11 and 22.
   95-115 has one invalid ID, 99.
   998-1012 has one invalid ID, 1010.
   1188511880-1188511890 has one invalid ID, 1188511885.
   222220-222224 has one invalid ID, 222222.
   1698522-1698528 contains no invalid IDs.
   446443-446449 has one invalid ID, 446446.
   38593856-38593862 has one invalid ID, 38593859.
   The rest of the ranges contain no invalid IDs.

   565653-565659,
   824824821-824824827,2121212118-2121212124
*/
#[test]
fn test_invalid_ids() {
	macro_rules! check_ids {
		($start:expr, $end:expr, [$($val:expr),*]) => {
			{
				let range = IdRange { start: $start, end: $end };
				#[allow(unused_mut)]
				let mut invalid_ids = range.invalid_ids();
				$(
					assert!(invalid_ids.remove(&$val), "Expected to find invalid ID: {}", $val);
				)*

				assert_eq!(invalid_ids.len(), 0);
			}
		};
	}

	check_ids!(11, 22, [11, 22]);
	check_ids!(95, 115, [99]);
	check_ids!(998, 1012, [1010]);
	check_ids!(1188511880, 1188511890, [1188511885]);
	check_ids!(222220, 222224, [222222]);
	check_ids!(1698522, 1698528, []);
	check_ids!(446443, 446449, [446446]);
	check_ids!(38593856, 38593862, [38593859]);
	check_ids!(565653, 565659, []);
	check_ids!(824824821, 824824827, []);
	check_ids!(2121212118, 2121212124, []);
}
