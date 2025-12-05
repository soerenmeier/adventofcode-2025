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

fn main() {
	let p1 = part1();
	println!("Part 1: {}", p1);
}
