use crate::WORLD;
use aoc_runner_derive::aoc;
use lamellar::active_messaging::prelude::*;
use lamellar::darc::prelude::*;

fn parse_num_list(line: &str) -> Vec<usize> {
    line.trim()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

#[AmData]
struct Part1 {
    seed: usize,
    #[AmGroup(static)]
    maps: Darc<Maps>,
}

#[am]
impl LamellarAm for Part1 {
    async fn exec() -> usize {
        self.maps.get_location(self.seed)
    }
}

#[AmData]
struct Part2 {
    start_seed: usize,
    length: usize,
    maps: Darc<Maps>,
}

#[am]
impl LamellarAm for Part2 {
    async fn exec() -> usize {
        self.maps.get_location_ranges(self.start_seed, self.length)
    }
}

struct Maps {
    seed_to_soil: Vec<Vec<usize>>,
    soil_to_fert: Vec<Vec<usize>>,
    fert_to_water: Vec<Vec<usize>>,
    water_to_light: Vec<Vec<usize>>,
    light_to_temperature: Vec<Vec<usize>>,
    temperature_to_humidity: Vec<Vec<usize>>,
    humidity_to_location: Vec<Vec<usize>>,
}

impl Maps {
    fn new<'a>(mut lines: impl Iterator<Item = &'a str>) -> Maps {
        let mut maps = Maps {
            seed_to_soil: Vec::new(),
            soil_to_fert: Vec::new(),
            fert_to_water: Vec::new(),
            water_to_light: Vec::new(),
            light_to_temperature: Vec::new(),
            temperature_to_humidity: Vec::new(),
            humidity_to_location: Vec::new(),
        };
        let name_maps = [
            ("seed-to-soil map:", &mut maps.seed_to_soil),
            ("soil-to-fertilizer map:", &mut maps.soil_to_fert),
            ("fertilizer-to-water map:", &mut maps.fert_to_water),
            ("water-to-light map:", &mut maps.water_to_light),
            ("light-to-temperature map:", &mut maps.light_to_temperature),
            (
                "temperature-to-humidity map:",
                &mut maps.temperature_to_humidity,
            ),
            ("humidity-to-location map:", &mut maps.humidity_to_location),
        ];
        for (name, map) in name_maps {
            assert_eq!(name, lines.next().expect("properly formatted input"));
            while let Some(line) = lines.next() {
                if line != "" {
                    let numbers = parse_num_list(&line);
                    map.push(numbers);
                } else {
                    break;
                }
            }
        }
        maps
    }

    fn get_val(&self, map: &Vec<Vec<usize>>, key: usize) -> usize {
        let mut val = key; // assume its not in a range in the map
        for data in map.iter() {
            if data[1] <= key && key < data[1] + data[2] {
                val = data[0] + (key - data[1]);
                break;
            }
        }
        val
    }

    fn get_val_ranges(
        &self,
        map: &Vec<Vec<usize>>,
        start: usize,
        len: usize,
    ) -> Vec<(usize, usize)> {
        let mut cur_start = start;
        let mut cur_len = len;
        let mut cur_end = cur_start + cur_len - 1;
        let mut ranges = Vec::new();

        loop {
            for data in map.iter() {
                let d_end = data[1] + data[2];
                if data[1] <= cur_start && cur_start < d_end {
                    // println!("cur_start is in this entry in the map");
                    let val_start = data[0] + (cur_start - data[1]);
                    if cur_end < d_end {
                        // println!("cur_end is in this entry in the map");
                        //entire range is in this entry in the map
                        ranges.push((val_start, cur_len));
                        return ranges; //no need to check other entries
                    } else {
                        // println!("cur_end is not in this entry in the map");
                        //need to split this range
                        let new_len = d_end - cur_start;
                        ranges.push((val_start, new_len));
                        cur_start = d_end;
                        cur_len -= new_len;
                        cur_end = cur_start + cur_len;
                    }
                } else if data[1] <= cur_end && cur_end < d_end {
                    // println!("cur_start is not but cur_end is in this entry in the map");
                    let val_start = data[0];
                    let new_len = cur_end - data[1];
                    ranges.push((val_start, new_len));
                    cur_start = data[1];
                    cur_len -= new_len;
                    cur_end = cur_start + cur_len;
                } else {
                    // println!("cur_start and cur_end are not in this entry in the map");
                    if cur_start <= data[1] && data[1] + data[2] < cur_end {
                        // println!("this entry from the map entire in cur_start -- cur_end");
                        let val_start = data[0];
                        let new_len = data[2];
                        ranges.push((val_start, new_len));
                        let left = self.get_val_ranges(map, cur_start, data[1] - cur_start);
                        let right = self.get_val_ranges(
                            map,
                            data[1] + data[2],
                            cur_end - (data[1] + data[2]),
                        );
                        ranges.extend(left);
                        ranges.extend(right);
                        return ranges; // no need to check other entries
                    }
                }
            }
            // println!("no overlap");
            ranges.push((cur_start, cur_len));
            return ranges;
        }
    }

    fn get_location(&self, seed: usize) -> usize {
        let soil = self.get_val(&self.seed_to_soil, seed);
        let fert = self.get_val(&self.soil_to_fert, soil);
        let water = self.get_val(&self.fert_to_water, fert);
        let light = self.get_val(&self.water_to_light, water);
        let temp = self.get_val(&self.light_to_temperature, light);
        let hum = self.get_val(&self.temperature_to_humidity, temp);
        self.get_val(&self.humidity_to_location, hum)
    }

    fn get_location_ranges(&self, start_seed: usize, len: usize) -> usize {
        let mut min = usize::MAX;
        for soil_range in self
            .get_val_ranges(&self.seed_to_soil, start_seed, len)
            .iter()
        {
            for fert_range in self
                .get_val_ranges(&self.soil_to_fert, soil_range.0, soil_range.1)
                .iter()
            {
                for water_range in self
                    .get_val_ranges(&self.fert_to_water, fert_range.0, fert_range.1)
                    .iter()
                {
                    for light_range in self
                        .get_val_ranges(&self.water_to_light, water_range.0, water_range.1)
                        .iter()
                    {
                        for temp_range in self
                            .get_val_ranges(
                                &self.light_to_temperature,
                                light_range.0,
                                light_range.1,
                            )
                            .iter()
                        {
                            for hum_range in self
                                .get_val_ranges(
                                    &self.temperature_to_humidity,
                                    temp_range.0,
                                    temp_range.1,
                                )
                                .iter()
                            {
                                let mut loc_ranges = self.get_val_ranges(
                                    &self.humidity_to_location,
                                    hum_range.0,
                                    hum_range.1,
                                );
                                loc_ranges.sort();
                                let temp_min = loc_ranges[0].0;
                                if temp_min < min {
                                    min = temp_min;
                                }
                            }
                        }
                    }
                }
            }
        }
        min
    }
}

