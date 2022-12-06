use bevy::prelude::*;

use crate::{CalenderState, advent_calendar::CalendarAssets};

use super::{DayItem, Day};


pub struct Day2Plugin;

impl Plugin for Day2Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(CalenderState::Day2)
        .with_system(super::spawn_day::<2>)
        .with_system(read_data))
        .add_system_set(SystemSet::on_exit(CalenderState::Day2)
        .with_system(crate::cleanup::<DayItem>))
        .init_resource::<Hands>();
    }
}

#[derive(Default)]
struct StrategyGuide{
    moves: Vec<(Move, Move)>,
    total_score: usize,
}

#[derive(Resource)]
struct Hands {
    rock: Handle<Image>,
    paper: Handle<Image>,
    scissors: Handle<Image>,
}

impl Hands {
    fn get_hand(&self, hand: &Move) -> Handle<Image> {
        match hand {
            Move::Rock => self.rock.clone(),
            Move::Paper => self.paper.clone(),
            Move::Scissors => self.scissors.clone(),
        }
    }
}

impl FromWorld for Hands {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Hands { rock: asset_server.load("rock.png"), paper: asset_server.load("paper.png"), scissors: asset_server.load("scissors.png") }
    }
}

impl StrategyGuide {
    fn play_move(&mut self, there_move: Move, your_move: Move) {
        let outcome = your_move.vs(&there_move);
        self.total_score += outcome as usize + your_move as usize;
        self.moves.push((there_move, your_move));
    }
}

