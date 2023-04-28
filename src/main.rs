use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

pub const DEGREES_PER_POS : f32 = 1.4;
pub const SHOOTER_SPRITE_SIZE: f32 = 64.0;
pub const SHOOTER_SPRITE_START: Vec2 = Vec2{x: 1.0, y: 1545.0};
pub const SHOOTER_SPRITE_N_ROWS: usize = 4;
pub const SHOOTER_SPRITE_N_COLUMNS: usize = 16;
pub const SHOOTER_MAX_INDICES: usize = SHOOTER_SPRITE_N_ROWS * SHOOTER_SPRITE_N_COLUMNS - 1;
pub const BLUE_BALL_START : Vec2 = Vec2{x: 1.0, y: 1854.0};
pub const RED_BALL_START : Vec2 = Vec2{x: 1.0, y: 1887.0};
pub const PURPLE_BALL_START : Vec2 = Vec2{x: 1.0, y: 1920.0};
pub const GREY_BALL_START_X : Vec2 = Vec2{x: 1.0, y: 1953.0};
pub const YELLOW_BALL_START : Vec2 = Vec2{x: 555.0, y: 1854.0};
pub const GREEN_BALL_START : Vec2 = Vec2{x: 555.0, y:1887.0};
pub const ORANGE_BALL_START : Vec2 = Vec2{x:555.0, y: 1920.0};
pub const SILVER_BALL_START : Vec2 = Vec2{y: 1953.0, x: 555.0};

pub enum BubbleColor {
    Blue, Red, Purple, Grey, Yellow, Green, Silver
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_event::<ShootedBallEvent>()
        .add_startup_system(setup)
        .add_system(rotate_shooter)
        .add_system(shoot_ball)
        .add_system(move_shooted_bubble)
        .add_system(set_shooter_ball)
        .add_system(bubble_collide_wall)
        // .add_system(animate_sprite)
        .add_system(bevy::window::close_on_esc)
        .run()
}

#[derive(Resource, Deref, DerefMut)]
pub struct BustAMoveTexture(Handle<Image>);

#[derive(Component)]
pub struct Shooter;

#[derive(Component, Deref, DerefMut)]
pub struct Bubble(BubbleColor);

#[derive(Component)]
pub struct ShooterBubble;

#[derive(Component)]
pub struct ShootedBubble;

#[derive(Component, Deref, DerefMut)]
pub struct Direction(Vec3);

#[derive(Component, Debug, Deref, DerefMut)]
pub struct ShooterAngle(f32);

#[derive(Component, Debug, Deref, DerefMut)]
pub struct ShooterSpriteIndex(usize);

pub struct ShootedBallEvent {
    pub shooter: Entity,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>)
{
    let tex_handle = asset_server.load("textures/bust-a-move.png");
    let ref_tex = tex_handle.clone_weak();
    commands.insert_resource(BustAMoveTexture(tex_handle.clone()));
    // let config_handle = asset_server.load("config/test.ron");
    // commands.insert_resource(BustAMoveTexture(tex_handle));
    let tex_atlas =
        TextureAtlas::from_grid(ref_tex,
                                Vec2::new(SHOOTER_SPRITE_SIZE, SHOOTER_SPRITE_SIZE),
                                SHOOTER_SPRITE_N_COLUMNS, SHOOTER_SPRITE_N_ROWS,
                                Some(Vec2::ONE), Some(SHOOTER_SPRITE_START));

    let tex_atlas_handle = texture_atlases.add(tex_atlas);
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
           texture_atlas: tex_atlas_handle,
           sprite: TextureAtlasSprite {
               index: 0,
               flip_x: true,
               ..default()
           },
           transform: Transform::from_scale(Vec3::splat(1.0)),
           ..default()
        },
        Shooter,
        ShooterAngle(0.0),
    ))
        .with_children(|parent| {
            parent.spawn(build_shooter_bubble(tex_handle.clone_weak(), 0.0));
        });
    build_wall(&mut commands);
}

