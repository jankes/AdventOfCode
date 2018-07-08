use std::str::FromStr;
use std::fs;

fn main() {
    let firewall_spec = fs::read_to_string("C:\\Users\\jankes\\Documents\\AdventOfCode\\2017\\13\\firewall.txt")
                        .expect("should be able to read firewall specification");

    let mut sim = FirewallSim::from_spec(&firewall_spec);

    for (layer, maybe_security) in sim.layers.iter().enumerate() {
        match maybe_security {
            Some(security) => println!("{}: {}", layer, security.range),
            None           => println!("{}:", layer)
        };
    }

    let trip_severity = sim.simulate();
    println!("trip severity = {}", trip_severity);
}

struct Security {
    position: u8,
    is_forwards: bool,
    range: u8
}

struct FirewallSim {
    my_position: u8,
    layers: Vec<Option<Security>>
}

impl Security {
    fn with_range(range: u8) -> Security {
        Security {
            position: 0,
            is_forwards: true,
            range: range
        }
    }
}

impl FirewallSim {
    fn from_spec(spec: &str) -> FirewallSim {
        let mut layers = Vec::<Option<Security>>::new();
        let mut next_layer = 0u8;
        for line in spec.lines() {
            let (layer, range) = parse_layer_and_range(line);

            next_layer = fill_empty_layers(&mut layers, layer, next_layer);

            layers.push(Some(Security::with_range(range)));
        }
        return FirewallSim { my_position: 0, layers: layers };

        fn fill_empty_layers(layers: &mut Vec<Option<Security>>, current_layer: u8, mut next_layer: u8) -> u8 {
            if next_layer == 0 && current_layer != 0 {
                panic!("expect definition for layer 0 to be first line of firewall spec");
            } else {
                while current_layer != next_layer {
                    next_layer += 1;
                    layers.push(None);
                }
            }
            current_layer + 1
        }

        fn parse_layer_and_range(line: &str) -> (u8, u8) {
            let mut parts = line.split(": ");
            let layer_str = parts.next().expect("expect layer number");
            let layer = u8::from_str(layer_str).expect("expect layer number to be a number storable within a u8");
            let range_str = parts.next().expect("expect range");
            let range = u8::from_str(range_str).expect("expect range to be a number storable within a u8");
            (layer, range)
        }
    }

    fn simulate(&mut self) -> u16 {
        // picosecond 0: no severity even if we do get caught
        // just step the security programs to put us in a state where further {step_me, step_security} steps work as expected
        self.step_security();

        let mut total_severity = 0u16;
        while let (severity, false) = self.step() {
            total_severity += severity;
        }
        total_severity
    } 

    fn step(&mut self) -> (u16, bool) {
        let (severity, done) = self.step_me();
        self.step_security();
        (severity, done)
    }

    fn step_me(&mut self) -> (u16, bool) {
        if self.my_position as usize + 1 < self.layers.len() {
            self.my_position += 1;
            let severity =
                if let Some(ref security) = self.layers[self.my_position as usize] {
                    if security.position == 0 {
                        println!("caught at layer {}!", self.my_position);
                        self.my_position as u16 * security.range as u16
                    } else {
                        0
                    }
                } else {
                    0
                };
            (severity, false)
        } else {
            (0, true)
        }
    }

    fn step_security(&mut self) {
        for layer in self.layers.iter_mut() {
            if let Some(ref mut security) = layer {
                if security.is_forwards {
                    if security.position + 1 < security.range {
                        security.position += 1;
                    } else {
                        security.is_forwards = false;
                        if security.position != 0 {
                            security.position -= 1;
                        }
                    }
                } else {
                    if security.position != 0 {
                        security.position -= 1;
                    } else {
                        security.is_forwards = true;
                        if security.range > 1 {
                            security.position = 1;
                        }
                    }
                }
            }
        }
    }
}