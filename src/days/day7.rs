use std::collections::HashMap;

use bevy::prelude::*;
use rand::{SeedableRng, Rng, seq::SliceRandom};

use crate::{CalenderState, advent_calendar::{CalendarAssets, AdventData}};

use super::{DayItem, Day};

pub struct DayPlugin;

impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(CalenderState::Day7)
        .with_system(super::spawn_day::<7>)
        .with_system(read_data))
        .add_system_set(SystemSet::on_exit(CalenderState::Day7)
        .with_system(crate::cleanup::<DayItem>));
    }
}

fn read_data(
    mut commands: Commands,
    days: Res<Assets<Day>>,
    assert_server: Res<AssetServer>,
    calender_assets: Res<CalendarAssets>,
) {
    let Some(day) = days.get(&assert_server.load("days/day7.day.ron")) else {error!("Day 7 in not loaded"); return;};
    let mut fs = FileSystem::new();
    let mut lines = day.data.lines().peekable();
    while let Some(line) = lines.next() {
        let mut words = line.split(' ');
        let Some(w) = words.next() else {error!("Need at least one Word"); continue;};
        if w != "$" {error!("commands start with $"); continue;};
        match words.next() {
            Some("cd") => {
                let Some(path) = words.next() else {error!("No path after cd"); continue;};
                fs.cd(path);
            },
            Some("ls") => {
                while if let Some(next) = lines.peek() {!next.starts_with('$')} else {false} {
                    let mut segs = lines.next().unwrap().split(' ');
                    match segs.next() {
                        Some("dir") => {
                            if let Some(name) = segs.next() {
                                if let Err(e) = fs.add_dir(name) {
                                    error!("{:?}", e);
                                }
                            } else {
                                error!("Expect File Name");
                            }
                        }
                        Some(size) => {
                            let size = match size.parse::<usize>() {
                                Ok(v) => v,
                                Err(e) => {error!("{:?}", e); continue;}
                            };
                            if let Some(name) = segs.next() {
                                if let Err(e) = fs.add_file(name, size) {
                                    error!("{:?}", e);
                                }
                            } else {
                                error!("Expect File Name");
                            }
                        },
                        None => {error!("Expect size or dir of file");}
                    }
                }
            },
            Some(_) |
            None => {error!("command needs to be followed by ls or cd");}
        }
    }
    println!("Sum of small dir = {}", fs.sum_small());
    let free = 70000000-fs.items.size();
    println!("FS space: 70000000; {} free;", free);
    let need = 30000000-free;
    println!("Need: 30000000 free; missing {};", need);
    let delete = find_delete(&fs.items, need);
    let delete_name = find_name(&fs.items, delete);
    println!("delete: {} to free {}", delete_name, delete);
    let root = commands.spawn((NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            position: UiRect::top(Val::Px(150.)),
            size: Size::new(Val::Percent(100.), Val::Percent(85.)),
            ..Default::default()
        },
        ..Default::default()
    }, DayItem)).id();
    draw_fs(&mut commands, &fs.items, root, &calender_assets.text_font);   
}

fn draw_fs(commands: &mut Commands, item: &Item, root: Entity, font: &Handle<Font>) {
    if let Item::Dir(items) = item {
        commands.entity(root).with_children( |p| {
            for (name, val) in items.iter() {
                match val {
                    Item::File(size) => {
                        p.spawn(TextBundle {
                            text: Text { sections: vec![TextSection {
                                value: format!("{}:{}", name, size),
                                style: TextStyle { font: font.clone(), font_size: 10., color: Color::BLACK },
                            }], alignment: TextAlignment::CENTER },
                            ..Default::default()
                        });
                    },
                    Item::Dir(_) => {
                        p.spawn(ButtonBundle::default()).with_children(|p| {p.spawn(TextBundle {
                            text: Text { sections: vec![TextSection {
                                value: format!("{}: Dir", name),
                                style: TextStyle { font: font.clone(), font_size: 10., color: Color::BLACK },
                            }], alignment: TextAlignment::CENTER },
                            ..Default::default()
                        });});
                    }
                }
            }
        });
    }
}

