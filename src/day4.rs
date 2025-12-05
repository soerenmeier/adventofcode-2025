const INPUT: &str = include_str!("../inputs/day4.txt");

struct Map<'a> {
	inner: Vec<&'a [u8]>,
}

impl<'a> Map<'a> {
	fn accessible_rolls(&self) -> u64 {
		let mut count = 0;

		for (y, line) in self.inner.iter().enumerate() {
			for (x, &c) in line.iter().enumerate() {
				if c == b'@' && self.is_accessible(x, y) {
					count += 1;
				}
			}
		}

		count
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
			.filter_map(|&(dx, dy)| {
				let nx = usize::try_from(x as i32 + dx).ok()?;
				let ny = usize::try_from(y as i32 + dy).ok()?;

				self.inner.get(ny)?.get(nx).filter(|&&c| c == b'@')
			})
			.count();

		adjacent < 4
	}
}

fn parse_input<'a>(input: &'a str) -> Map<'a> {
	Map {
		inner: input.trim().lines().map(|l| l.as_bytes()).collect(),
	}
}

fn part1() -> u64 {
	let map = parse_input(INPUT);

	map.accessible_rolls()
}

fn main() {
	let p1 = part1();
	println!("Part 1: {p1}");
}

#[test]
fn test_part1() {
	let map = parse_input(
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

	assert_eq!(map.accessible_rolls(), 13);
}
