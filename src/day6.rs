const INPUT: &str = include_str!("../inputs/day6.txt");

enum Symbol {
	Add,
	Mul,
}

impl Symbol {
	fn new(byte: u8) -> Self {
		match byte {
			b'+' => Symbol::Add,
			b'*' => Symbol::Mul,
			_ => panic!("Invalid symbol byte: {}", byte as char),
		}
	}

	fn calc(&self, nums: impl Iterator<Item = u64>) -> u64 {
		match self {
			Symbol::Add => nums.sum(),
			Symbol::Mul => nums.product(),
		}
	}
}

struct Problems {
	numbers: Vec<Vec<u64>>,
	symbols: Vec<Symbol>,
}

impl Problems {
	fn solutions(&self) -> impl Iterator<Item = u64> {
		self.symbols
			.iter()
			.enumerate()
			.map(|(i, symb)| symb.calc(self.numbers.iter().map(|nums| nums[i])))
	}
}

fn parse_input(input: &str) -> Problems {
	let mut numbers = Vec::new();
	let mut symbols = Vec::new();

	let mut iter = input.trim().lines();

	for line in &mut iter {
		let line = line.trim();

		if line.as_bytes()[0].is_ascii_digit() {
			numbers.push(
				line.split_ascii_whitespace()
					.map(|n| n.parse().unwrap())
					.collect(),
			);
		} else {
			assert!(symbols.is_empty());

			symbols = line
				.split_ascii_whitespace()
				.map(|b| Symbol::new(b.as_bytes()[0]))
				.collect();
		}
	}

	Problems { numbers, symbols }
}

fn part1() -> u64 {
	let problems = parse_input(INPUT);

	problems.solutions().sum()
}

fn main() {
	let p1 = part1();
	println!("Part 1: {p1}");
}
