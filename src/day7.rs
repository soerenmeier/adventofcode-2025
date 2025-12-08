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
	tachion_count: Vec<u64>,
}

impl Map {
	// runs the tachion simulation
	fn run(&mut self) {
		for y in 1..self.height {
			for x in 0..self.width {
				let up_idx = (y - 1) * self.width + x;
				let up = self.cells[up_idx];

				let idx = y * self.width + x;
				let curr = &mut self.cells[idx];

				match (up, *curr) {
					(Symbol::Start | Symbol::Tachion, Symbol::Splitter) => {
						// at least one
						let count = self.tachion_count[up_idx].max(1);

						let left_idx = (x > 0).then(|| y * self.width + x - 1);
						if let Some(left_idx) = left_idx {
							self.cells[left_idx] = Symbol::Tachion;
							self.tachion_count[left_idx] += count;
						}

						let right_idx = (x + 1 < self.width)
							.then(|| y * self.width + x + 1);
						if let Some(right_idx) = right_idx {
							self.cells[right_idx] = Symbol::Tachion;
							self.tachion_count[right_idx] += count;
						}
					}
					(Symbol::Start | Symbol::Tachion, _) => {
						*curr = Symbol::Tachion;
						self.tachion_count[idx] +=
							self.tachion_count[up_idx].max(1);
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

	fn count_tachions(&self) -> u64 {
		// count all tachions from the last line
		self.tachion_count[(self.height - 1) * self.width..]
			.iter()
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
		tachion_count: vec![0; cells.len()],
		cells,
	}
}

fn part1() -> u64 {
	let mut map = parse_input(INPUT);
	map.run();
	map.count_splits()
}

fn part2() -> u64 {
	let mut map = parse_input(INPUT);
	map.run();
	map.count_tachions()
}

fn main() {
	let p1 = part1();
	println!("Part 1: {p1}");
	assert_eq!(p1, 1635);

	let p2 = part2();
	println!("Part 2: {p2}");
	assert_eq!(p2, 58097428661390);
}

#[test]
fn test_p2() {
	let mut map = parse_input(
		"\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
	);
	map.run();
	assert_eq!(map.count_tachions(), 40);
}
