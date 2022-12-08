use bevy::prelude::*;
use indexmap::IndexMap;

use crate::{CalenderState, advent_calendar::AdventData, elf::{ElfParts, Elf, ElfPart}};

use super::{DayItem, Day};

pub struct DayPlugin;

impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(CalenderState::Day1)
        .with_system(super::spawn_day::<1>)
        .with_system(read_data))
        .add_system_set(SystemSet::on_exit(CalenderState::Day1)
        .with_system(crate::cleanup::<DayItem>))
        .init_resource::<Rations>();
    }
}

#[derive(Resource)]
struct Rations(Vec<Handle<Image>>);
impl FromWorld for Rations {
    fn from_world(world: &mut World) -> Self {
        use rand::seq::SliceRandom;
        use rand::SeedableRng;
        let asset_server = world.resource::<AssetServer>();
        let advent_data = world.resource::<AdventData>();
        let mut rations: Vec<Handle<Image>> = asset_server.load_folder("rations").unwrap().into_iter().map(|r| r.typed()).collect();
        let mut rng = rand::rngs::StdRng::seed_from_u64(advent_data.rng_seed);
        rations.shuffle(&mut rng);
        Rations(rations)
    }
}

#[derive(Default, Resource)]
struct ElfData {
    elfs: IndexMap<Handle<Elf>, ElfRations>,
    min: usize,
    max: usize,
}

impl ElfData {
    fn add(&mut self, elf: ElfRations, id: Handle<Elf>) {
        for r in elf.rations.iter() {
            self.min = (*r).min(self.min);
            self.max = (*r).max(self.max);
        }
        self.elfs.insert(id, elf);
    }
}

#[derive(Default)]
struct ElfRations {
    rations: Vec<usize>,
    total: usize,
}

impl ElfRations {
    fn add(&mut self, ration: usize) {
        self.rations.push(ration);
        self.total += ration;
    }
}

fn read_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<Day>>,
    mut asset_elf: ResMut<Assets<Elf>>,
    elf_parts: Res<ElfParts>,
    advent_data: Res<AdventData>,
    rations: Res<Rations>,
    asset_parts: Res<Assets<ElfPart>>,
) {
    let handle = asset_server.load("days/day1.day.ron");
    let Some(day) = assets.get(&handle) else {error!("Day 1 Not Loaded"); return;};
    let mut elf_data = ElfData::default();
    let mut seed = advent_data.rng_seed;
    let mut elf_id = asset_elf.add(elf_parts.random_elf(seed));
    let mut current_elf = ElfRations::default();
    for line in day.data.lines() {
        let line = line.trim();
        if line.len() == 0 {
            elf_data.add(std::mem::take(&mut current_elf), elf_id);
            seed += 1;
            elf_id = asset_elf.add(elf_parts.random_elf(seed));
            continue;
        }
        let ration = line.parse().unwrap();
        current_elf.add(ration);
    }
    elf_data.add(std::mem::take(&mut current_elf), elf_id);
    let mut children = vec![];
    let range = (elf_data.max - elf_data.min) / rations.0.len();
    let mut most = 0;
    let mut most2 = 0;
    let mut most3 = 0;
    for (elf_handle, elf_rations) in elf_data.elfs.iter() {
        if elf_rations.total >= most3 {
            if elf_rations.total >= most2 {
                most3 = most2;
                if elf_rations.total >= most {
                    most2 = most;
                    most = elf_rations.total;
                }
                else {
                    most2 = elf_rations.total;
                }
            } else {
                most3 = elf_rations.total;
            }

        let elf = asset_elf.get(elf_handle).unwrap();
        let elf_id = elf.draw(asset_parts.as_ref(), &mut commands, 0.2).unwrap();
        let child = commands
        .spawn((NodeBundle{
            style: Style {
                justify_content: JustifyContent::FlexStart,
                size: Size::new(Val::Percent(100.), Val::Px(40.)),
                ..Default::default()
            },
            ..Default::default()
        }, elf_handle.clone()))
        .add_child(elf_id).with_children(|p| {
            for ration in elf_rations.rations.iter() {
                let ration_index = (ration - elf_data.min) / range;
                p.spawn(ImageBundle {
                    image: rations.0[ration_index % rations.0.len()].clone().into(),
                    style: Style {
                        size: Size::new(Val::Px(25.), Val::Px(25.)),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
        }).id();
        children.push(child);
        }
    }
    commands.spawn((NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            position_type: PositionType::Absolute,
            position: UiRect::top(Val::Px(150.)),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        ..Default::default()
    }, DayItem)).push_children(&children);
    println!("Out of {} elfs\nThe top 3 are = {}:{}:{};\nFor A Total of: {}", elf_data.elfs.len(), most, most2, most3, most+most2+most3);
    commands.insert_resource(elf_data);
}