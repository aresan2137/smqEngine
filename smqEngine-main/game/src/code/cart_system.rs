use bevy_ecs::prelude::*;
use glam::*;
use winit::keyboard::KeyCode;
use crate::*;

#[derive(Component)]
struct Dropoff {
    position_min: Vec3,
    position_max: Vec3,
    is_fill: bool,
    uses: f32
}

#[derive(Component)]
struct Cart {
    holded: bool,
    fillage: f32
}

impl Dropoff {
    pub fn from_position_size(position: Vec3, size: Vec3, is_fill: bool, uses: f32) -> Self {
        return Self { 
            position_min: position - size/2.0, 
            position_max: position + size/2.0, 
            is_fill, 
            uses
        };
    }
}

pub fn cart_system_start(world: &mut World, schedule: &mut Schedule) {
    world.spawn((Dropoff::from_position_size(Vec3::new(0.0, 1.0, 0.0) ,Vec3::new(3.0, 3.0, 3.0), false, 0.0),
        Position3D {
            position: Vec3::new(0.0, 1.0, 0.0),
            rotation: Quat::IDENTITY,
            size: Vec3::new(3.0, 3.0, 3.0),
            mesh: MeshID::Cube
        }
    ));

    world.spawn((Dropoff::from_position_size(Vec3::new(0.0, 1.0, 15.0) ,Vec3::new(3.0, 3.0, 3.0), true, 10.0),
        Position3D {
            position: Vec3::new(0.0, 1.0, 15.0),
            rotation: Quat::IDENTITY,
            size: Vec3::new(3.0, 3.0, 3.0),
            mesh: MeshID::Cube
        }
    ));

    world.spawn((Cart {
        holded: false,
        fillage: 0.0
    }, Position3D {
        position: Vec3::new(0.0, 0.0, -5.0),
        rotation: Quat::IDENTITY,
        size: Vec3::ONE,
        mesh: MeshID::Cart
    }
    ));
    
    schedule.add_systems(dropoff_system);
    schedule.add_systems(cart_movment_system);
}

fn dropoff_system(mut dropoff_query: Query<&mut Dropoff>, mut cart_query: Query<(&Position3D, &mut Cart)>) {
    for mut dropoff in dropoff_query.iter_mut() {
        for (position, mut cart) in cart_query.iter_mut() {
            if dropoff.position_min.x < position.position.x &&
                dropoff.position_min.y < position.position.y &&
                dropoff.position_min.z < position.position.z &&
                dropoff.position_max.x > position.position.x &&
                dropoff.position_max.y > position.position.y &&
                dropoff.position_max.z > position.position.z {
                if dropoff.is_fill {
                    cart.fillage += dropoff.uses;
                    dropoff.uses = 0.0;
                } else {
                    dropoff.uses += cart.fillage;
                    cart.fillage = 0.0;
                }
            }
        }     
        
        if !dropoff.is_fill {
            println!("fuli: {}", dropoff.uses);
        } 
    }
}

fn cart_movment_system(player_query: Query<&free_cam::FreeCamera>, mut cart_query: Query<(&mut Position3D, &mut Cart)>, input: Res<InputState>) {
    let Ok(player) = player_query.single() else { return; };

    if input.pressed(KeyCode::KeyE) {
        let mut dropped_something = false;

        for (_, mut cart) in cart_query.iter_mut() {
            if cart.holded {
                cart.holded = false;
                dropped_something = true;
            }
        }

        if !dropped_something {
            for (position, mut cart) in cart_query.iter_mut() {
                if (position.position - player.position).length() < 3.0 {
                    cart.holded = true;
                    break;
                }
            }
        }
    }

    for (mut position, cart) in cart_query.iter_mut() {
        if cart.holded {
            let q_yaw = Quat::from_axis_angle(Vec3::Y, player.yaw.to_radians());
            let rotation = q_yaw.normalize();

            let forward = rotation * Vec3::new(0.0, 0.0, -1.0);
            let up = Vec3::new(0.0, 1.0, 0.0);

            let offset = forward * 1.0 + up * -1.2;

            position.position = player.position + offset;
            position.rotation = rotation;
        }
    }
}