pub fn shoot_ball(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    shooter_query: Query<(Entity, &ShooterAngle), With<Shooter>>,
    shooter_bubble_query: Query<Entity, With<ShooterBubble>>,
    mut shoot_event_writer: EventWriter<ShootedBallEvent>
) {
    if keyboard.just_pressed(KeyCode::Z) {
        if let Ok((shooter, angle)) = shooter_query.get_single() {
            if let Ok(shoot_bubble) = shooter_bubble_query.get_single() {
                let mut bubble_entity = commands.entity(shoot_bubble);
                bubble_entity.remove::<ShooterBubble>();
                let x = (90.0 - angle.0).to_radians().cos();
                let y = (90.0 - angle.0).to_radians().sin();
                bubble_entity.insert(ShootedBubble);
                bubble_entity.insert(Direction(Vec3::new(x, y, 0.0)));
                shoot_event_writer.send(ShootedBallEvent{shooter});
            }
        }
    }
}

pub fn move_shooted_bubble(
    mut shooted_bubble_query: Query<(&mut Transform, &Direction), With<ShootedBubble>>,
    time: Res<Time>,
) {
    for (mut transform, direction) in shooted_bubble_query.iter_mut() {
        let direction = direction.0.clone();
        transform.translation += direction * time.delta_seconds() * 200.0;
    }
}

pub fn rotate_shooter(
    keyboard: Res<Input<KeyCode>>,
    mut shooter_query: Query<(&mut ShooterAngle, &mut TextureAtlasSprite), With<Shooter>>,
    mut shooter_bubble_query: Query<&mut Transform, With<ShooterBubble>>
) {
    if let Ok((mut angle, mut sprite)) = shooter_query.get_single_mut() {

        if sprite.index == 0 && (keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Q) ) {
            sprite.flip_x = !sprite.flip_x;
            sprite.index = 1;
            angle.0 = DEGREES_PER_POS;
        }
        else if sprite.flip_x {
            if keyboard.pressed(KeyCode::Q) && sprite.index + 1 <= SHOOTER_MAX_INDICES
            {
                sprite.index +=1;
                angle.0 = -DEGREES_PER_POS * sprite.index as f32;
            }
            else if keyboard.pressed(KeyCode::D)
            {
                sprite.index -= 1;
                angle.0 = -DEGREES_PER_POS * sprite.index as f32;
            }
        }
        else if !sprite.flip_x {
            if keyboard.pressed(KeyCode::D) && sprite.index + 1 <= SHOOTER_MAX_INDICES
            {
                sprite.index +=1;
                angle.0 = DEGREES_PER_POS * sprite.index as f32;
            }
            else if keyboard.pressed(KeyCode::Q)
            {
                sprite.index -= 1;
                angle.0 = DEGREES_PER_POS * sprite.index as f32;
            }
        }
        // println!("{:?}", angle);
        for mut shoot_bubble in shooter_bubble_query.iter_mut() {
            let transform = &mut shoot_bubble.translation;
            transform.x = 26.0 * (90.0 - angle.0).to_radians().cos();
            transform.y = 26.0 * (90.0 - angle.0).to_radians().sin();
        }
    }
}

fn set_shooter_ball(
    mut commands: Commands,
    mut shoot_event_reader: EventReader<ShootedBallEvent>,
    game_texture: Res<BustAMoveTexture>,
    shooter_query: Query<(Entity, &ShooterAngle),With<Shooter>>
) {
    for evt in shoot_event_reader.iter() {
        if let Ok((shooter,angle)) = shooter_query.get(evt.shooter) {
            let shooter_bubble = commands.spawn(build_shooter_bubble(game_texture.0.clone_weak(), angle.0)).id();
            commands.entity(shooter).add_child(shooter_bubble);
        }
    }
}

fn build_shooter_bubble(game_texture: Handle<Image>, angle: f32) -> (SpriteBundle, ShooterBubble, Bubble) {
    (
        SpriteBundle {
            texture: game_texture,
            sprite: Sprite {
                rect: Some(Rect::new(RED_BALL_START.x, RED_BALL_START.y, RED_BALL_START.x + 16.0, RED_BALL_START.y + 16.0)),
                ..default()
            },
            transform: Transform::from_xyz(26.0*(90.0 - angle).cos(), 26.0*(90.0-angle).sin(), 1.0),
            ..default()
        },
        ShooterBubble,
        Bubble(BubbleColor::Red),
    )
}

