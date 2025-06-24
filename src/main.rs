use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Component)]
struct SelectedObject;

#[derive(Component)]
struct ObjectRotation {
    rotation_speed: f32,
}

#[derive(Resource)]
struct ObjectType {
    current: String,
}

#[derive(Resource)]
struct MouseControl {
    is_dragging: bool,
    last_position: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(ObjectType {
            current: "Cube".to_string(),
        })
        .insert_resource(MouseControl {
            is_dragging: false,
            last_position: Vec2::ZERO,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (
            mouse_control_system,
            keyboard_control_system,
            ui_system,
            object_selection_system,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Single directional light for clean lighting
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 2500.0, // Reduced intensity
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
    ));

    // Moderate ambient light for material visibility
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.25, // Increased for better material visibility
    });

    // Ground plane for shadows
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Plane3d::default().mesh().size(10.0, 10.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.8, 0.8), // Light gray
            metallic: 0.0,
            perceptual_roughness: 0.8, // Rough surface
            reflectance: 0.2,
            ..default()
        })),
        Transform::from_xyz(0.0, -1.0, 0.0), // Position below objects
    ));

    // Initial object (cube)
    spawn_object(&mut commands, &mut meshes, &mut materials, "Cube");
}

fn spawn_object(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    object_type: &str,
) {
    let mesh = match object_type {
        "Cube" => meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
        "Sphere" => meshes.add(Mesh::from(Sphere::new(0.5))),
        "Cylinder" => meshes.add(Mesh::from(Cylinder::new(0.5, 1.0))),
        "Torus" => meshes.add(Mesh::from(Torus::new(0.3, 0.8))),
        "Cone" => meshes.add(Mesh::from(Cone::new(0.5, 1.0))),
        _ => meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
    };

    // Create materials with better light reflection properties
    let material = match object_type {
        "Cube" => materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2), // Red
            metallic: 0.3,
            perceptual_roughness: 0.2, // Smoother for better reflections
            reflectance: 0.5,
            ..default()
        }),
        "Sphere" => materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.2), // Green
            metallic: 0.4,
            perceptual_roughness: 0.1, // Very smooth
            reflectance: 0.6,
            ..default()
        }),
        "Cylinder" => materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.8), // Blue
            metallic: 0.2,
            perceptual_roughness: 0.3,
            reflectance: 0.4,
            ..default()
        }),
        "Torus" => materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.6, 0.2), // Orange
            metallic: 0.1,
            perceptual_roughness: 0.4,
            reflectance: 0.3,
            ..default()
        }),
        "Cone" => materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.8), // Magenta
            metallic: 0.5,
            perceptual_roughness: 0.15, // Smooth and reflective
            reflectance: 0.7,
            ..default()
        }),
        _ => materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.7, 0.6),
            metallic: 0.2,
            perceptual_roughness: 0.5,
            reflectance: 0.4,
            ..default()
        }),
    };

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        SelectedObject,
        ObjectRotation {
            rotation_speed: 1.0,
        },
    ));
}

fn mouse_control_system(
    mut mouse_control: ResMut<MouseControl>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut object_query: Query<&mut Transform, With<SelectedObject>>,
    mut contexts: EguiContexts,
) {
    // Check if mouse is over UI
    let ctx = contexts.ctx_mut();
    let is_mouse_over_ui = ctx.is_pointer_over_area();

    // Handle mouse dragging only if not over UI
    if mouse_button_input.just_pressed(MouseButton::Left) && !is_mouse_over_ui {
        mouse_control.is_dragging = true;
    }
    if mouse_button_input.just_released(MouseButton::Left) {
        mouse_control.is_dragging = false;
    }

    if mouse_control.is_dragging && !is_mouse_over_ui {
        for event in mouse_motion_events.read() {
            if let Ok(mut transform) = object_query.get_single_mut() {
                transform.rotate_y(event.delta.x * 0.01);
                transform.rotate_x(-event.delta.y * 0.01);
            }
        }
    }

    // Handle mouse wheel for scaling only if not over UI
    if !is_mouse_over_ui {
        for event in mouse_wheel_events.read() {
            if let Ok(mut transform) = object_query.get_single_mut() {
                let scale_factor = 1.0 + event.y * 0.1;
                transform.scale *= scale_factor;
            }
        }
    }
}

