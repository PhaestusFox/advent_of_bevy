use bevy::{prelude::*, render::texture::ImageSampler};

mod advent_calendar;
mod days;
mod elf;
mod utils;

fn main() {
    println!("Hello, bevy!");
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSampler::nearest_descriptor(),
        }).set(AssetPlugin {watch_for_changes: true, ..Default::default()}))
        .add_plugin(bevy_editor_pls::EditorPlugin)
        .add_state(CalenderState::CalenderMenu)
        .add_startup_system(spawn_cam)
        .add_plugin(advent_calendar::AdventPlugin)
        .add_plugins(days::DaysPlugin)
        .add_plugin(elf::ElfPlugin)
        .run();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
enum CalenderState {
    CalenderMenu,
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl CalenderState {
    fn from_day(day: u8) -> Self {
        use CalenderState::*;
        match day {
            1 => Day1,
            2 => Day2,
            3 => Day3,
            4 => Day4,
            5 => Day5,
            6 => Day6,
            7 => Day7,
            8 => Day8,
            9 => Day9,
            10 => Day10,
            11 => Day11,
            12 => Day12,
            13 => Day13,
            14 => Day14,
            15 => Day15,
            16 => Day16,
            17 => Day17,
            18 => Day18,
            19 => Day19,
            20 => Day20,
            21 => Day21,
            22 => Day22,
            23 => Day23,
            24 => Day24,
            25 => Day25,
            _ => CalenderMenu,
        }
    }
}

fn cleanup<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
){
    println!("Runing Cleanup");
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_cam(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


fn hide_ui<const TO: bool, T: Component>(
    mut query: Query<&mut Visibility, With<T>>
) {
    for mut vis in &mut query {
        vis.is_visible = TO;
    }
}