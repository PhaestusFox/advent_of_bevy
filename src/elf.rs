use std::collections::HashMap;

use bevy::{prelude::*, sprite::Anchor, reflect::TypeUuid, asset::{AssetLoader, LoadedAsset}};
use serde::Deserialize;

use crate::advent_calendar::AdventData;

pub struct ElfPlugin;

impl Plugin for ElfPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset_loader(ElfPartLoader);
        app.add_asset::<Elf>();
        app.add_asset::<ElfPart>();
        app.init_resource::<ElfParts>();
    }
}

#[derive(TypeUuid)]
#[uuid="a215c642-c161-4caa-abbc-50ea2f23e302"]
pub struct Elf {
    pub hat: Handle<ElfPart>,
    pub head: Handle<ElfPart>,
    pub body: Handle<ElfPart>,
    pub legs: Handle<ElfPart>,
}

#[derive(Resource)]
pub struct ElfParts {
    pub heads: Vec<Handle<ElfPart>>,
    pub hats: Vec<Handle<ElfPart>>,
    pub bodys: Vec<Handle<ElfPart>>,
    pub legs: Vec<Handle<ElfPart>>,
}

impl ElfParts {
    pub fn random_elf(&self, seed: u64) -> Elf {
        use rand::Rng;
        use rand::SeedableRng;
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        Elf {
            hat:  self.hats [rng.gen_range(0..self.hats.len())].clone(),
            head: self.heads[rng.gen_range(0..self.heads.len())].clone(),
            body: self.bodys[rng.gen_range(0..self.bodys.len())].clone(),
            legs: self.legs [rng.gen_range(0..self.legs.len())].clone(),
        }
    }
}

impl FromWorld for ElfParts {
    fn from_world(world: &mut World) -> Self {
        use rand::{seq::SliceRandom, SeedableRng};
        let asset_server = world.resource::<AssetServer>();
        let advent_data = world.resource::<AdventData>();
        let mut rng = rand::rngs::StdRng::seed_from_u64(advent_data.rng_seed);
        let mut heads :Vec<Handle<ElfPart>>= asset_server.load_folder("elf/head").unwrap().into_iter().map(|f| f.typed()).collect();
        let mut hats  :Vec<Handle<ElfPart>>= asset_server.load_folder("elf/hat").unwrap().into_iter().map(|f|  f.typed()).collect();
        let mut bodys :Vec<Handle<ElfPart>>= asset_server.load_folder("elf/body").unwrap().into_iter().map(|f| f.typed()).collect();
        let mut legs  :Vec<Handle<ElfPart>>= asset_server.load_folder("elf/legs").unwrap().into_iter().map(|f| f.typed()).collect();
        hats.shuffle(&mut rng);
        heads.shuffle(&mut rng);
        bodys.shuffle(&mut rng);
        legs.shuffle(&mut rng);
        ElfParts { heads, hats, bodys, legs }
    }
}

fn scale_ui_rect(mut rect: UiRect, scale: f32) -> UiRect {
    use Val::*;
    match rect.top {
        Px(v) => rect.top = Px(v * scale),
        Percent(v) => rect.top = Percent(v * scale),
        _ => {},
    }
    match rect.bottom {
        Px(v) => rect.bottom = Px(v * scale),
        Percent(v) => rect.bottom = Percent(v * scale),
        _ => {},
    }
    match rect.left {
        Px(v) => rect.left = Px(v * scale),
        Percent(v) => rect.left = Percent(v * scale),
        _ => {},
    }
    match rect.right {
        Px(v) => rect.right = Px(v * scale),
        Percent(v) => rect.right = Percent(v * scale),
        _ => {},
    }
    rect
}

fn scale_size(mut size: Size, scale: f32) -> Size {
    use Val::*;
    match size.height {
        Px(v) => size.height = Px(v * scale),
        Percent(v) => size.height = Percent(v * scale),
        _ => {},
    }
    match size.width {
        Px(v) => size.width = Px(v * scale),
        Percent(v) => size.width = Percent(v * scale),
        _ => {},
    }
    size
}

