use std::collections::BTreeMap;

const INPUT: &str = include_str!("../inputs/day8.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
	x: u32,
	y: u32,
	z: u32,
}

// a will always be < b
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Conn {
	a: usize,
	b: usize,
}

impl Conn {
	fn new(a: usize, b: usize) -> Self {
		if a < b {
			Self { a, b }
		} else {
			Self { a: b, b: a }
		}
	}
}

struct Universe {
	boxes: Vec<Coord>,
	distances: BTreeMap<Conn, f64>,
}

impl Universe {
	pub fn calc_dist(&mut self) {
		self.distances =
			Vec::with_capacity(self.boxes.len() * (self.boxes.len() - 1));
	}
}

fn parse_input(input: &str) -> Universe {
	Universe {
		boxes: input
			.trim()
			.lines()
			.map(|l| {
				let mut parts = l.split(',');
				Coord {
					x: parts.next().unwrap().parse().unwrap(),
					y: parts.next().unwrap().parse().unwrap(),
					z: parts.next().unwrap().parse().unwrap(),
				}
			})
			.collect(),
		distances: vec![],
	}
}

fn main() {}