pub fn build_wall(commands: &mut Commands) {
    let wall_width = 32.0;
    let wall_height = 450.0;
    let half_wall_width = wall_width / 2.0;
    let half_wall_height = wall_height / 2.0;
    let origin = Vec2::new(-150.0, 50.0);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GOLD,
                custom_size: Some(Vec2::new(wall_width, wall_height)),
                ..default()
            },
            transform: Transform::from_xyz(origin.x, origin.y, 0.0),
            ..default()
        },
        Wall{width: wall_width, height: wall_height},
        Normal(Vec3::X)
        ));
    spawn_debug_point(commands, Vec2::new(origin.x - half_wall_width, origin.y - half_wall_height), Color::RED);
    spawn_debug_point(commands, Vec2::new(origin.x + half_wall_width, origin.y - half_wall_height), Color::CRIMSON);
    spawn_debug_point(commands, Vec2::new(origin.x - half_wall_width, origin.y + half_wall_height), Color::ORANGE);
    spawn_debug_point(commands, Vec2::new(origin.x + half_wall_width, origin.y + half_wall_height), Color::MAROON);
    let origin = Vec2::new(150.0, 50.0);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GOLD,
                custom_size: Some(Vec2::new(wall_width, wall_height)),
                ..default()
            },
            transform: Transform::from_xyz(150.0, 50.0, 0.0),
            ..default()
        },
        Wall{width: wall_width, height: wall_height},
        Normal(Vec3::NEG_X)
        ));
    spawn_debug_point(commands, Vec2::new(origin.x - half_wall_width, origin.y - half_wall_height), Color::RED);
    spawn_debug_point(commands, Vec2::new(origin.x + half_wall_width, origin.y - half_wall_height), Color::CRIMSON);
    spawn_debug_point(commands, Vec2::new(origin.x - half_wall_width, origin.y + half_wall_height), Color::ORANGE);
    spawn_debug_point(commands, Vec2::new(origin.x + half_wall_width, origin.y + half_wall_height), Color::MAROON);

    spawn_debug_point(commands, Vec2::new(50.0, 0.0), Color::RED);
    spawn_debug_point(commands, Vec2::new(100.0, 50.0), Color::BLUE);

}

fn spawn_debug_point(commands: &mut Commands, origin: Vec2, color: Color) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(2.0, 2.0)),
            ..default()
        },
        transform: Transform::from_xyz(origin.x, origin.y, 1.0),
        ..default()
    });
}

#[derive(Component, DerefMut, Deref)]
pub struct Normal(Vec3);

#[derive(Component)]
pub struct Wall{
    width: f32,
    height: f32
}

pub fn bubble_collide_wall(
    mut shooted_bubble_query: Query<(&Transform, &mut Direction), With<ShootedBubble>>,
    wall_query: Query<(&Normal, &Transform, &Wall), Without<Direction>>,
) {
    for (wall_normal, wall_transform, wall) in wall_query.iter() {
        for (bubble_transform, mut bubble_dir) in shooted_bubble_query.iter_mut() {
            let wall_origin = Vec2::new(wall_transform.translation.x - wall.width/2.0, wall_transform.translation.y + wall.height/2.0);
            let bubble_collider = Circle::new(bubble_transform.translation.xy(),8.0);
            let a = Vec2::new(wall_origin.x, wall_origin.y);
            let b = Vec2::new(wall_origin.x + wall.width, wall_origin.y);
            let c = Vec2::new(wall_origin.x + wall.width, wall_origin.y - wall.height);
            let d = Vec2::new(wall_origin.x, wall_origin.y - wall.height);
            let segments = vec![Segment(a,b),Segment(b,c), Segment(c, d), Segment(a,d)];
            let r = Rect::new(wall_origin.x, wall_origin.y, c.x, c.y);
            if r.contains(bubble_transform.translation.xy())
               || segments.iter().any(|segment| bubble_collider.circle_intersect_segment(segment))
            {
                bubble_dir.x *= -1.0;
            }
        }
    }
}

#[derive(Clone)]
pub struct Segment(Vec2, Vec2);

pub struct Circle{
    center: Vec2, radius: f32
}

impl Circle {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self {
            center, radius
        }
    }

    pub fn circle_intersect_segment(&self, segment: &Segment) -> bool {
        let ac = self.center - segment.0;
        let ab = segment.1 - segment.0;
        let (ad, k) = vector_projection(ac, ab);
        if k < 0.0 || k > 1.0 {
            return false;
        }
        let d = segment.0 + ad;
        let cd = self.center - d;
        self.radius * self.radius >= cd.length_squared()
    }
}

fn vector_projection(v1: Vec2, v2: Vec2) -> (Vec2, f32) {
    let k = v1.dot(v2) / v2.dot(v2);
    (k * v2, k)
}