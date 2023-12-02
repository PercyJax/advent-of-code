use std::collections::HashMap;

fn main() {
    let input = String::from_utf8(include_bytes!("../input.txt").to_vec()).unwrap();
    let mut lines = input.split('\n');

    let mut seedlist: Vec<usize> = Vec::new();
    let mut maps = HashMap::new();

    'main: loop {
        let Some(line) = lines.next() else {
            break 'main;
        };
        if line.is_empty() {
            continue;
        }

        let mut line = line.split(':');
        let map_type = line.next().unwrap().split_whitespace().next().unwrap();
        match map_type {
            "seeds" => {
                seedlist = line
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.trim().parse().unwrap())
                    .collect()
            }
            s => {
                let mapping = s.trim();
                maps.insert(mapping, Vec::new());
                'section: loop {
                    let Some(line) = lines.next() else {
                        break 'main;
                    };
                    if line.is_empty() {
                        break 'section;
                    }
                    let mut s = line.split_whitespace();
                    let dest_range_start: usize = s.next().unwrap().parse().unwrap();
                    let src_range_start: usize = s.next().unwrap().parse().unwrap();
                    let range_len: usize = s.next().unwrap().parse().unwrap();
                    let map = maps.get_mut(mapping).unwrap();
                    map.push((dest_range_start, src_range_start, range_len));
                }
            }
        }
    }

    let now = std::time::Instant::now();

    let locations = seedlist.iter().map(|seed| {
        let soil = get_mapping(maps.get("seed-to-soil").unwrap(), seed);
        let fertilizer = get_mapping(maps.get("soil-to-fertilizer").unwrap(), &soil);
        let water = get_mapping(maps.get("fertilizer-to-water").unwrap(), &fertilizer);
        let light = get_mapping(maps.get("water-to-light").unwrap(), &water);
        let temperature = get_mapping(maps.get("light-to-temperature").unwrap(), &light);
        let humidity = get_mapping(maps.get("temperature-to-humidity").unwrap(), &temperature);
        let location = get_mapping(maps.get("humidity-to-location").unwrap(), &humidity);
        location
    });
    let part1: usize = locations.min().unwrap();

    let part1_time = now.elapsed();

    let now = std::time::Instant::now();

    let mut locations = Vec::new();
    let mut start = 0;
    while start < seedlist.len() {
        let soil = get_mapping_regions(
            maps.get("seed-to-soil").unwrap(),
            vec![(seedlist[start], seedlist[start + 1])],
        );
        let fertilizer = get_mapping_regions(maps.get("soil-to-fertilizer").unwrap(), soil);
        let water = get_mapping_regions(maps.get("fertilizer-to-water").unwrap(), fertilizer);
        let light = get_mapping_regions(maps.get("water-to-light").unwrap(), water);
        let temperature = get_mapping_regions(maps.get("light-to-temperature").unwrap(), light);
        let humidity =
            get_mapping_regions(maps.get("temperature-to-humidity").unwrap(), temperature);
        let location = get_mapping_regions(maps.get("humidity-to-location").unwrap(), humidity);

        locations.extend(location);

        start += 2;
    }
    let part2: usize = locations.into_iter().map(|l| l.0).min().unwrap();

    let part2_time = now.elapsed();

    println!("Part 1: {part1} - took {part1_time:.2?}");
    println!("Part 2: {part2} - took {part2_time:.2?}");
}

fn get_mapping(map: &Vec<(usize, usize, usize)>, src: &usize) -> usize {
    for element in map {
        if *src < element.1 {
            continue;
        }
        if *src >= element.1 + element.2 {
            continue;
        }
        return element.0 + (*src - element.1);
    }
    *src
}

fn get_mapping_regions(
    map: &[(usize, usize, usize)],
    input: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    for s in input {
        let src = s.0;
        let len = s.1;

        let mut map = map.to_vec();
        map.sort_by(|a, b| a.1.cmp(&b.1));

        let mut start = src;
        let end = start + len;

        for element in map {
            if start < element.1 {
                if end < element.1 {
                    continue;
                } else {
                    res.push((
                        start,
                        (isize::try_from(element.1).unwrap() - isize::try_from(start).unwrap())
                            .try_into()
                            .unwrap(),
                    ));
                    start = element.1;
                }
            }
            if start >= element.1 + element.2 {
                continue;
            }
            let offset: isize =
                isize::try_from(start).unwrap() - isize::try_from(element.1).unwrap();
            let dest: isize = isize::try_from(element.0).unwrap() + offset;
            let map_runway =
                isize::try_from(element.1 + element.2).unwrap() - isize::try_from(start).unwrap();
            let input_runway = isize::try_from(end).unwrap() - isize::try_from(start).unwrap();

            res.push((
                dest.try_into().unwrap(),
                usize::try_from(map_runway.min(input_runway)).unwrap(),
            ));
            start += usize::try_from(map_runway.min(input_runway)).unwrap();
        }
        if start < end {
            res.push((
                start,
                (isize::try_from(end).unwrap() - isize::try_from(start).unwrap())
                    .try_into()
                    .unwrap(),
            ));
        }
    }
    res
}
