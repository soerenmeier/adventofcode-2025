const INPUT: &str = include_str!("../inputs/day6.txt");

#[derive(Debug)]
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

#[derive(Debug)]
struct ProblemsP1 {
	numbers: Vec<Vec<u64>>,
	symbols: Vec<Symbol>,
}

impl ProblemsP1 {
	fn solutions(&self) -> impl Iterator<Item = u64> {
		self.symbols
			.iter()
			.enumerate()
			.map(|(i, symb)| symb.calc(self.numbers.iter().map(|nums| nums[i])))
	}
}

fn parse_input_p1(input: &str) -> ProblemsP1 {
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

	ProblemsP1 { numbers, symbols }
}

fn part1() -> u64 {
	let problems = parse_input_p1(INPUT);

	problems.solutions().sum()
}

#[derive(Debug)]
struct ProblemsP2<'a> {
	widths: Vec<usize>,
	numbers: Vec<Vec<&'a [u8]>>,
	symbols: Vec<Symbol>,
}

impl<'a> ProblemsP2<'a> {
	fn solutions(&self) -> impl Iterator<Item = u64> {
		self.symbols.iter().zip(self.widths.iter()).enumerate().map(
			|(i, (symbol, width))| {
				let nums = self.numbers.iter().map(|nums| nums[i]);

				let final_nums = (0..*width).map(|i| {
					nums.clone()
						.filter_map(|n| {
							let num = n[i];
							num.is_ascii_digit().then(|| num - b'0')
						})
						.fold(0u64, |a, b| a * 10 + b as u64)
				});

				symbol.calc(final_nums)
			},
		)
	}
}

fn parse_input_p2<'a>(input: &'a str) -> ProblemsP2<'a> {
	let input = input.trim_matches('\n');

	let mut widths = Vec::new();
	let mut symbols = Vec::new();

	let last_line = input.lines().rev().next().unwrap();

	let mut curr_width = 0;
	for byte in last_line.as_bytes() {
		if !byte.is_ascii_whitespace() {
			symbols.push(Symbol::new(*byte));
			if curr_width != 0 {
				// -1 because there is always a whitespace between symbols
				// +1 because the symbol itself takes one space
				widths.push(curr_width - 1 + 1);
				curr_width = 0;
			}
		} else {
			curr_width += 1;
		}
	}
	// +1 for the symbol
	widths.push(curr_width + 1);

	let mut numbers: Vec<Vec<_>> = Vec::new();

	for line in input.lines() {
		if !line.trim_start().as_bytes()[0].is_ascii_digit() {
			break;
		}

		let line = line.as_bytes();

		let mut idx = 0;
		numbers.push(
			widths
				.iter()
				.map(|a| {
					let num = &line[idx..(idx + a)];
					idx += a + 1;
					num
				})
				.collect(),
		);
	}

	ProblemsP2 {
		widths,
		numbers,
		symbols,
	}
}

fn part2() -> u64 {
	let problems = parse_input_p2(INPUT);

	problems.solutions().sum()
}

fn main() {
	let p1 = part1();
	println!("Part 1: {p1}");
	assert_eq!(p1, 4309240495780);

	let p2 = part2();
	println!("Part 2: {p2}");
	assert_eq!(p2, 9170286552289);
}

#[test]
fn test_p2() {
	#[rustfmt::skip]
	let problems = parse_input_p2("123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ");

	let mut solutions = problems.solutions();
	assert_eq!(solutions.next(), Some(8544));
	assert_eq!(solutions.next(), Some(625));
	assert_eq!(solutions.next(), Some(3253600));
	assert_eq!(solutions.next(), Some(1058));
	assert_eq!(solutions.next(), None);
}
