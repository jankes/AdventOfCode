use std::str::FromStr;

static INPUT: &'static str =
"1224	926	1380	688	845	109	118	88	1275	1306	91	796	102	1361	27	995\n\
1928	2097	138	1824	198	117	1532	2000	1478	539	1982	125	1856	139	475	1338\n\
848	202	1116	791	1114	236	183	186	150	1016	1258	84	952	1202	988	866\n\
946	155	210	980	896	875	925	613	209	746	147	170	577	942	475	850\n\
1500	322	43	95	74	210	1817	1631	1762	128	181	716	171	1740	145	1123\n\
3074	827	117	2509	161	206	2739	253	2884	248	3307	2760	2239	1676	1137	3055\n\
183	85	143	197	243	72	291	279	99	189	30	101	211	209	77	198\n\
175	149	259	372	140	250	168	142	146	284	273	74	162	112	78	29\n\
169	578	97	589	473	317	123	102	445	217	144	398	510	464	247	109
3291	216	185	1214	167	495	1859	194	1030	3456	2021	1622	3511	222	3534	1580\n\
2066	2418	2324	93	1073	82	102	538	1552	962	91	836	1628	2154	2144	1378\n\
149	963	1242	849	726	1158	164	1134	658	161	1148	336	826	1303	811	178\n\
3421	1404	2360	2643	3186	3352	1112	171	168	177	146	1945	319	185	2927	2289\n\
543	462	111	459	107	353	2006	116	2528	56	2436	1539	1770	125	2697	2432\n\
1356	208	5013	4231	193	169	3152	2543	4430	4070	4031	145	4433	4187	4394	1754\n\
5278	113	4427	569	5167	175	192	3903	155	1051	4121	5140	2328	203	5653	3233";

fn main() {

    /*
    for (i, line) in INPUT.lines().enumerate() {
        let numbers = line.split_whitespace()
                        .map(str_to_number)
                        .collect::<Vec<u16>>();

        print!("line {}: ", i);
        for (j, first) in numbers.iter().enumerate() {
            for second in numbers.iter().skip((j + 1) as usize) {
                let (first, second) = if first > second { (first, second) } else { (second, first) };
                if first % second == 0 {
                    print!("({},{}) ", first, second);
                }
            }
        }
        println!();
    }
    */

    part_1();
    part_2();
}

fn part_1() {
    let checksum =
    INPUT.lines()
         .map(|line| {
             line.split_whitespace()
                 .map(str_to_number)
                 .fold((u16::max_value(), u16::min_value()), update_min_max)
         })
         .map(|(min, max)| max - min)
         .sum::<u16>();

    println!("part 1: checksum = {}", checksum);
}

fn part_2() {
    let checksum =
    INPUT.lines()
         .map(|line| {
             line.split_whitespace()
                 .map(str_to_number)
                 .collect::<Vec<u16>>()
         })
         .map(|numbers| divide_only_evenly_divisible_pair(&numbers))
         .sum::<u16>();
    println!("part 2: checksum = {}", checksum);
}

fn update_min_max(min_max: (u16, u16), current: u16) -> (u16, u16) {
    let (min, max) = min_max;
    (u16::min(min, current), u16::max(max, current))
}

fn divide_only_evenly_divisible_pair(numbers: &[u16]) -> u16 {
    for (i, first) in numbers.iter().enumerate() {
        for second in numbers.iter().skip((i + 1) as usize) {
            let (first, second) = if first > second { (first, second) } else { (second, first) };
            if first % second == 0 {
                return first / second;
            }
        }
    }
    panic!("no evenly divisible pair");
}

fn str_to_number(s: &str) -> u16 {
    u16::from_str(s).unwrap()
}