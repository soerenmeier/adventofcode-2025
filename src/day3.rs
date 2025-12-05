use std::slice;

const INPUT: &str = include_str!("../inputs/day3.txt");

struct DigitIter<'a> {
	numbers: slice::Iter<'a, u8>,
}

impl<'a> DigitIter<'a> {
	fn new(numbers: &'a [u8]) -> Self {
		Self {
			numbers: numbers.iter(),
		}
	}
}

impl Iterator for DigitIter<'_> {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		let num = self.numbers.next()?;
		assert!(num.is_ascii_digit());

		Some(num - b'0')
	}
}

struct BatteryBank<'a> {
	// numbers in bytes not actual number
	numbers: &'a [u8],
}

impl<'a> BatteryBank<'a> {
	fn new(numbers: &'a str) -> BatteryBank<'a> {
		BatteryBank {
			numbers: numbers.as_bytes(),
		}
	}

	fn largest_jolt(&self) -> u32 {
		let mut iter = DigitIter::new(self.numbers);

		let mut d1 = iter.next().unwrap() as u32;
		let mut d2 = iter.next().unwrap() as u32;

		for d in iter {
			let d = d as u32;

			if d2 > d1 {
				d1 = d2;
				d2 = d;
			} else if d > d2 {
				d2 = d;
			}
		}

		d1 * 10 + d2
	}
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = BatteryBank<'a>> {
	input.trim().lines().map(|l| BatteryBank::new(l.trim()))
}

fn part1() -> u32 {
	let banks = parse_input(INPUT);

	banks.map(|b| b.largest_jolt()).sum()
}

fn main() {
	let p1 = part1();
	println!("Part 1: {p1}");
}

/*
   In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
   In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
   In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
   In 818181911112111, the largest joltage you can produce is 92.
*/
#[test]
fn test_p1() {
	assert_eq!(BatteryBank::new("987654321111111").largest_jolt(), 98);
	assert_eq!(BatteryBank::new("811111111111119").largest_jolt(), 89);
	assert_eq!(BatteryBank::new("234234234234278").largest_jolt(), 78);
	assert_eq!(BatteryBank::new("818181911112111").largest_jolt(), 92);
}