fn keyboard_control_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut object_query: Query<&mut Transform, With<SelectedObject>>,
) {
    if let Ok(mut transform) = object_query.get_single_mut() {
        let mut movement = Vec3::ZERO;
        
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            movement.x -= 0.02;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            movement.x += 0.02;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            movement.y += 0.02;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            movement.y -= 0.02;
        }
        if keyboard_input.pressed(KeyCode::KeyQ) {
            movement.z -= 0.02;
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            movement.z += 0.02;
        }

        transform.translation += movement;
    }
}

fn ui_system(
    mut contexts: EguiContexts,
    mut object_type: ResMut<ObjectType>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    object_query: Query<Entity, With<SelectedObject>>,
    mut object_transform_query: Query<&mut Transform, With<SelectedObject>>,
) {
    egui::Window::new("3D Object Viewer")
        .default_width(250.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Object Selection");
            
            let objects = ["Cube", "Sphere", "Cylinder", "Torus", "Cone"];
            let mut selected_index = objects.iter().position(|&x| x == object_type.current).unwrap_or(0);
            
            egui::ComboBox::from_label("Select Object")
                .selected_text(&object_type.current)
                .show_ui(ui, |ui| {
                    for (i, &obj) in objects.iter().enumerate() {
                        ui.selectable_value(&mut selected_index, i, obj);
                    }
                });

            if object_type.current != objects[selected_index] {
                object_type.current = objects[selected_index].to_string();
                
                // Remove old object
                for entity in object_query.iter() {
                    commands.entity(entity).despawn();
                }
                
                // Spawn new object
                spawn_object(&mut commands, &mut meshes, &mut materials, &object_type.current);
            }

            ui.separator();
            ui.heading("Movement Controls");
            
            if let Ok(mut transform) = object_transform_query.get_single_mut() {
                ui.horizontal(|ui| {
                    if ui.button("← Left").clicked() {
                        transform.translation.x -= 0.1;
                    }
                    if ui.button("→ Right").clicked() {
                        transform.translation.x += 0.1;
                    }
                });
                
                ui.horizontal(|ui| {
                    if ui.button("↑ Up").clicked() {
                        transform.translation.y += 0.1;
                    }
                    if ui.button("↓ Down").clicked() {
                        transform.translation.y -= 0.1;
                    }
                });
                
                ui.horizontal(|ui| {
                    if ui.button("Forward").clicked() {
                        transform.translation.z -= 0.1;
                    }
                    if ui.button("Back").clicked() {
                        transform.translation.z += 0.1;
                    }
                });

                ui.separator();
                ui.heading("Rotation Controls");
                
                ui.horizontal(|ui| {
                    if ui.button("⟲ Rotate Left").clicked() {
                        transform.rotate_y(0.1);
                    }
                    if ui.button("⟳ Rotate Right").clicked() {
                        transform.rotate_y(-0.1);
                    }
                });
                
                ui.horizontal(|ui| {
                    if ui.button("↻ Rotate X+").clicked() {
                        transform.rotate_x(0.1);
                    }
                    if ui.button("↺ Rotate X-").clicked() {
                        transform.rotate_x(-0.1);
                    }
                });

                ui.separator();
                ui.heading("Scale Controls");
                
                ui.horizontal(|ui| {
                    if ui.button("+ Scale Up").clicked() {
                        transform.scale *= 1.1;
                    }
                    if ui.button("- Scale Down").clicked() {
                        transform.scale *= 0.9;
                    }
                });

                if ui.button("Reset Position").clicked() {
                    transform.translation = Vec3::ZERO;
                    transform.rotation = Quat::IDENTITY;
                    transform.scale = Vec3::ONE;
                }
            }

            ui.separator();
            ui.heading("Controls Help");
            ui.label("Mouse: Click and drag to rotate");
            ui.label("Scroll: Scale object");
            ui.label("WASD/Arrows: Move object");
            ui.label("Q/E: Move forward/back");
        });
}

fn object_selection_system() {
    // This system can be expanded for more complex object selection logic
    // Currently all objects are selected by default
}
