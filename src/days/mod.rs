use bevy::{prelude::*, asset::{AssetLoader, LoadedAsset, LoadContext}, app::PluginGroupBuilder, reflect::TypeUuid};
use serde::{Deserialize, Serialize};

use crate::{advent_calendar::{CalendarAssets}, CalenderState};

mod day1;
mod day2;
mod day3;
mod day4;

pub struct DaysPlugin;
struct DayPlugin;

impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Day>();
        app.add_asset_loader(DayLoader);
        app.init_resource::<Days>();
        app.add_system(update_button);
    }
}

#[derive(Resource)]
struct Days(Vec<HandleUntyped>);

impl FromWorld for Days {
    fn from_world(world: &mut World) -> Self {
        let ass = world.resource::<AssetServer>();
        Days(ass.load_folder("days").unwrap())
    }
}

impl PluginGroup for DaysPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
        .add(DayPlugin)
        .add(day1::DayPlugin)
        .add(day2::DayPlugin)
        .add(day3::DayPlugin)
        .add(day4::DayPlugin)
    }
}

#[derive(Deserialize, Serialize, TypeUuid)]
#[uuid="711e45de-e047-4c73-8383-86c0b8c1773e"]
struct Day {
    tital: String,
    data: String,
}

fn spawn_day<const DAY: u8>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    days: Res<Assets<Day>>,
    assets: Res<CalendarAssets>,
    // advent_data: Res<AdventData>,
) {
    let Some(day) = days.get(&asset_server.load(&format!("days/day{}.day.ron", DAY))) else {error!("Day {} not loaded", DAY); return;};
    //let des = vec![spawn_description(&mut commands, "Click to See Task 1".to_string(), day.description, assets.text_font.clone())];
    // if advent_data.stars.is_gold(&AdventStar {day: DAY - 1, star: 0}) {
    //     des.push(
    //         spawn_description(&mut commands, "Click to See Task 2".to_string(), day.description_2, assets.text_font.clone())
    //     );
    // }
    commands.spawn((NodeBundle{
        style: Style {
            position_type: PositionType::Absolute,
            flex_wrap: FlexWrap::Wrap,
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            align_items: AlignItems::FlexStart,
            ..Default::default()
        },
        ..default()
    }, DayItem))
    .with_children(|p| {
        p.spawn((ButtonBundle {
            image: asset_server.load("home.png").into(),
            style: Style {
                size: Size::new(Val::Px(100.), Val::Px(100.)),
                position: UiRect::new(Val::Px(0.), Val::Auto, Val::Px(0.), Val::Auto),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        }, CalenderState::CalenderMenu));
        p.spawn(TextBundle {
            text: Text { sections: vec![TextSection {
                value: day.tital.clone(),
                style: TextStyle { font: assets.calender_font.clone(), font_size: 100., color: Color::BLACK },
            }], alignment: TextAlignment::CENTER},
            style: Style {
                margin: UiRect::horizontal(Val::Auto),
                ..Default::default()
            },
            ..Default::default()
        });
    });
}

#[derive(Component)]
struct DayItem;

#[derive(Component)]
struct Content;

struct DayLoader;

impl AssetLoader for DayLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move { load_day(bytes, load_context) })
    }

    fn extensions(&self) -> &[&str] {
        &["day.ron"]
    }
}

fn load_day<'a>(bytes: &'a [u8], load_context: &'a mut LoadContext) -> Result<(), bevy::asset::Error> {
    let mut day = ron::Deserializer::from_bytes(bytes)?;
        let day = Day::deserialize(&mut day)?;
        load_context.set_default_asset(LoadedAsset::new(day));
        Ok(())
}
fn spawn_description(
    commands: &mut Commands,
    short: String,
    val: String,
    font: Handle<Font>,
) -> Entity {
    commands.spawn(NodeBundle{z_index: ZIndex::Global(-1), ..Default::default()}).with_children(|p| {
        p.spawn(ButtonBundle{style: Style {position_type: PositionType::Absolute, ..Default::default()}, ..Default::default()}).with_children(|p| {
            p.spawn(TextBundle{
                text: Text { sections: vec![TextSection {
                    value: short,
                    style: TextStyle { font: font.clone(), font_size: 25., color: Color::BLACK },
                }], alignment: TextAlignment::default() },
                ..Default::default()
            });
        });
        p.spawn(ButtonBundle{style: Style {position_type: PositionType::Absolute, ..Default::default()},visibility: Visibility{is_visible: false},  ..Default::default()}).with_children(|p| {
            p.spawn(TextBundle{
                text: Text { sections: vec![TextSection {
                    value: val,
                    style: TextStyle { font: font.clone(), font_size: 20., color: Color::BLACK },
                }], alignment: TextAlignment::default() },
                ..Default::default()
            });
        });
    }).id()
}

fn update_button(
    mut vis: Query<&mut Visibility>,
    query: Query<(&Parent, &Interaction), (With<Button>, Changed<Interaction>)>,
    children: Query<&Children>,
) {
    for (p, i) in &query {
        if *i == Interaction::Clicked {
            for c in children.get(p.get()).expect("Parent to have childeren") {
                if let Ok(mut vis) = vis.get_mut(*c) {
                    vis.is_visible = !vis.is_visible;
                } else {
                    warn!("Child did not have vis");
                }
            }
        }
    }
}