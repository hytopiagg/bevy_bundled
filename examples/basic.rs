use bevy_bundled::*;

#[derive(Bundled)]
struct EntityData {
    pos: Vec3,
    vel: Vec3,
    health: f32,
}
