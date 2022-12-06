use bevy::{prelude::*, ui::FocusPolicy};
use serde::{Serialize, Deserialize};

use crate::CalenderState;

pub struct AdventPlugin;

impl Plugin for AdventPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CalendarAssets>();
        app.init_resource::<AdventData>();
        app.add_system(advent_buttons);
        app.add_system_set(SystemSet::on_enter(CalenderState::CalenderMenu)
            .with_system(setup_calender)
        )
        .add_system_set(SystemSet::on_pause(CalenderState::CalenderMenu)
            .with_system(super::hide_ui::<false, CalenderItem>)
        )
        .add_system_set(SystemSet::on_resume(CalenderState::CalenderMenu)
            .with_system(super::hide_ui::<true, CalenderItem>)
            .with_system(update_stars)
        )
        .add_system_set(SystemSet::on_exit(CalenderState::CalenderMenu)
            .with_system(super::cleanup::<CalenderItem>)
        );
    }
}

#[derive(Resource, Serialize, Deserialize)]
pub struct AdventData {
    pub rng_seed: u64,
    pub stars: Stars,
    #[serde(skip_serializing)]
    #[serde(default = "get_day")]
    day: u8,
}

fn get_day() -> u8 {
    use chrono::Datelike;
    let now = chrono::Utc::now();
    if now.month() == 12 {
        now.date_naive().day0() as u8
    } else {
        30
    }
}

impl AdventData {
    fn new() -> AdventData {
        AdventData {
            rng_seed: rand::random(), stars: Stars::default(), day: get_day(),
        }
    }
}

impl FromWorld for AdventData {
    fn from_world(_: &mut World) -> Self {
        let Ok(data) = std::fs::read_to_string("./advent.dat") else {
            let ad = AdventData::new();
            let _ = std::fs::write("./advent.dat", ron::to_string(&ad).unwrap());
            return ad;
        };
        ron::from_str(data.as_ref()).unwrap()
    }
}

#[derive(Component)]
struct CalenderItem;

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct Stars([[bool; 2]; 25]);

impl Stars {
    pub fn is_gold(&self, star: &AdventStar) -> bool {
        self.0[star.day as usize][star.star as usize]
    }
    pub fn set_gold(&mut self, star: &AdventStar) {
        self.0[star.day as usize][star.star as usize] = true;
    }
}

#[derive(Resource)]
pub(crate) struct CalendarAssets {
    pub gold_star: Handle<Image>,
    pub gray_star: Handle<Image>,
    pub black_star: Handle<Image>,
    pub calender_font: Handle<Font>,
    pub text_font: Handle<Font>,
}

impl FromWorld for CalendarAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let gold_star = asset_server.load("GoldStar.png");
        let gray_star = asset_server.load("GrayStar.png");
        let black_star = asset_server.load("BlackStar.png");
        let calender_font = asset_server.load("From Cartoon Blocks.ttf");
        let text_font = asset_server.load("Rabbit Hole.ttf");
        CalendarAssets { gold_star, gray_star, calender_font, text_font, black_star }
    }
}

const BOXSIZE: f32 = 100.;

fn setup_calender(
    assets: Res<CalendarAssets>,
    mut commands: Commands,
    advent_data: Res<AdventData>,
){
    use rand::SeedableRng;
    use rand::seq::SliceRandom;
    let mut boxs = Vec::new();
    let mut order = (0..25).collect::<Vec<u8>>();
    let mut rng = rand::rngs::StdRng::seed_from_u64(advent_data.rng_seed);
    order.shuffle(&mut rng);
    for i in order {
        let state: CalenderState = CalenderState::from_day(i + 1);
        boxs.push(commands.spawn((ButtonBundle {
            style: Style {
                size: Size { width: Val::Px(100.), height: Val::Px(100.) },
                margin: UiRect::right(Val::Px(5.)),
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            ..default()
        }, state)).with_children(|p| {
            p.spawn(TextBundle {
                text: Text { sections: vec![TextSection {
                    value: format!("{:02}", i + 1),
                    style: TextStyle { font: assets.calender_font.clone(), font_size: 70., color: Color::BLACK }
                }],
                alignment: TextAlignment::CENTER
                },
            style: Style {
                margin: UiRect::all(Val::Auto),
                size: Size::new(Val::Px(100.), Val::Auto),
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            ..Default::default()
            });
            p.spawn((ImageBundle {
                image: if advent_data.day < i {assets.black_star.clone().into()} else if advent_data.stars.is_gold(&AdventStar { day: i, star: 0 }){assets.gold_star.clone().into()} else {assets.gray_star.clone().into()},
                style: Style {
                    size: Size::new(Val::Px(10.), Val::Px(10.)),
                    ..default()
                },
                focus_policy: FocusPolicy::Pass,
                ..default()
            }, AdventStar{day: i, star: 0})
            );
            p.spawn((ImageBundle {
                image: if advent_data.day < i {assets.black_star.clone().into()} else if advent_data.stars.is_gold(&AdventStar { day: i, star: 1 }) {assets.gold_star.clone().into()} else {assets.gray_star.clone().into()},
                style: Style {
                    size: Size::new(Val::Px(10.), Val::Px(10.)),
                    ..default()
                },
                focus_policy: FocusPolicy::Pass,
                ..default()
            }, AdventStar{day: i, star: 1}),
            );
        }).id());
    }
    commands.spawn((NodeBundle {
        style: Style {
            position: UiRect { left: Val::Auto, right: Val::Auto, top: Val::Auto, bottom: Val::Auto },
            size: Size { width: Val::Px((BOXSIZE + 5.) * 5.), height: Val::Px((BOXSIZE + 5.) * 5.) },
            flex_wrap: FlexWrap::Wrap,
            align_content: AlignContent::SpaceBetween,
            align_self: AlignSelf::Center,
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        ..Default::default()
    }, CalenderItem)).push_children(&boxs);
}

#[derive(Component)]
pub struct AdventStar {
    pub day: u8,
    pub star: u8,
}

fn advent_buttons(
    query: Query<(&Interaction, &CalenderState), (Changed<Interaction>, With<Button>)>,
    mut res: ResMut<State<CalenderState>>,
){
    for (interaction, state) in &query {
        if interaction == &Interaction::Clicked {
            info!("set state to {:?}", state);
            let _ = res.set(*state);
            return;
        }
    }
}

fn update_stars(
    mut query: Query<(&mut UiImage, &AdventStar)>,
    assets: Res<CalendarAssets>,
    data: Res<AdventData>,
) {
    for (mut image, star) in &mut query {
        if data.stars.is_gold(star) {
            image.0 = assets.gold_star.clone();
        }
    }
}