impl Elf {
    pub fn draw(
        &self,
        assets: &Assets<ElfPart>,
        commands: &mut Commands,
        scale: f32,
    ) -> Option<Entity> {
        let Some(body) = assets.get(&self.body) else {error!("body not loaded"); return None;};
        let Some(head) = assets.get(&self.head) else {error!("head not loaded"); return None;};
        let Some(hat) = assets.get(&self.hat) else {error!("hat not loaded"); return None;};
        let Some(legs) = assets.get(&self.legs) else {error!("legs not loaded"); return None;};
        let id = commands.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    margin: scale_ui_rect(body.margin, scale),
                    size: scale_size(body.size, scale),
                    ..default()
                },
                image: body.image.clone().into(),
                ..Default::default()
            }).with_children(|body_root| {
                body_root.spawn(ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        margin: scale_ui_rect(head.margin, scale),
                        size: scale_size(head.size, scale),
                        position: scale_ui_rect(body.nodes.get(&NodeId::Head).cloned().unwrap_or_default(), scale),
                        ..default()
                    },
                    image: head.image.clone().into(),
                    ..Default::default()
                }).with_children(|head_root| {
                    head_root.spawn(ImageBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            justify_content: JustifyContent::Center,
                            margin: scale_ui_rect(hat.margin, scale),
                            size: scale_size(hat.size, scale),
                            position: scale_ui_rect(head.nodes.get(&NodeId::Hat).cloned().unwrap_or_default(), scale),
                            ..default()
                        },
                        image: hat.image.clone().into(),
                        ..Default::default()
                    });
                });
                body_root.spawn(ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        margin: scale_ui_rect(legs.margin, scale),
                        size: scale_size(legs.size, scale),
                        position: scale_ui_rect(body.nodes.get(&NodeId::Legs).cloned().unwrap_or_default(), scale),
                        ..default()
                    },
                    image: legs.image.clone().into(),
                    ..Default::default()
                });
        }).id();
        Some(id)
    }
}

#[derive(serde::Serialize, Deserialize, Hash, PartialEq, Eq)]
enum NodeId {
    Torso,
    Hat,  
    Head,
    Legs,
}

#[derive(TypeUuid)]
#[uuid="70850d07-804c-4f55-98ae-5416aee17ef7"]
pub struct ElfPart {
    image: Handle<Image>,
    size: Size,
    nodes: HashMap<NodeId, UiRect>,
    margin: UiRect,
}

#[derive(Deserialize, serde::Serialize)]
struct ElfPartAsset {
    image: String,
    size: Vec2,
    #[serde(default)]
    margin: [Val; 4],
    #[serde(default)]
    nodes: HashMap<NodeId, [Val; 4]>,
}

struct ElfPartLoader;

impl AssetLoader for ElfPartLoader {
    fn load<'a>(
            &'a self,
            bytes: &'a [u8],
            load_context: &'a mut bevy::asset::LoadContext,
        ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move { load(bytes, load_context) })
    }
    fn extensions(&self) -> &[&str] {
        &["part.ron"]
    }
}

fn load<'a>(bytes: &'a [u8], load_context: &'a mut bevy::asset::LoadContext) -> Result<(), bevy::asset::Error> {
    let mut de = ron::Deserializer::from_bytes(bytes)?;
    let elf_part = ElfPartAsset::deserialize(&mut de)?;
    let image_path = bevy::asset::AssetPath::from(elf_part.image);
    let elf_part = ElfPart {
        margin: UiRect::new(elf_part.margin[0], elf_part.margin[1], elf_part.margin[2], elf_part.margin[3]),
        image: Handle::weak(image_path.clone().into()),
        size: Size::new(Val::Px(elf_part.size.x), Val::Px(elf_part.size.y)),
        nodes: elf_part.nodes.into_iter().map(|(node, data)| (node, UiRect::new(data[0], data[1], data[2], data[3]))).collect(),
    };
    load_context.set_default_asset(LoadedAsset::new(elf_part).with_dependency(image_path));
    Ok(())
}


// #[test]
// fn test() {
//     let asset = ElfPartAsset {
//         image: "test".to_string(),
//         size: Vec2 { x: 0.0, y: 0.0 },
//         nodes: [(NodeId::Hat, [Val::Auto, Val::Undefined, Val::Px(0.), Val::Percent(0.)]),
//         (NodeId::Head, [Val::Auto, Val::Undefined, Val::Px(0.), Val::Percent(0.)]),
//         (NodeId::Legs, [Val::Auto, Val::Undefined, Val::Px(0.), Val::Percent(0.)])].into_iter().collect()
//     };
//     println!("{}", ron::ser::to_string_pretty(&asset, ron::ser::PrettyConfig::default()).unwrap());
// }
