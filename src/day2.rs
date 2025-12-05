use std::collections::BTreeSet;

const INPUT: &str = include_str!("../inputs/day2.txt");

#[derive(Debug)]
struct PatternRepeater {
	// ex: 1, 10, 100
	pat: u64,
	digits: u32,
	repeat: u32,
}

impl PatternRepeater {
	fn new(pat: u64, repeat: u32) -> Self {
		Self {
			pat,
			digits: pat.ilog10() + 1,
			repeat,
		}
	}

	fn add(&mut self) {
		let n_pat = self.pat + 1;
		if n_pat >= 10u64.pow(self.digits) {
			self.repeat += 1;
			self.pat = 10u64.pow(self.digits - 1);
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
		let max_pat_len = (end_digits) / 2;

		eprintln!(
			"checking range: {}-{}, max pattern length: {}",
			self.start, self.end, max_pat_len
		);

		let mut invalid_ids = BTreeSet::new();

		let pat_len = max_pat_len;

		// for pat_len in 1..=max_pat_len {
		// if the pattern will never be able
		// to fit in the range, skip it
		// if start_digits % pat_len != 0 && end_digits % pat_len != 0 {
		// 	continue;
		// }

		let pat = 10u64.pow(pat_len - 1);
		// todo we could do some optimization here
		// to avoid checking patterns that are to low

		let mut repeater = PatternRepeater::new(pat, start_digits / pat_len);

		for pattern in &mut repeater {
			if pattern > self.end {
				break;
			}

			eprintln!("patt: {pattern}");

			if pattern >= self.start {
				eprintln!("found invalid ID: {pattern}");
				invalid_ids.insert(pattern);
			}
		}
		// }

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

	input.map(|r| r.invalid_ids().len() as u64).sum()
}

fn main() {
	let p1 = part1();
	println!("Part 1: {p1}");
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
	fn new(start: u64, end: u64) -> IdRange {
		IdRange { start, end }
	}

	assert_eq!(new(11, 22).invalid_ids().len(), 2);
	assert_eq!(new(95, 115).invalid_ids().len(), 1);
	assert_eq!(new(998, 1012).invalid_ids().len(), 1);
	assert_eq!(new(1188511880, 1188511890).invalid_ids().len(), 1);
	assert_eq!(new(222220, 222224).invalid_ids().len(), 1);
	assert_eq!(new(1698522, 1698528).invalid_ids().len(), 0);
	assert_eq!(new(446443, 446449).invalid_ids().len(), 1);
	assert_eq!(new(38593856, 38593862).invalid_ids().len(), 1);
	assert_eq!(new(565653, 565659).invalid_ids().len(), 0);
	assert_eq!(new(824824821, 824824827).invalid_ids().len(), 0);
	assert_eq!(new(2121212118, 2121212124).invalid_ids().len(), 0);
}