fn find_delete(item: &Item, min: usize) -> usize {
    match item {
        Item::File(_) => usize::MAX,
        Item::Dir(items) => {
            let size = item.size();
            let sub_min = items.values().map(|i| find_delete(i, min)).min().unwrap_or(0);
            if sub_min > min && sub_min != usize::MAX {
                sub_min
            } else if size > min {
                size
            } else {
                usize::MAX
            }
        }
    }
}

fn find_name(item: &Item, size: usize) -> String {
    match item {
        Item::File(_) => {"File".to_string()},
        Item::Dir(items) => {
            for (name, item) in items.iter() {
                if item.size() == size {return name.to_string();}
                let sub_name = find_name(item, size);
                if sub_name != "File" && sub_name != "Dir" {return name.to_string()};
            }
            return "Dir".to_string();
        }
    }
}

struct FileSystem {
    items: Item,
    current_path: Vec<String>,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem { items: Item::Dir(HashMap::new()), current_path: Vec::new() }
    }
    fn cd(&mut self, to: &str) {
        if to == ".." {
            self.current_path.pop();
        } else if to == "/" {
            self.current_path.clear();
        } else {
            self.current_path.push(to.to_string());
        }
    }
    fn ls(&self) -> Result<Vec<String>, FSError> {
        let mut dir = &self.items;
        let mut p = Vec::with_capacity(self.current_path.len());
        for path in self.current_path.iter() {
            p.push(path);
            if let Item::Dir(items) = dir {
                dir = items.get(path).ok_or(FSError::NoSuchFileOrDir(format!("{:?}", p)))?;
            } else {
                return Err(FSError::NotDir(path.clone()));
            }
        }
        if let Item::Dir(items) = dir {
            Ok(items.keys().cloned().collect())
        } else {
            Err(FSError::NotDir(format!("{:?}", p)))
        }
    }
    fn add_file(&mut self, name: &str, size: usize) -> Result<(), FSError> {
        let mut dir = &mut self.items;
        for path in self.current_path.iter() {
            if let Item::Dir(items) = dir {
                dir = items.get_mut(path).ok_or(FSError::NoSuchFileOrDir(path.to_string()))?;
            } else {
                return Err(FSError::NotDir(path.clone()));
            }
        }
        if let Item::Dir(items) = dir {
            items.insert(name.to_string(), Item::File(size));
        }
        Ok(())
    }
    fn add_dir(&mut self, name: &str) -> Result<(), FSError> {
        let mut dir = &mut self.items;
        for path in self.current_path.iter() {
            if let Item::Dir(items) = dir {
                dir = items.get_mut(path).ok_or(FSError::NoSuchFileOrDir(path.to_string()))?;
            } else {
                return Err(FSError::NotDir(path.clone()));
            }
        }
        if let Item::Dir(items) = dir {
            items.insert(name.to_string(), Item::Dir(HashMap::new()));
        }
        Ok(())
    }
    fn sum_small(&self) -> usize {
        self.items.filter_dir(100000)
    }
}

#[derive(Debug)]
enum Item {
    Dir(HashMap<String, Item>),
    File(usize),
}

impl Item {
    fn size(&self) -> usize {
        match self {
            Item::File(size) => *size,
            Item::Dir(items) => {
                items.values().map(|i| i.size()).sum()
            }
        }
    }
    fn filter_dir(&self, size: usize) -> usize {
        match self {
            Item::File(_) => 0,
            Item::Dir(items) => {
                let sum: usize = items.values().map(|i| i.filter_dir(size)).sum();
                let d_size = self.size();
                sum + if d_size <= size {d_size} else {0}
            },
        }
    }
}

#[derive(Debug)]
enum FSError {
    NotDir(String),
    NoSuchFileOrDir(String),
}

struct Dir;