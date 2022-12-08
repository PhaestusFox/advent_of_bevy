use bevy::prelude::*;
use rand::{SeedableRng, Rng, seq::SliceRandom};

use crate::{CalenderState, advent_calendar::{CalendarAssets, AdventData}};

use super::{DayItem, Day};


pub struct DayPlugin;

impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(CalenderState::Day5)
        .with_system(super::spawn_day::<5>)
        .with_system(read_data))
        .add_system_set(SystemSet::on_exit(CalenderState::Day5)
        .with_system(crate::cleanup::<DayItem>))
        .init_resource::<Crates>();
    }
}

#[derive(Resource)]
struct Crates(Vec<Handle<Image>>);

impl FromWorld for Crates {
    fn from_world(world: &mut World) -> Self {
        let advent_data = world.resource::<AdventData>();
        let mut rng = rand::rngs::StdRng::seed_from_u64(advent_data.rng_seed);
        let asset_server = world.resource::<AssetServer>();
        let mut crates: Vec<Handle<Image>> = asset_server.load_folder("crates").unwrap().into_iter().map(|f| f.typed()).collect();
        crates.shuffle(&mut rng);
        Crates(crates)
    }
}

#[derive(Clone)]
struct Ship {
    stacks: Vec<Vec<char>>,
    steps: Vec<Step>,
}
#[derive(Debug, Clone, Copy)]
struct Step {
    from: usize,
    to: usize,
    x: usize,
}
impl Step {
    fn from_str(str: &str) -> Result<Step, Day5Error> {
        let str = str.trim();
        let mut words = str.split(' ');
        if words.next().ok_or(Day5Error::TooShort)? != "move" {return Err(Day5Error::NoMove);}
        let x = words.next().ok_or(Day5Error::TooShort)?.parse().or(Err(Day5Error::NotNum("X")))?;
        if words.next().ok_or(Day5Error::TooShort)? != "from" {return Err(Day5Error::NoFrom);}
        let from = words.next().ok_or(Day5Error::TooShort)?.parse().or(Err(Day5Error::NotNum("From")))?;
        if words.next().ok_or(Day5Error::TooShort)? != "to" {return Err(Day5Error::NoTo);}
        let to = words.next().ok_or(Day5Error::TooShort)?.parse().or(Err(Day5Error::NotNum("To")))?;
        Ok(Self { from, to, x })
    }
}
#[derive(Debug)]
enum Day5Error {
    NoMove,
    NoFrom,
    NoTo,
    TooShort,
    NotNum(&'static str),
}

impl Ship {
    fn from_str(str: &str) -> Ship {
        let mut lines = str.lines().peekable();
        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
        while if let Some(v) = lines.peek() {v.contains('[')} else {false} {
            let mut chars = lines.next().unwrap().chars();
            let mut stack = 0;
            while let Some(char) = chars.next() {
                if char == '[' {
                    stacks[stack].push(chars.next().unwrap());
                }
                if char == ']' {
                    stack += 1;
                    chars.next();
                }
                if char == ' ' {
                    stack += 1;
                    chars.next();
                    chars.next();
                    chars.next();
                }
            }
        }
        lines.next(); //skip stack id line
        for stack in stacks.iter_mut() {
            stack.reverse();
        }
        let mut ship = Ship { stacks, steps: Vec::new()};
        for line in lines {
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }
            ship.add_step(Step::from_str(line).unwrap());
        }
        ship
    }

    fn apply_steps(&mut self, single: bool) {
        for Step{ from, to, x } in self.steps.iter() {
            let mut from_vec = std::mem::take(&mut self.stacks[*from - 1]);
            if single {
                apply_step(&mut from_vec, &mut self.stacks[*to - 1], *x);
            } else {
                apply_muti_step(&mut from_vec, &mut self.stacks[*to - 1], *x);
            }
            std::mem::swap(&mut self.stacks[*from - 1], &mut from_vec);
        }
    }

    fn add_step(&mut self, step: Step) {
        self.steps.push(step);
    }
}

fn apply_step(from: &mut Vec<char>, to: &mut Vec<char>, x: usize) {
    for _ in 0..x {
        let out = from.pop();
        if let Some(c) = out {to.push(c)}
    }
}
fn apply_muti_step(from: &mut Vec<char>, to: &mut Vec<char>, x: usize) {
    let mut stack = Vec::with_capacity(x);
    for _ in 0..x {
        if let Some(c) = from.pop() {stack.push(c)}
    }
    while let Some(c) = stack.pop() {
        to.push(c);
    }
}

fn read_data(
    mut commands: Commands,
    days: Res<Assets<Day>>,
    assert_server: Res<AssetServer>,
    calendar_assets: Res<CalendarAssets>,
    crates: Res<Crates>,
) {
    let Some(day) = days.get(&assert_server.load("days/day5.day.ron")) else {error!("Day 5 in not loaded"); return;};
    let mut ship = Ship::from_str(&day.data);
    let mut ship1 = ship.clone();
    ship.apply_steps(true);
    ship1.apply_steps(false);
    let mut tops = String::with_capacity(9);
    let mut mtops = String::with_capacity(9);
    for stack in ship.stacks.iter() {
        if let Some(c) = stack.last() {
            tops.push(*c);
        }
    }
    for stack in ship1.stacks.iter() {
        if let Some(c) = stack.last() {
            mtops.push(*c);
        }
    }
    println!("Single Tops are {}\nMultiple Tops are {}", tops, mtops);
    const STACKSIZE: f32 = 100./9.;
    let mut stack_style = Style {
        position: UiRect::new(Val::Percent(0.), Val::Auto, Val::Px(150.), Val::Auto),
        size: Size::new(Val::Percent(STACKSIZE), Val::Auto),
        position_type: PositionType::Absolute,
        flex_direction: FlexDirection::ColumnReverse,
        min_size: Size::new(Val::Percent(STACKSIZE), Val::Percent(85.)),
        ..Default::default()
    };
    let container_style = Style {
        size: Size::new(Val::Percent(100.), Val::Auto),
        aspect_ratio: Some(1.0),
        align_content: AlignContent::Center,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };
    let text_style = TextStyle {
        font_size: 50.,
        font: calendar_assets.text_font.clone(),
        color: Color::WHITE,
    };
    for (i, stack) in ship1.stacks.iter().enumerate() {
        stack_style.position.left = Val::Percent(i as f32 * STACKSIZE);
        commands.spawn((NodeBundle {
            style: stack_style.clone(),
            ..Default::default()
        }, DayItem)).with_children(|p| {
            for container in stack {
                p.spawn(ImageBundle {
                    image: crates.0[(*container as u8 - b'A') as usize].clone().into(),
                    style: container_style.clone(),
                    ..Default::default()
                }).with_children(|p| {
                    p.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {value: container.to_string(), style: text_style.clone()}],
                            alignment: TextAlignment::CENTER,
                        },
                        style: Style {
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
            }
        });
    }
}