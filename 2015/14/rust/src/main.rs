fn main() {
	println!("part 1 example:");
	{
		let mut comet = Reindeer::new(14, 10, 127);
		let mut dancer = Reindeer::new(16, 11, 162);
		for _ in 0..1000 {
			comet.simulate_1_sec();
			dancer.simulate_1_sec();
		}
		println!("comet = {}, dancer = {}", comet.distance_traveled, dancer.distance_traveled);
	}

	let mut reindeer = vec!(
		Reindeer::new(8, 8, 53),
		Reindeer::new(13, 4, 49),
		Reindeer::new(20, 7, 132),
		Reindeer::new(12, 4, 43),
		Reindeer::new(9, 5, 38),
		Reindeer::new(10, 4, 37),
		Reindeer::new(3, 37, 76),
		Reindeer::new(9, 12, 97),
		Reindeer::new(37, 1, 36));

	let mut points = Vec::<i32>::with_capacity(reindeer.len());
	points.resize(reindeer.len(), 0);

	for _ in 0..2503 {
		for r in reindeer.iter_mut() {
			r.simulate_1_sec();
		}

		let lead_distance = furthest_distance(&reindeer);
		for (i, r) in reindeer.iter().enumerate() {
			if r.distance_traveled == lead_distance {
				points[i] += 1;
			}
		}
	}

	let winner_distance = furthest_distance(&reindeer);
	let most_points = points.iter().max().unwrap();

	println!("winning distance reindeer goes {} km", winner_distance);
	println!("most points received is: {}", most_points);
}

fn furthest_distance(reindeer: &[Reindeer]) -> i32 {
	reindeer.iter()
	        .map(|r| r.distance_traveled)
			.max()
			.unwrap()
	/*	
	reindeer.iter().fold(0, |max_distance, r| {
		if r.distance_traveled > max_distance {
			r.distance_traveled
		} else {
			max_distance
		}
	})
	*/
}

struct Reindeer {
	speed: i32,
	fly_limit: i32,
	rest_limit: i32,

	state: State,
	current_state_time: i32,
	distance_traveled: i32
}

#[derive(Debug)]
enum State {
	Resting, Flying
}

impl Reindeer {
	fn new(speed: i32, fly_limit: i32, rest_limit: i32) -> Reindeer {
		Reindeer {
			speed: speed,
			fly_limit: fly_limit,
			rest_limit: rest_limit,
			state: State::Flying,
			current_state_time: 0,
			distance_traveled: 0
		}
	}

	fn simulate_1_sec(&mut self) {
		match self.state {
			State::Resting => {
				if self.current_state_time < self.rest_limit {
					self.current_state_time += 1;
				} else {
					self.state = State::Flying;
					self.distance_traveled += self.speed;
					self.current_state_time = 1;
				}
			},
			State::Flying => {
				if self.current_state_time < self.fly_limit {
					self.distance_traveled += self.speed;
					self.current_state_time += 1
				} else {
					self.state = State::Resting;
					self.current_state_time = 1;
				}
			}
		}
	}
}