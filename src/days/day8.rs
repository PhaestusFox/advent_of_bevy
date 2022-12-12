use std::collections::HashMap;

use bevy::prelude::*;
use rand::{SeedableRng, Rng, seq::SliceRandom};

use crate::{CalenderState, advent_calendar::{CalendarAssets, AdventData}};

use super::{DayItem, Day};

pub struct DayPlugin;

impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(CalenderState::Day8)
        .with_system(super::spawn_day::<8>)
        .with_system(read_data))
        .add_system_set(SystemSet::on_exit(CalenderState::Day8)
        .with_system(crate::cleanup::<DayItem>))
        .init_resource::<Trees>();
    }
}

#[derive(Resource)]
struct Trees(Vec<Handle<Image>>);

impl FromWorld for Trees {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let seed = world.resource::<AdventData>();
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed.rng_seed);
        let mut handle: Vec<Handle<Image>> = asset_server.load_folder("trees").unwrap().into_iter().map(|f| f.typed()).collect();
        handle.shuffle(&mut rng);
        Trees(handle)
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Tree {
    x: usize,
    y: usize,
}

impl Tree {
    fn new(x: usize, y: usize) -> Tree {
        Tree { x, y }
    }
}

struct Forest {
    width: usize,
    hight: usize,
    trees: HashMap<Tree, u8>
}

impl Forest {
    fn new() -> Forest {
        Forest { width: 0, hight: 0, trees: HashMap::default() }
    }
    fn is_visible(&self, tree: &Tree) -> bool {
        let hight = self.trees.get(tree).unwrap();
        if tree.x == 0 || tree.y == 0 || tree.x == self.width - 1 || tree.y == self.hight - 1 {return true;}
        let mut vis_x = true;
        let mut vis_xn = true;
        let mut vis_y = true;
        let mut vis_yn = true;
        for x in 0..tree.x {
            if self.trees.get(&Tree { x, y: tree.y }).unwrap() >= hight {
                vis_x = false;
            }
        }
        for x in (tree.x..self.width).skip(1) {
            if self.trees.get(&Tree { x, y: tree.y }).unwrap() >= hight {
                vis_xn = false;
            }
        }
        for y in 0..tree.y {
            if self.trees.get(&Tree { x: tree.x, y}).unwrap() >= hight {vis_y = false;}
        }
        for y in (tree.y..self.hight).skip(1) {
            if self.trees.get(&Tree { x: tree.x, y}).unwrap() >= hight {vis_yn = false;}
        }
        vis_x || vis_xn || vis_y || vis_yn
    }
    fn scenic(&self, tree: &Tree) -> usize {
        if tree.x == 0 || tree.y == 0 || tree.x == self.width - 1 || tree.y == self.hight - 1 {return 0;}
        let hight = self.trees.get(tree).unwrap();
        let mut vis_x = self.width - tree.x - 1;
        let mut vis_xn = tree.x;
        let mut vis_y = self.hight - tree.y - 1;
        let mut vis_yn = tree.y;
        for (i, x) in (0..tree.x).enumerate().skip(1) {
            let x = tree.x - x;
            if self.trees.get(&Tree { x, y: tree.y }).unwrap() >= hight {
                vis_xn = i;
                break;
            }
        }
        for (i, x) in (tree.x..self.width).enumerate().skip(1) {
            if self.trees.get(&Tree { x, y: tree.y }).unwrap() >= hight {
                vis_x = i;
                break;
            }
        }
        for (i, y) in (0..tree.y).enumerate().skip(1) {
            let y = tree.y - y;
            if self.trees.get(&Tree { x: tree.x, y}).unwrap() >= hight {
                vis_yn = i; break;}
        }
        for (i, y) in (tree.y..self.hight).enumerate().skip(1) {
            if self.trees.get(&Tree { x: tree.x, y}).unwrap() >= hight {vis_y = i; break;}
        }
        vis_x * vis_y * vis_xn * vis_yn
    }
    fn add_tree(&mut self, tree: Tree, hight: u8) {
        self.width = (tree.x + 1).max(self.width);
        self.hight = (tree.y + 1).max(self.hight);
        self.trees.insert(tree, hight);
    }
}

fn read_data(
    mut commands: Commands,
    days: Res<Assets<Day>>,
    assert_server: Res<AssetServer>,
    trees: Res<Trees>,
) {
    let Some(day) = days.get(&assert_server.load("days/day8.day.ron")) else {error!("Day 8 in not loaded"); return;};
    let mut forest = Forest::new();
    for (y, line) in day.data.lines().enumerate() {
        for (x, char) in line.char_indices() {
            forest.add_tree(Tree::new(x, y), char as u8 - 0x30);
        }
    }
    println!("Forest is {}x{}", forest.width, forest.hight);
    let mut visible = 0;
    let mut scenic = 0;
    let mut scenic_tree = Tree::new(0, 0);
    let root = commands.spawn((NodeBundle {
        style: Style {
            align_self: AlignSelf::Center,
            position: UiRect::top(Val::Px(150.)),
            size: Size::new(Val::Percent(85.), Val::Percent(85.)),
            flex_wrap: FlexWrap::Wrap,
            margin: UiRect::all(Val::Auto),
            ..Default::default()
        },
        ..Default::default()
    }, DayItem)).id();
    for (tree, hight) in forest.trees.iter().filter(|(t, _)| forest.is_visible(t)) {
        commands.spawn(ImageBundle {
            image: trees.0[*hight as usize].clone().into(),
            style: Style {
                size: Size::new(Val::Percent(200./forest.width as f32), Val::Percent(400./forest.hight as f32)),
                ..Default::default()
            },
            ..Default::default()
        }).set_parent(root);
        let new_scenic = forest.scenic(tree);
        if new_scenic > scenic {
            scenic = new_scenic;
            scenic_tree = *tree;
        }
        visible += 1;
    }
    println!("{} are visible", visible);
    println!("most scenic is {}:{} with a score of {}",scenic_tree.x, scenic_tree.y, scenic);
}