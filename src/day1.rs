const INPUT: &str = include_str!("../inputs/day1.txt");

const MAX: i32 = 100;

#[derive(Debug)]
struct Dial {
	pub idx: i32,
}

impl Dial {
	fn new(idx: i32) -> Self {
		Self { idx }
	}

	fn is_zero(&self) -> bool {
		self.idx == 0
	}

	fn rotate(&mut self, dir: Dir, steps: u32) {
		match dir {
			Dir::Left => {
				self.idx = (self.idx - steps as i32).rem_euclid(MAX);
			}
			Dir::Right => {
				self.idx = (self.idx + steps as i32) % MAX;
			}
		}
	}

	fn rotate_with_count(&mut self, dir: Dir, steps: u32) -> u32 {
		let mut count = steps / MAX as u32;
		let rem_steps = steps % MAX as u32;

		match dir {
			Dir::Left => {
				let n_idx = self.idx - rem_steps as i32;

				if n_idx <= 0 && self.idx > 0 {
					count += 1;
				}

				if n_idx < 0 {
					self.idx = MAX + n_idx;
				} else {
					self.idx = n_idx;
				}
			}
			Dir::Right => {
				let n_idx = self.idx + rem_steps as i32;

				if n_idx >= MAX {
					count += 1;
					self.idx = n_idx - MAX;
				} else {
					self.idx = n_idx;
				}
			}
		}

		count
	}
}

#[derive(Debug, Copy, Clone)]
enum Dir {
	Left,
	Right,
}

fn parse_input(input: &str) -> impl Iterator<Item = (Dir, u32)> {
	input.trim().lines().map(|l| {
		let l = l.trim();
		// first comes a char
		let dir = match l.chars().next().unwrap() {
			'L' => Dir::Left,
			'R' => Dir::Right,
			c => panic!("Unknown direction char: {c}"),
		};
		let num = l[1..].parse().expect("Failed to parse number");

		(dir, num)
	})
}

fn part1() -> u32 {
	let input = parse_input(INPUT);

	let mut dial = Dial::new(50);

	input
		.map(|(dir, steps)| {
			dial.rotate(dir, steps);
			dial.is_zero() as u32
		})
		.sum::<u32>()
}

fn part2() -> u32 {
	let input = parse_input(INPUT);

	let mut dial = Dial::new(50);

	input
		.map(|(dir, steps)| dial.rotate_with_count(dir, steps))
		.sum::<u32>()
}

fn main() {
	let p1 = part1();
	println!("Part 1: {p1}");
	assert_eq!(p1, 1055);

	let p2 = part2();
	println!("Part 2: {p2}");
	assert_eq!(p2, 6386);
}

/*
   The dial starts by pointing at 50.
   The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
   The dial is rotated L30 to point at 52.
   The dial is rotated R48 to point at 0.
   The dial is rotated L5 to point at 95.
   The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
   The dial is rotated L55 to point at 0.
   The dial is rotated L1 to point at 99.
   The dial is rotated L99 to point at 0.
   The dial is rotated R14 to point at 14.
   The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.
*/
#[test]
fn test_with_count() {
	let mut dial = Dial::new(50);

	assert_eq!(dial.rotate_with_count(Dir::Left, 68), 1);
	assert_eq!(dial.rotate_with_count(Dir::Left, 30), 0);
	assert_eq!(dial.rotate_with_count(Dir::Right, 48), 1);
	assert_eq!(dial.rotate_with_count(Dir::Left, 5), 0);
	assert_eq!(dial.rotate_with_count(Dir::Right, 60), 1);
	assert_eq!(dial.rotate_with_count(Dir::Left, 55), 1);
	assert_eq!(dial.rotate_with_count(Dir::Left, 1), 0);
	assert_eq!(dial.rotate_with_count(Dir::Left, 99), 1);
	assert_eq!(dial.rotate_with_count(Dir::Right, 14), 0);
	assert_eq!(dial.rotate_with_count(Dir::Left, 82), 1);
}
