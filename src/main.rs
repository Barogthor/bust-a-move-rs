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
        .add_startup_system(setup)
        .add_system(rotate_shooter)
        .add_system(shoot_ball)
        .add_system(move_shooted_bubble)
        // .add_system(animate_sprite)
        .add_system(bevy::window::close_on_esc)
        .run()
}

#[derive(Resource, Deref, DerefMut)]
pub struct BustAMoveTexture(Handle<Image>);

#[derive(Component)]
pub struct Shooter;

#[derive(Component)]
pub struct Bubble(BubbleColor);

#[derive(Component)]
pub struct ShooterBubble;

#[derive(Component)]
pub struct ShootedBubble;

#[derive(Component, Deref, DerefMut)]
pub struct Direction(Vec3);

#[derive(Component, Debug)]
pub struct ShooterAngle(f32);

#[derive(Component, Debug)]
pub struct ShooterSpriteIndex(usize);

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
        Shooter{},
        ShooterAngle(0.0),
    ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: tex_handle.clone_weak(),
                    sprite: Sprite {
                        rect: Some(Rect::new(RED_BALL_START.x, RED_BALL_START.y, RED_BALL_START.x + 16.0, RED_BALL_START.y + 16.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 24.0, 1.0),
                    ..default()
                },
                ShooterBubble{},
                Bubble(BubbleColor::Red),
            ));
        });
}

pub fn shoot_ball(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut shooter_query: Query<(&ShooterAngle, &TextureAtlasSprite), With<Shooter>>,
    mut shooter_bubble_query: Query<Entity, With<ShooterBubble>>
) {
    if keyboard.pressed(KeyCode::Z) {
        if let Ok((angle, sprite)) = shooter_query.get_single() {
            if let Ok(shoot_bubble) = shooter_bubble_query.get_single() {
                let mut bubble_entity = commands.entity(shoot_bubble);
                bubble_entity.remove::<ShooterBubble>();
                let sign = if sprite.flip_x { 1.0 } else { -1.0 };
                let x = (90.0 + angle.0 * sign).to_radians().sin();
                let y = (90.0 + angle.0 * sign).to_radians().cos();
                bubble_entity.insert(ShootedBubble{});
                bubble_entity.insert(Direction(Vec3::new(x, y, 0.0)));
            }
        }
    }
}

pub fn move_shooted_bubble(
    mut shooted_bubble_query: Query<(&mut Transform, &Direction), With<ShootedBubble>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, direction)) = shooted_bubble_query.get_single_mut() {
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
        for mut shoot_bubble in shooter_bubble_query.iter_mut() {
            let sign = if sprite.flip_x { 1.0 } else { -1.0 };
            let transform = &mut shoot_bubble.translation;
            transform.y = 26.0 * (90.0 + angle.0 * sign).to_radians().sin();
            transform.x = 26.0 * (90.0 + angle.0 * sign).to_radians().cos();
        }
    }
}