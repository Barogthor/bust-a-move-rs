use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

pub const DEGREES_PER_POS : f32 = 1.4;
pub const SHOOTER_SPRITE_SIZE: f32 = 64.0;
pub const SHOOTER_SPRITE_X_START: f32 = 1.0;
pub const SHOOTER_SPRITE_Y_START: f32 = 1545.0;
pub const SHOOTER_SPRITE_N_ROWS: usize = 4;
pub const SHOOTER_SPRITE_N_COLUMNS: usize = 16;
pub const SHOOTER_MAX_INDICES: usize = SHOOTER_SPRITE_N_ROWS * SHOOTER_SPRITE_N_COLUMNS - 1;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate_shooter)
        // .add_system(animate_sprite)
        .add_system(bevy::window::close_on_esc)
        .run()
}

#[derive(Component)]
pub struct Shooter;

#[derive(Component, Debug)]
pub struct ShooterAngle(f32);

#[derive(Component, Debug)]
pub struct ShooterSpriteIndex(usize);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}


pub fn setup( mut commands: Commands,
              asset_server: Res<AssetServer>,
              mut texture_atlases: ResMut<Assets<TextureAtlas>>)
{
    let tex_handle = asset_server.load("textures/bust-a-move.png");
    let tex_atlas =
        TextureAtlas::from_grid(tex_handle,
                                Vec2::new(SHOOTER_SPRITE_SIZE, SHOOTER_SPRITE_SIZE),
                                SHOOTER_SPRITE_N_COLUMNS, SHOOTER_SPRITE_N_ROWS,
                                Some(Vec2::ONE), Some(Vec2::new(SHOOTER_SPRITE_X_START, SHOOTER_SPRITE_Y_START)));

    let tex_atlas_handle = texture_atlases.add(tex_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 63 };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: tex_atlas_handle,
            sprite: TextureAtlasSprite {
                index: animation_indices.first,
                flip_x: true,
                ..default()
            },
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        Shooter{},
        ShooterAngle(0.0),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

pub fn rotate_shooter(
    keyboard: Res<Input<KeyCode>>,
    mut shooter_query: Query<(&mut ShooterAngle, &mut TextureAtlasSprite), With<Shooter>>
) {
    if let Ok((mut angle, mut sprite)) = shooter_query.get_single_mut() {
        if sprite.index == 0 && (keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Q) ) {
            sprite.flip_x = !sprite.flip_x;
            sprite.index = 1;
            angle.0 = 0.0;
        }
        else if sprite.flip_x {
            if keyboard.pressed(KeyCode::Q) && sprite.index + 1 <= SHOOTER_MAX_INDICES
            {
                sprite.index +=1;
                angle.0 = DEGREES_PER_POS * sprite.index as f32;
            }
            else if keyboard.pressed(KeyCode::D)
            {
                sprite.index -= 1;
                angle.0 = DEGREES_PER_POS * sprite.index as f32;
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
        println!("{:?}", angle);
    }
}