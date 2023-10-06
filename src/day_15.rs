use std::collections::HashSet;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::input;

#[test]
fn part_1() {
    let input = input!(15);
    let sensors = parse_input(input);
    let ans = num_non_beacons(&sensors);
    dbg!(ans);
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Sensor {
    sensor: Point,
    nearest_beacon: Point,
}

fn parse_input(input: &str) -> Vec<Sensor> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Sensor {
    static RE: Lazy<Regex> = Lazy::new(|| {
        let n = r"(-?\d+)";
        let re = format!(r"^Sensor at x={n}, y={n}: closest beacon is at x={n}, y={n}$");
        Regex::new(&re).unwrap()
    });

    let caps = RE.captures(line).unwrap();
    let x = caps[1].parse().unwrap();
    let y = caps[2].parse().unwrap();
    let sensor = Point { x, y };
    let x = caps[3].parse().unwrap();
    let y = caps[4].parse().unwrap();
    let nearest_beacon = Point { x, y };
    
    Sensor { sensor, nearest_beacon }
}

fn num_non_beacons(sensors: &[Sensor]) -> usize {
    let y = 2_000_000;

    // Shadow and beacons on the row we care about (y=2M)
    // We only store the x-values of these points, since there's only 1 row.
    let mut shadow = HashSet::<i32>::new();
    let mut beacons = HashSet::<i32>::new();

    for s in sensors {
        if s.nearest_beacon.y == y {
            beacons.insert(s.nearest_beacon.x);
        }

        // Cast a shadow onto the row we care about
        let radius = dist(s.sensor, s.nearest_beacon);
        let diam = 1 + 2 * radius;
        let dy = s.sensor.y.abs_diff(y);
        let shadow_diam = diam.saturating_sub(2 * dy);
        draw_shadow(s.sensor.x, shadow_diam as i32, &mut shadow);
    }

    shadow.len() - beacons.len()
}

fn dist(p1: Point, p2: Point) -> u32 {
    p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y)
}

fn draw_shadow(center: i32, diam: i32, out: &mut HashSet<i32>) {
    if diam == 0 {
        return;
    }
    assert_eq!(diam % 2, 1);
    out.insert(center);
    let radius = diam / 2;
    for i in 1..=radius {
        out.insert(center + i);
        out.insert(center - i);
    }
}
