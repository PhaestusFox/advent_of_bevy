use bevy::prelude::*;
use rand::{SeedableRng, Rng, seq::SliceRandom};

use crate::{CalenderState, advent_calendar::{CalendarAssets, AdventData}};

use super::{DayItem, Day};

pub struct DayPlugin;

impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(CalenderState::Day6)
        .with_system(super::spawn_day::<6>)
        .with_system(read_data))
        .add_system_set(SystemSet::on_exit(CalenderState::Day6)
        .with_system(crate::cleanup::<DayItem>));
    }
}

fn contains_dup(string: &str) -> bool {
    for (i, char) in string.char_indices() {
        if string[i+1..].contains(char) {return true;}
    }
    false
}


fn read_data(
    mut commands: Commands,
    days: Res<Assets<Day>>,
    assert_server: Res<AssetServer>,
    calendar_assets: Res<CalendarAssets>,
) {
    let Some(day) = days.get(&assert_server.load("days/day6.day.ron")) else {error!("Day 6 in not loaded"); return;};
    let mut shutter = &day.data[..14];
    let mut start = 0;
    let mut message = 0;
    for i in 14..day.data.len() {
        if !contains_dup(&shutter[..4]) && start == 0 {start = i - 10;}
        if !contains_dup(shutter) {message = i; println!("{}", shutter); break;}
        shutter = &day.data[i-13..i+1];
    }
    if start == 0 {error!("No start-of-packet"); return;}
    println!("start-of-packet @ {}\nstart-of-message @ {}", start, message);
    let mut parent = commands.spawn((NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(85.)),
            position_type: PositionType::Absolute,
            position: UiRect::top(Val::Px(150.)),
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        ..Default::default()
    }, DayItem));
    const BARHIGHT: f32 = 100./28.;
    const BARSIZE: Size = Size {height: Val::Percent(BARHIGHT), width: Val::Percent(100./18.)};
    for i in start-4..start {
        parent.with_children(|p| {
            p.spawn(NodeBundle {
                background_color: Color::BLUE.into(),
                style: Style {
                    size: BARSIZE,
                    position: UiRect::top(Val::Percent(((day.data.bytes().nth(i).unwrap() as u8 - b'a') as f32) * BARHIGHT)),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    };
    for i in message-14..message {
        parent.with_children(|p| {
            p.spawn(NodeBundle {
                background_color: Color::RED.into(),
                style: Style {
                    size: BARSIZE,
                    position: UiRect::top(Val::Percent(((day.data.bytes().nth(i).unwrap() as u8 - b'a') as f32) * BARHIGHT)),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    }
}