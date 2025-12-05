const INPUT: &str = include_str!("../inputs/day4.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
	Empty,
	Roll,
	RemoveableRoll,
}

impl Cell {
	fn new(c: u8) -> Self {
		match c {
			b'.' => Cell::Empty,
			b'@' => Cell::Roll,
			b'x' => Cell::RemoveableRoll,
			_ => panic!("Invalid cell character: {}", c as char),
		}
	}

	fn is_roll(&self) -> bool {
		matches!(self, Cell::Roll | Cell::RemoveableRoll)
	}
}

#[derive(Debug)]
struct Map {
	inner: Vec<Cell>,
	width: usize,
	height: usize,
}

impl Map {
	fn remove_accessible(&mut self) -> u64 {
		let mut count = 0;

		for y in 0..self.height {
			for x in 0..self.width {
				let idx = y * self.width + x;

				if self.inner[idx].is_roll() && self.is_accessible(x, y) {
					self.inner[idx] = Cell::RemoveableRoll;
					count += 1;
				}
			}
		}

		count
	}

	fn clean_removeable(&mut self) {
		for c in &mut self.inner {
			if *c == Cell::RemoveableRoll {
				*c = Cell::Empty;
			}
		}
	}

	fn is_accessible(&self, x: usize, y: usize) -> bool {
		#[rustfmt::skip]
		const DISPLACEMENT: &[(i32, i32); 8] = &[
			(-1, -1), (-1, 0), (-1, 1),
			(0, -1),           (0, 1),
			(1, -1),  (1, 0),  (1, 1),
		];

		let adjacent = DISPLACEMENT
			.iter()
			.filter_map(|(dx, dy)| {
				let nx = usize::try_from(x as i32 + dx)
					.ok()
					.filter(|&nx| nx < self.width)?;
				let ny = usize::try_from(y as i32 + dy).ok()?;

				self.inner.get(ny * self.width + nx).filter(|c| c.is_roll())
			})
			.count();

		adjacent < 4
	}
}

fn parse_input(input: &str) -> Map {
	let mut width = 0;

	let inner = input
		.trim()
		.lines()
		.map(|l| {
			let bytes = l.as_bytes();
			if width != bytes.len() {
				assert!(width == 0);
				width = bytes.len();
			}

			bytes.iter().map(|&b| Cell::new(b))
		})
		.flatten()
		.collect::<Vec<_>>();

	Map {
		width,
		height: inner.len() / width,
		inner,
	}
}

fn part1() -> u64 {
	let mut map = parse_input(INPUT);

	map.remove_accessible()
}

fn part2() -> u64 {
	let mut map = parse_input(INPUT);

	let mut count = 0;

	loop {
		let removeable = map.remove_accessible();
		if removeable == 0 {
			return count;
		}

		count += removeable;
		map.clean_removeable();
	}
}

fn main() {
	let p1 = part1();
	println!("Part 1: {p1}");
	assert_eq!(p1, 1604);

	let p2 = part2();
	println!("Part 2: {p2}");
}

#[test]
fn test_part1() {
	let mut map = parse_input(
		"\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
",
	);

	assert_eq!(map.remove_accessible(), 13);
}