fn read_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<Day>>,
    hands: Res<Hands>,
    calender_assets: Res<CalendarAssets>,
) {
    let Some(day) = assets.get(&asset_server.load("days/day2.day.ron")) else {error!("Day 2 not loaded"); return;} ;
    let mut game0 = StrategyGuide::default();
    let mut perfect_game = StrategyGuide::default();
    let mut game1 = StrategyGuide::default();
    for line in day.data.lines() {
        let line = line.trim();
        let there_move = match &line[..1].chars().next().unwrap() {
            'A' => {Move::Rock},
            'B' => {Move::Paper},
            'C' => {Move::Scissors},
            a => {error!("{} is not a vailid input", a); continue;}
        };
        let (your_move0, your_move1) = match &line[line.len()-1..].chars().next().unwrap() {
            'X' => {(Move::Rock ,OutCome::Loss.agains(&there_move))},
            'Y' => {(Move::Paper ,OutCome::Draw.agains(&there_move))},
            'Z' => {(Move::Scissors ,OutCome::Win.agains(&there_move))},
            a => {error!("{} is not a vailid input", a); continue;}
        };
        game0.play_move(there_move, your_move0);
        perfect_game.play_move(there_move, OutCome::Win.agains(&there_move));
        game1.play_move(there_move, your_move1);
    }
    println!("Strategy 1 Score = {}", game0.total_score);
    println!("Strategy 2 Score = {}", game1.total_score);
    println!("Perfect Score = {}", perfect_game.total_score);
    println!("Total plays = {}", perfect_game.moves.len());
    let games_skip = perfect_game.moves.len() / 100;
    let mut there_moves = vec![];
    let mut children_g0 = vec![];
    let mut children_g1 = vec![];
    let mut children_pg = vec![];
    // let mut gc = 0;
    const HANDSIZE: f32 = 50.;
    const FONT_SIZE: f32 = 20.;
    for (i,((there_move, g0),((_, g1),(_, pg)))) in game0.moves.iter().zip(game1.moves.iter().zip(perfect_game.moves.iter())).enumerate().step_by(games_skip) {
        let hand_style = Style {
                size: Size::new(Val::Px(HANDSIZE), Val::Px(HANDSIZE)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            };
        there_moves.push(commands.spawn(ImageBundle {
            image: hands.get_hand(there_move).into(),
            style: hand_style.clone(),
            ..Default::default()
        }).with_children(|p| {
            p.spawn(TextBundle {
                text: Text { sections: vec![TextSection{
                    value: i.to_string(),
                    style: TextStyle { font: calender_assets.text_font.clone(), font_size: FONT_SIZE, color: Color::PURPLE }
                }], alignment: TextAlignment::CENTER },
                ..Default::default()
            });
        }).id());
        children_g0.push(commands.spawn(ImageBundle {
            image: hands.get_hand(g0).into(),
            style: hand_style.clone(),
            ..Default::default()
        }).with_children(|p| {
            p.spawn(TextBundle {
                text: Text { sections: vec![TextSection{
                    value: i.to_string(),
                    style: TextStyle { font: calender_assets.text_font.clone(), font_size: FONT_SIZE, color: Color::MIDNIGHT_BLUE }
                }], alignment: TextAlignment::CENTER },
                ..Default::default()
            });
        }).id());
        children_g1.push(commands.spawn(ImageBundle {
            image: hands.get_hand(g1).into(),
            style: hand_style.clone(),
            ..Default::default()
        }).with_children(|p| {
            p.spawn(TextBundle {
                text: Text { sections: vec![TextSection{
                    value: i.to_string(),
                    style: TextStyle { font: calender_assets.text_font.clone(), font_size: FONT_SIZE, color: Color::GOLD }
                }], alignment: TextAlignment::CENTER },
                ..Default::default()
            });
        }).id());
        children_pg.push(commands.spawn(ImageBundle {
            image: hands.get_hand(pg).into(),
            style: hand_style,
            ..Default::default()
        }).with_children(|p| {
            p.spawn(TextBundle {
                text: Text { sections: vec![TextSection{
                    value: i.to_string(),
                    style: TextStyle { font: calender_assets.text_font.clone(), font_size: FONT_SIZE, color: Color::DARK_GREEN }
                }], alignment: TextAlignment::CENTER },
                ..Default::default()
            });
        }).id());
    }
    let mut style = Style {
        size: Size::new(Val::Percent(25.), Val::Percent(85.)),
        position_type: PositionType::Absolute,
        position: UiRect::new(Val::Percent(0.), Val::Auto, Val::Px(150.), Val::Auto),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        flex_wrap: FlexWrap::Wrap,
        ..Default::default()
    };
    commands.spawn(NodeBundle {
        style: style.clone(),
        ..Default::default()
    }).push_children(&there_moves);
    style.position.left = Val::Percent(25.);
    commands.spawn(NodeBundle {
        style: style.clone(),
        ..Default::default()
    }).push_children(&children_g0);
    style.position.left = Val::Percent(50.);
    commands.spawn(NodeBundle {
        style: style.clone(),
        ..Default::default()
    }).push_children(&children_g1);
    style.position.left = Val::Percent(75.);
    commands.spawn(NodeBundle {
        style,
        ..Default::default()
    }).push_children(&children_pg);
}

#[derive(Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn vs(&self, other: &Move) -> OutCome {
        use OutCome::*;
        match (self, other) {
            (Move::Rock, Move::Rock) => Draw,
            (Move::Rock, Move::Paper) => Loss,
            (Move::Rock, Move::Scissors) => Win,
            (Move::Paper, Move::Rock) => Win,
            (Move::Paper, Move::Paper) => Draw,
            (Move::Paper, Move::Scissors) => Loss,
            (Move::Scissors, Move::Rock) => Loss,
            (Move::Scissors, Move::Paper) => Win,
            (Move::Scissors, Move::Scissors) => Draw,
        }
    }
}

enum OutCome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl OutCome {
    fn agains(&self, other: &Move) -> Move {
        use OutCome::*;
        use Move::*;
        match (self, other) {
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
            (Draw, Rock) => Rock,
            (Draw, Paper) => Paper,
            (Draw, Scissors) => Scissors,
            (Loss, Rock) => Scissors,
            (Loss, Paper) => Rock,
            (Loss, Scissors) => Paper,
        }
    }
}