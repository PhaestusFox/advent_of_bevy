use bevy::prelude::*;
use rand::{SeedableRng, Rng};

use crate::{CalenderState, advent_calendar::AdventData};

use super::{DayItem, Day};


pub struct DayPlugin;

impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(CalenderState::Day3)
        .with_system(super::spawn_day::<3>)
        .with_system(read_data))
        .add_system_set(SystemSet::on_exit(CalenderState::Day3)
        .with_system(crate::cleanup::<DayItem>))
        .init_resource::<Items>();
    }
}

#[derive(Resource)]
struct Items(Vec<Handle<Image>>);

impl FromWorld for Items {
    fn from_world(world: &mut World) -> Self {
        use rand::seq::SliceRandom;
        let advent_data = world.resource::<AdventData>();
        let mut rng = rand::rngs::StdRng::seed_from_u64(advent_data.rng_seed);
        let assert_server = world.resource::<AssetServer>();
        let mut items: Vec<Handle<Image>> = assert_server.load_folder("items").unwrap().into_iter().map(|f| f.typed()).collect();
        items.shuffle(&mut rng);
        Items(items)
    }
}

struct Bag {
    pocket_one: u64,
    pocket_two: u64,
}

#[derive(Debug)]
enum Pocket {
    PocketOne,
    PocketTwo,
}

impl Bag {
    fn add_item(&mut self, pocket: Pocket, item: char) -> Result<(), char> {
        if item < 'A' || item > 'z' || item < 'a' && item > 'Z' {
            error!("item {} can not go in pocket", item);
            return Ok(());
        }
        let item_type = if item <= 'Z' {
            item as u8 - b'A' + 26
        } else {
            item as u8 - b'a'
        };
        let id = 1 << item_type;
        match pocket {
            Pocket::PocketOne => {self.pocket_one |= id},
            Pocket::PocketTwo => {self.pocket_two |= id}
        }
        if self.pocket_one & self.pocket_two != 0 {
            self.pocket_two ^= id;
            Err(item)
        } else {
            Ok(())
        }
    }
    fn new() -> Bag {
        Bag { pocket_one: 0, pocket_two: 0 }
    }
    fn content(&self) -> u64 {
        self.pocket_one | self.pocket_two
    }
}

fn read_data(
    mut commands: Commands,
    days: Res<Assets<Day>>,
    assert_server: Res<AssetServer>,
    items: Res<Items>,
    windows: Res<Windows>,
    advent_data: Res<AdventData>,
) {
    let Some(day) = days.get(&assert_server.load("days/day3.day.ron")) else {error!("Day 3 in not loaded"); return;};
    let mut dups = Vec::new();
    let mut bags = Vec::new();
    for line in day.data.lines() {
        let line = line.trim();
        let half = line.len() / 2;
        let mut backpack = Bag::new();
        let mut dup = false;
        for (i, char) in line.chars().enumerate() {
            let pocket = if i < half {
                Pocket::PocketOne
            } else {
                Pocket::PocketTwo
            };
            if let Err(char) = backpack.add_item(pocket, char) {
                if !dup {
                    dups.push(char);
                }
                dup = true;
            }
        }
        bags.push(backpack);
    }
    let mut total = 0;
    let mut rng = rand::rngs::StdRng::seed_from_u64(advent_data.rng_seed);
    let p_window = windows.get_primary().unwrap();
    let width = p_window.width();
    let height = p_window.height();
    for char in dups {
        let id = if char <= 'Z' {
            (char as u8 - b'A' + 27) as usize
        } else {
            (char as u8 - b'a' + 1) as usize
        };
        let left = rng.gen();
        let top = rng.gen();
        let mut position = UiRect::all(Val::Auto);
        if left {
            position.left = Val::Px(rng.gen_range(0.0..width));
        } else {
            position.right = Val::Px(rng.gen_range(0.0..width));
        }
        if top {
            position.top = Val::Px(rng.gen_range(150.0..height));
        } else {
            position.bottom = Val::Px(rng.gen_range(0.0..height-150.));
        }
        commands.spawn((ImageBundle {
            image: items.0[id].clone().into(),
            style: Style {
                size: Size::new(Val::Px(50.), Val::Px(50.)),
                position,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        }, DayItem));
        total += id;
    }
    println!("Total = {}", total);
    let mut badge_total = 0;
    for i in (0..bags.len()).step_by(3) {
        let mut overlap = bags[i].content() & bags[i + 1].content() & bags[i +2].content();
        while overlap > 0 {
            badge_total += 1;
            overlap >>= 1;
        }
    }
    println!("badge total = {}", badge_total);
}