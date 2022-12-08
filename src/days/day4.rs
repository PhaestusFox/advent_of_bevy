use bevy::prelude::*;
use rand::{SeedableRng, Rng};

use crate::{CalenderState, advent_calendar::{CalendarAssets, AdventData}};

use super::{DayItem, Day};


pub struct DayPlugin;

impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(CalenderState::Day4)
        .with_system(super::spawn_day::<4>)
        .with_system(read_data))
        .add_system_set(SystemSet::on_exit(CalenderState::Day4)
        .with_system(crate::cleanup::<DayItem>));
    }
}

struct Range {
    top: usize,
    bottom: usize,
}

impl Range {
    fn new(input: &str) -> Result<Range, ()> {
        let input = input.trim();
        let mut segs = input.split('-');
        let first = segs.next().unwrap();
        let second = segs.next().unwrap();
        Ok(Range { top: second.parse().unwrap(), bottom: first.parse().unwrap() })
    }
    fn contains(&self, other: &Range) -> bool {
        other.top <= self.top && other.bottom >= self.bottom
    }
    fn overlap(&self, other: &Range) -> bool {
        other.top >= self.bottom && other.bottom <= self.top
    }
}

fn read_data(
    mut commands: Commands,
    days: Res<Assets<Day>>,
    assert_server: Res<AssetServer>,
    advent_data: Res<AdventData>,
) {
    let Some(day) = days.get(&assert_server.load("days/day4.day.ron")) else {error!("Day 4 in not loaded"); return;};
    let mut contained = 0;
    let mut overlap = 0;
    for line in day.data.lines() {
        let line = line.trim();
        let ranges: Vec<&str> = line.split(',').collect();
        let r0 = Range::new(ranges[0]).unwrap();
        let r1 = Range::new(ranges[1]).unwrap();
        if r0.contains(&r1) || r1.contains(&r0) {contained += 1}
        if r0.overlap(&r1) {overlap += 1};
    }
    println!("{} Overlaps and {} are Full\n", overlap, contained);
}