#[aoc(day5, part1, A_INIT_WORLD)]
pub fn part_1(_input: &str) -> u32 {
    WORLD.num_pes() as u32
}

#[aoc(day5, part1, serial)]
pub fn part_1_serial(input: &str) -> u32 {
    let mut lines = input.lines();

    let seeds_line = lines.next().expect("properly formatted input");
    let seeds = parse_num_list(
        seeds_line
            .split(":")
            .last()
            .expect("properly formatted input"),
    );
    assert_eq!("", lines.next().expect("properly formatted input"));
    let maps = Maps::new(lines);

    let min = seeds
        .iter()
        .map(|&seed| maps.get_location(seed))
        .min()
        .unwrap();
    min as u32
}

#[aoc(day5, part1, am)]
pub fn part_1_am(input: &str) -> u32 {
    let mut lines = input.lines();

    let seeds_line = lines.next().expect("properly formatted input");
    let seeds = parse_num_list(
        seeds_line
            .split(":")
            .last()
            .expect("properly formatted input"),
    );
    assert_eq!("", lines.next().expect("properly formatted input"));
    let maps = Darc::new(WORLD.team(), Maps::new(lines)).unwrap();

    let reqs = seeds
        .iter()
        .map(|&seed| {
            WORLD.exec_am_local(Part1 {
                seed,
                maps: maps.clone(),
            })
        })
        .collect::<Vec<_>>();
    let min = *WORLD
        .block_on(futures::future::join_all(reqs))
        .iter()
        .min()
        .unwrap();
    min as u32
}

#[aoc(day5, part2, serial)]
pub fn part_2_serial(input: &str) -> u32 {
    let mut lines = input.lines();

    let seeds_line = lines.next().expect("properly formatted input");
    let seed_ranges = parse_num_list(
        seeds_line
            .split(":")
            .last()
            .expect("properly formatted input"),
    );

    assert_eq!("", lines.next().expect("properly formatted input"));
    let maps = Maps::new(lines);

    let min = seed_ranges
        .chunks(2)
        .into_iter()
        .map(|seed_range| maps.get_location_ranges(seed_range[0], seed_range[1]))
        .min()
        .unwrap();
    min as u32
}

#[aoc(day5, part2, am)]
pub fn part_2_am(input: &str) -> u32 {
    let mut lines = input.lines();

    let seeds_line = lines.next().expect("properly formatted input");
    let seed_ranges = parse_num_list(
        seeds_line
            .split(":")
            .last()
            .expect("properly formatted input"),
    );

    assert_eq!("", lines.next().expect("properly formatted input"));
    let maps = Darc::new(WORLD.team(), Maps::new(lines)).unwrap();

    let reqs = seed_ranges
        .chunks(2)
        .into_iter()
        .map(|seed_range| {
            WORLD.exec_am_local(Part2 {
                start_seed: seed_range[0],
                length: seed_range[1],
                maps: maps.clone(),
            })
        })
        .collect::<Vec<_>>();

    let min = *WORLD
        .block_on(futures::future::join_all(reqs))
        .iter()
        .min()
        .unwrap();
    min as u32
}
