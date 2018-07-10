fn main() {
    // let securities = vec!((0u8, 3u8), (1, 2), (4, 4), (6, 4))
                    //  .into_iter()
                    //  .map(|(depth, range)| Security::new(depth, range))
                    //  .collect::<Vec<Security>>();

    let securities = vec!(
        (0u8, 4u8), (1, 2), (2, 3), (4, 4), (6, 8), (8, 5), (10, 6), (12, 6),
        (14, 10), (16, 8), (18, 6), (20, 9), (22, 8), (24, 6), (26, 8), (28, 8),
        (30, 12), (32, 12), (34, 12), (36, 12), (38, 10), (40, 12), (42, 12), (44, 14),
        (46, 8), (48, 14), (50, 12), (52, 14), (54, 14), (58, 14), (60, 12), (62, 14),
        (64, 14), (66, 12), (68, 12), (72, 14), (74, 18), (76, 17), (86, 14), (88, 20),
        (92, 14), (94, 14), (96, 18), (98, 18))
        .into_iter()
        .map(|(depth, range)| Security::new(depth, range))
        .collect::<Vec<Security>>();

    for security in securities.iter() {
        println!("{}: {}", security.depth, security.period);
    }

    // part 1
    let mut severity = 0u16;
    for security in securities.iter() {
        if would_catch_me(security, 0) {
            severity += security.get_severity();
        }
    }
    println!("trip severity = {}", severity);

    // part 2
    let mut delay = 0;
    while delay < 10000000 {
        let mut got_caught = false;
        for security in securities.iter() {
            if would_catch_me(security, delay) {
                got_caught = true;
                break;
            }
        }
        if !got_caught {
            println!("can get through with delay = {}", delay);
            break;
        }
        delay += 2;
    }
}

fn would_catch_me(security: &Security, delay: u32) -> bool {
    let depth = security.depth as u32;
    let period = security.period as u32;
    (depth + delay) % period == 0
}

struct Security {
    depth: u8,
    period: u8
}

impl Security {
    fn new(depth: u8, range: u8) -> Security {
        Security {
            depth: depth,
            period: 2 * (range - 1)
        }
    }

    fn get_severity(&self) -> u16 {
        self.depth as u16 * ((self.period / 2) + 1) as u16
    }
}