const INPUT: &str = include_str!("../inputs/day7.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
	Empty,
	Start,
	Splitter,
	Tachion,
}

impl Symbol {
	fn new(byte: &u8) -> Self {
		match byte {
			b'.' => Symbol::Empty,
			b'S' => Symbol::Start,
			b'^' => Symbol::Splitter,
			b'|' => Symbol::Tachion,
			_ => panic!("Unknown symbol: {}", *byte as char),
		}
	}
}

#[derive(Debug)]
struct Map {
	width: usize,
	height: usize,
	cells: Vec<Symbol>,
}

impl Map {
	// runs the tachion simulation
	fn run(&mut self) {
		for y in 1..self.height {
			for x in 0..self.width {
				let up = self.cells[(y - 1) * self.width + x];
				let curr = &mut self.cells[y * self.width + x];
				match (up, *curr) {
					(Symbol::Start | Symbol::Tachion, Symbol::Splitter) => {
						if let Some(left) = (x > 0)
							.then(|| &mut self.cells[y * self.width + x - 1])
						{
							*left = Symbol::Tachion;
						}
						if let Some(right) = (x + 1 < self.width)
							.then(|| &mut self.cells[y * self.width + x + 1])
						{
							*right = Symbol::Tachion;
						}
					}
					(Symbol::Start | Symbol::Tachion, _) => {
						*curr = Symbol::Tachion
					}
					_ => {}
				}
			}
		}
	}

	fn count_splits(&self) -> u64 {
		(1..self.height)
			.map(|y| {
				(0..self.width)
					.filter(|x| {
						let prev = self.cells[(y - 1) * self.width + x];
						let curr = self.cells[y * self.width + x];

						matches!(
							(prev, curr),
							(Symbol::Tachion, Symbol::Splitter)
						)
					})
					.count() as u64
			})
			.sum()
	}
}

fn parse_input(input: &str) -> Map {
	let mut width = 0;

	let cells = input
		.trim()
		.lines()
		.map(|l| {
			if width != l.len() {
				assert_eq!(width, 0);
				width = l.len();
			}

			l.as_bytes().iter().map(|b| Symbol::new(b))
		})
		.flatten()
		.collect::<Vec<_>>();

	Map {
		width,
		height: cells.len() / width,
		cells,
	}
}

fn part1() -> u64 {
	let mut map = parse_input(INPUT);
	map.run();
	map.count_splits()
}

fn main() {
	let p1 = part1();
	println!("Part 1: {p1}");
	assert_eq!(p1, 1635);
}
