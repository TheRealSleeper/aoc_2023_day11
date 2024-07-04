use std::env::args;
use std::fs::read_to_string; 

fn get_lengths(from: i32, galaxies: &Vec<(i32, i32)>, verbose: bool) -> i32 {
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

    if from < galaxies.len() as i32 - 2 {
        sum + get_lengths(from + 1, galaxies, verbose)
    } else {
        sum
    }
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

    let mut _part1 = false;
    let mut _part2 = false;
    let mut path: Option<String> = None;
    let mut verbose = false;
    let mut args = args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-i" | "--input" => path = Some(args.next().expect("No path provided!")),
            "-p1" | "--part1" => _part1 = true,
            "-p2" | "--part2" => _part2 = true,
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

    let mut galaxy_map = if let Some(c) = content{
        c.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
    } else {
        sample
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
    };

    // Find empty columns and create a copy of each one, walking back to front so that no index manipulation is needed
    let row_len = galaxy_map[0].len();
    for i in 1..=row_len {
        let mut empty = true;

        for y in 0..galaxy_map.len() {
            if galaxy_map[y][row_len - i] == '#' {
                empty = false;
            }
        }

        if empty {
            for ii in 0..galaxy_map.len() {
                galaxy_map[ii].insert(row_len - i, '.');
            }
        }
    }

    // Find empty rows and create a copy of each one, walking bottom to top so that no index manipulation is needed
    let row_count = galaxy_map.len();
    for i in 1..=row_count {
        let mut empty = true;
        for ch in &galaxy_map[row_count - i] {
            if *ch == '#' {
                empty = false;
            }
        }

        if empty {
            galaxy_map.insert(row_count - i, galaxy_map[row_count - i].clone());
        }
    }

    if verbose {
        println!(
            "{}\n",
            galaxy_map
                .iter()
                .map(|row| {
                    let mut tmp = row.iter().collect::<String>();
                    tmp.push('\n');
                    tmp
                })
                .collect::<String>()
        );
    }

    let mut galaxies: Vec<(i32, i32)> = vec![];
    for y in 0..galaxy_map.len() {
        for x in 0..galaxy_map[y].len() {
            if galaxy_map[y][x] == '#' {
                galaxies.push((x as i32, y as i32));
            }
        }
    }

    if verbose {
        for galaxy in &galaxies {
            println!("x:{} y:{}", galaxy.0, galaxy.1);
        }
        print!("\n"); 
    }

    println!("total distance is {}", get_lengths(0, &galaxies, verbose));
}
