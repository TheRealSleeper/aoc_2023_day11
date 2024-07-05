use std::env::args;
use std::fs::read_to_string;

fn get_distance(from: i64, galaxies: &Vec<(i64, i64)>, verbose: bool) -> i64 {
    let mut sum = 0;
    for i in ((from + 1) as usize)..galaxies.len() {
        let dist = (&galaxies[from as usize].0 - &galaxies[i].0).abs()
            + (&galaxies[from as usize].1 - &galaxies[i].1).abs();
        sum += dist;
        if verbose {
            println!(
                "From galaxy {0} at [{1}, {2}] to galaxy {3} at [{4}, {5}] is {6} steps",
                from,
                &galaxies[from as usize].0,
                &galaxies[from as usize].1,
                i,
                &galaxies[i].0,
                &galaxies[i].1,
                dist
            );
        }
    }

    if from < galaxies.len() as i64 - 2 {
        sum + get_distance(from + 1, galaxies, verbose)
    } else {
        sum
    }
}

fn expand_universe(galaxy_map: &mut Vec<(Vec<(char, usize)>, usize)>, factor: usize) {
    // Expand rows
    for row in &mut *galaxy_map {
        let mut empty = true;
        for ch in &mut *row.0 {
            if ch.0 == '#' {
                empty = false;
            }
        }

        if empty {
            row.1 = factor;
        }
    }

    // Expand columns
    for x in 0..galaxy_map[0].0.len() {
        let mut empty = true;
        for y in 0..galaxy_map.len() {
            if galaxy_map[y].0[x].0 == '#' {
                empty = false;
            }
        }

        if empty {
            for y in 0..galaxy_map.len() {
                galaxy_map[y].0[x].1 = factor;
            }
        }
    }
}

fn get_galaxy_locations(galaxy_map: &mut Vec<(Vec<(char, usize)>, usize)>) -> Vec<(i64, i64)> {
    let mut galaxies: Vec<(i64, i64)> = vec![];
    let mut act_y = 0;

    for y in 0..galaxy_map.len() {
        let mut act_x = 0;
        act_y += galaxy_map[y].1 - 1;

        for x in 0..galaxy_map[y].0.len() {
            act_x += galaxy_map[y].0[x].1 - 1;
            if galaxy_map[y].0[x].0 == '#' {
                galaxies.push((act_x as i64, act_y as i64));
            }

            act_x += 1;
        }

        act_y += 1;
    }

    galaxies
}

fn main() {
    let sample = "\
        ...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#....."
        .to_string();

    let mut part1 = false;
    let mut part2 = false;
    let mut path: Option<String> = None;
    let mut verbose = false;
    let mut args = args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-i" | "--input" => path = Some(args.next().expect("No path provided!")),
            "-p1" | "--part1" => part1 = true,
            "-p2" | "--part2" => part2 = true,
            "-v" | "--verbose" => verbose = true,
            "-h" | "--help" => {
                print!(
                    "This solves the challenge found at https://adventofcode.com/2023/day/10\n\
                    The following are valid commands:\n\
                    -o  | --open    : Uses specified input file\n\
                    -h  | --help    : opens this help page\n\
                    -p1 | --part1   : solves part 1 of the challenge
                    -p2 | --part2   : solves part 2 of the challenge (not implemented yet)\n\
                    -v  | --verbose : prints each pipe that gets checked\n"
                );
                return;
            }
            _ => {}
        }
    }

    let content = if let Some(p) = path {
        Some(read_to_string(p).expect("Could not read specified file"))
    } else {
        None
    };

    // Parse input
    let mut galaxy_map = if let Some(c) = content {
        c.lines()
            .map(|line| {
                (
                    line.chars()
                        .map(|char| (char, 1))
                        .collect::<Vec<(char, usize)>>(),
                    1,
                )
            })
            .collect::<Vec<(Vec<(char, usize)>, usize)>>()
    } else {
        sample
            .lines()
            .map(|line| {
                (
                    line.chars()
                        .map(|char| (char, 1))
                        .collect::<Vec<(char, usize)>>(),
                    1,
                )
            })
            .collect::<Vec<(Vec<(char, usize)>, usize)>>()
    };

    let expansion_factor = if part1 {
        2
    } else if part2 {
        1000000
    } else {
        panic!("Neither part 1 or 2 selected!");
    };

    expand_universe(&mut galaxy_map, expansion_factor);

    let galaxies = get_galaxy_locations(&mut galaxy_map);

    if verbose {
        for galaxy in &galaxies {
            println!("x:{} y:{}", galaxy.0, galaxy.1);
        }
        print!("\n");
    }

    println!("total distance is {}", get_distance(0, &galaxies, verbose));
}
