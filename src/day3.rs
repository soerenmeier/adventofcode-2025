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

	fn largest_jolt<const S: usize>(&self) -> u64 {
		let mut iter = DigitIter::new(self.numbers);

		let mut arr = [0u8; S];

		for d in &mut arr {
			*d = iter.next().unwrap();
		}

		'main: for d in iter {
			// check if we can shift any values in the array
			for i in 0..(S - 1) {
				if arr[i + 1] > arr[i] {
					arr[i..].rotate_left(1);
					arr[S - 1] = d;
					continue 'main;
				}
			}

			if d > arr[S - 1] {
				arr[S - 1] = d;
			}
		}

		arr.iter().fold(0, |acc, d| acc * 10 + *d as u64)
	}
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = BatteryBank<'a>> {
	input.trim().lines().map(|l| BatteryBank::new(l.trim()))
}

fn part1() -> u64 {
	let banks = parse_input(INPUT);

	banks.map(|b| b.largest_jolt::<2>()).sum()
}

fn part2() -> u64 {
	let banks = parse_input(INPUT);

	banks.map(|b| b.largest_jolt::<12>()).sum()
}

fn main() {
	let p1 = part1();
	println!("Part 1: {p1}");
	assert_eq!(p1, 17321);

	let p2 = part2();
	println!("Part 2: {p2}");
	assert_eq!(p2, 171989894144198);
}

/*
   In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
   In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
   In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
   In 818181911112111, the largest joltage you can produce is 92.
*/
#[test]
fn test_p1() {
	assert_eq!(BatteryBank::new("987654321111111").largest_jolt::<2>(), 98);
	assert_eq!(BatteryBank::new("811111111111119").largest_jolt::<2>(), 89);
	assert_eq!(BatteryBank::new("234234234234278").largest_jolt::<2>(), 78);
	assert_eq!(BatteryBank::new("818181911112111").largest_jolt::<2>(), 92);
}

/*
   In 987654321111111, the largest joltage can be found by turning on everything except some 1s at the end to produce 987654321111.
   In the digit sequence 811111111111119, the largest joltage can be found by turning on everything except some 1s, producing 811111111119.
   In 234234234234278, the largest joltage can be found by turning on everything except a 2 battery, a 3 battery, and another 2 battery near the start to produce 434234234278.
   In 818181911112111, the joltage 888911112111 is produced by turning on everything except some 1s near the front.
*/
#[test]
fn test_p2() {
	assert_eq!(
		BatteryBank::new("987654321111111").largest_jolt::<12>(),
		987654321111
	);
	assert_eq!(
		BatteryBank::new("811111111111119").largest_jolt::<12>(),
		811111111119
	);
	assert_eq!(
		BatteryBank::new("234234234234278").largest_jolt::<12>(),
		434234234278
	);
	assert_eq!(
		BatteryBank::new("818181911112111").largest_jolt::<12>(),
		888911112111
	);
}
