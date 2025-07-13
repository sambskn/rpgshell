use bevy::{
    color::palettes::css::{BLACK, GHOST_WHITE},
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};

use crate::screens::Screen;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TextBox {
    pub text_strings: Vec<String>,
    pub current_text_index: usize,
    pub last_text_index_displayed: Option<usize>,
    pub time_since_last_text_displayed: f32,
    pub should_spawn_next_line: bool,
    pub indicator_visible: bool,
}

impl TextBox {
    pub fn new(text_strings: Vec<String>) -> Self {
        Self {
            text_strings,
            current_text_index: 0,
            last_text_index_displayed: None,
            time_since_last_text_displayed: 0.0,
            should_spawn_next_line: false,
            indicator_visible: false,
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TextLine;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TextBoxIndicator;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TextBoxText {
    pub is_visible: bool,
    pub visible_color: Color,
    pub appearance_time_s: f32,
    pub spawn_time_s: f32,
}

impl TextBoxText {
    pub fn new(appearance_time_s: f32, spawn_time_s: f32, visible_color: Color) -> Self {
        Self {
            is_visible: false,
            visible_color,
            appearance_time_s,
            spawn_time_s,
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TextBoxMesh {
    pub is_visible: bool,
    pub visible_alpha: f32,
    pub appearance_time_s: f32,
    pub spawn_time_s: f32,
    pub with_cutout: bool,
}

impl TextBoxMesh {
    pub fn new(
        appearance_time_s: f32,
        spawn_time_s: f32,
        visible_alpha: f32,
        with_cutout: bool,
    ) -> Self {
        Self {
            is_visible: false,
            visible_alpha,
            appearance_time_s,
            spawn_time_s,
            with_cutout,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TextBox>();
    app.register_type::<TextBoxMesh>();
    app.register_type::<TextBoxText>();
    app.register_type::<TextBoxIndicator>();

    app.add_systems(Update, animate_text_box_mesh_intro);
    app.add_systems(Update, animate_text_box_text_intro);
    app.add_systems(Update, animate_text_box_indicator);
    app.add_systems(Update, spawn_text_lines);
}

const TEXT_TRANSITION_TIME: f32 = 0.75;
fn spawn_text_lines(
    mut commands: Commands,
    mut textbox_query: Query<&mut TextBox>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    let mut textbox_check = textbox_query.single_mut();
    if textbox_check.is_err() {
        return;
    }
    let textbox = textbox_check.as_mut().unwrap();

    if textbox.last_text_index_displayed.is_none()
        && textbox.time_since_last_text_displayed >= TEXT_TRANSITION_TIME
    {
        // spawn initial text line
        commands.spawn(text_line(
            textbox.text_strings[0].clone(),
            time.elapsed_secs(),
        ));
        textbox.last_text_index_displayed = Some(0);
        textbox.current_text_index = 0;
        textbox.time_since_last_text_displayed = 0.;
    } else if textbox.last_text_index_displayed.is_some()
        && textbox.time_since_last_text_displayed >= TEXT_TRANSITION_TIME
        && !textbox.indicator_visible
    {
        println!("spawn that weidwejkd3we");
        // spawn indicator
        commands.spawn(text_box_next_indicator(
            meshes,
            materials,
            time.elapsed_secs(),
        ));
        textbox.indicator_visible = true;
    } else if textbox.time_since_last_text_displayed < TEXT_TRANSITION_TIME {
        textbox.time_since_last_text_displayed += time.delta_secs();
    }
}

fn animate_text_box_mesh_intro(
    mut mesh2d_query: Query<(&mut Mesh2d, &mut TextBoxMesh)>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
) {
    for (mut mesh2d, mut mesh_info) in mesh2d_query.iter_mut() {
        if mesh_info.is_visible {
            // skip if we're already visible
            continue;
        }
        let alpha = if time.elapsed_secs() - mesh_info.spawn_time_s > mesh_info.appearance_time_s {
            mesh_info.is_visible = true;
            mesh_info.visible_alpha
        } else {
            ((time.elapsed_secs() - mesh_info.spawn_time_s) / mesh_info.appearance_time_s)
                * mesh_info.visible_alpha
        };
        let new_mesh = get_text_box_mesh(mesh_info.with_cutout, alpha);
        let new_mesh_handle = meshes.add(new_mesh);
        mesh2d.0 = new_mesh_handle;
    }
}

const TRIANGLE_WOBBLE_SPEED: f32 = 4.;
const TRIANGLE_WOBBLE_OFFSET: f32 = 3.;
fn animate_text_box_indicator(
    mut mesh2d_query: Query<(&mut Mesh2d, &mut Transform), With<TextBoxIndicator>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
) {
    for (mut mesh2d, mut transform) in mesh2d_query.iter_mut() {
        let t = (time.elapsed_secs() * TRIANGLE_WOBBLE_SPEED).sin();
        let new_mesh =
            get_colored_triangle_mesh(TRIANGLE_INDICATOR_HEIGHT, TRIANGLE_INDICATOR_WIDTH, t);
        let new_mesh_handle = meshes.add(new_mesh);
        mesh2d.0 = new_mesh_handle;
        transform.translation.y = get_indicator_y(time.elapsed_secs());
    }
}

fn get_indicator_y(elapsed_secs: f32) -> f32 {
    TEXTBOX_OFFSET_FROM_CENTER_Y - (TEXTBOX_HEIGHT / 2.0)
        + (elapsed_secs * TRIANGLE_WOBBLE_SPEED).sin() * TRIANGLE_WOBBLE_OFFSET
}

fn animate_text_box_text_intro(
    mut text_query: Query<(&mut TextColor, &mut TextBoxText)>,
    time: Res<Time>,
) {
    for (mut text_color, mut text_info) in text_query.iter_mut() {
        if text_info.is_visible {
            // skip if we're already visible
            continue;
        }
        let alpha = if time.elapsed_secs() - text_info.spawn_time_s > text_info.appearance_time_s {
            text_info.is_visible = true;
            1.0
        } else {
            (time.elapsed_secs() - text_info.spawn_time_s) / text_info.appearance_time_s
        };
        let mut color = text_info.visible_color;
        color.set_alpha(alpha);
        text_color.0 = color;
    }
}

pub const TEXTBOX_OFFSET_FROM_CENTER_Y: f32 = -150.;
pub const TEXTBOX_BG_ALPHA: f32 = 1.0;
pub const TEXTBOX_BG_SHADOW_ALPHA: f32 = 0.2;
pub const TEXTBOX_BG_SHADOW_OFFSET: f32 = 8.0;

pub const TEXTBOX_WIDTH: f32 = 700.;
pub const TEXTBOX_HEIGHT: f32 = 200.;

pub const TRIANGLE_INDICATOR_WIDTH: f32 = 34.;
pub const TRIANGLE_INDICATOR_HEIGHT: f32 = 28.;

pub const BOX_BG_Z: f32 = 1.;
pub const BOX_BG_SHADOW_Z: f32 = 0.9;
pub const TEXT_SHADOW_Z: f32 = 1.1;
pub const TEXT_Z: f32 = 1.2;
pub const TRIANGLE_MESH_Z: f32 = 1.3;

pub const TEXT_SHADOW_OFFSET: f32 = 2.0;
pub const TEXT_FONT_SIZE: f32 = 25.0;

pub const LINE_THICKNESS: f32 = 10.;

pub const TEXTBOX_FADE_IN_TIME: f32 = 0.125;

pub fn text_box(
    text_strings: Vec<String>,
    spawn_time: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    let main_bg_mesh = get_text_box_mesh(true, 0.0);
    let bg_shadow_mesh = get_text_box_mesh(false, 0.0);

    let main_bg_mesh_handle = meshes.add(main_bg_mesh);
    let bg_shadow_mesh_handle = meshes.add(bg_shadow_mesh);
    (
        TextBox::new(text_strings),
        children![
            (
                Mesh2d(bg_shadow_mesh_handle),
                MeshMaterial2d(materials.add(ColorMaterial::default())),
                Transform::from_translation(Vec3::new(
                    TEXTBOX_BG_SHADOW_OFFSET,
                    TEXTBOX_OFFSET_FROM_CENTER_Y - TEXTBOX_BG_SHADOW_OFFSET,
                    BOX_BG_SHADOW_Z
                ))
                .with_scale(Vec3::splat(1.)),
                TextBoxMesh::new(
                    TEXTBOX_FADE_IN_TIME,
                    spawn_time,
                    TEXTBOX_BG_SHADOW_ALPHA,
                    false
                ),
            ),
            (
                Mesh2d(main_bg_mesh_handle),
                MeshMaterial2d(materials.add(ColorMaterial::default())),
                Transform::from_translation(Vec3::new(0., TEXTBOX_OFFSET_FROM_CENTER_Y, BOX_BG_Z))
                    .with_scale(Vec3::splat(1.)),
                TextBoxMesh::new(TEXTBOX_FADE_IN_TIME, spawn_time, TEXTBOX_BG_ALPHA, true),
            ),
        ],
    )
}

fn text_line(text: String, spawn_time: f32) -> impl Bundle {
    (
        StateScoped(Screen::Gameplay),
        TextLine,
        Transform::default(),
        children![
            (
                Text2d::new(text.clone()),
                TextFont {
                    font_size: TEXT_FONT_SIZE,
                    ..default()
                },
                Transform::from_translation(Vec3::new(
                    TEXT_SHADOW_OFFSET,
                    TEXTBOX_OFFSET_FROM_CENTER_Y - TEXT_SHADOW_OFFSET,
                    TEXT_SHADOW_Z
                ))
                .with_scale(Vec3::splat(1.)),
                TextColor(BLACK.into()),
                TextBoxText::new(TEXTBOX_FADE_IN_TIME, spawn_time, BLACK.into())
            ),
            (
                Text2d::new(text),
                TextFont {
                    font_size: TEXT_FONT_SIZE,
                    ..default()
                },
                Transform::from_translation(Vec3::new(0., TEXTBOX_OFFSET_FROM_CENTER_Y, TEXT_Z))
                    .with_scale(Vec3::splat(1.)),
                TextColor(GHOST_WHITE.into()),
                TextBoxText::new(TEXTBOX_FADE_IN_TIME, spawn_time, GHOST_WHITE.into())
            ),
        ],
    )
}

fn text_box_next_indicator(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    spawn_time: f32,
) -> impl Bundle {
    let triangle_mesh = get_colored_triangle_mesh(
        TRIANGLE_INDICATOR_HEIGHT,
        TRIANGLE_INDICATOR_WIDTH,
        spawn_time.sin(),
    );
    let triangle_mesh_handle = meshes.add(triangle_mesh);
    (
        Transform::default(),
        children![(
            Mesh2d(triangle_mesh_handle),
            MeshMaterial2d(materials.add(ColorMaterial::default())),
            Transform::from_translation(Vec3::new(
                0.,
                get_indicator_y(spawn_time),
                TRIANGLE_MESH_Z,
            ))
            .with_scale(Vec3::splat(1.)),
            TextBoxIndicator
        )],
        StateScoped(Screen::Gameplay),
    )
}

fn get_text_box_mesh(with_inner_vertices: bool, alpha: f32) -> Mesh {
    let half_height = TEXTBOX_HEIGHT * 0.5;
    let half_width = TEXTBOX_WIDTH * 0.5;
    let inner_height = half_height - LINE_THICKNESS;
    let inner_width = half_width - LINE_THICKNESS;
    let mut vertices = vec![
        [-half_width, half_height, 0.],
        [half_width, half_height, 0.],
        [half_width, -half_height, 0.],
        [-half_width, -half_height, 0.],
    ];
    if with_inner_vertices {
        vertices.extend([
            [-inner_width, inner_height, 0.],
            [inner_width, inner_height, 0.],
            [inner_width, -inner_height, 0.],
            [-inner_width, -inner_height, 0.],
        ]);
    }
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_indices(Indices::U32(if with_inner_vertices {
        vec![
            0, 1, 4, 4, 1, 5, 5, 1, 2, 6, 5, 2, 3, 7, 6, 2, 3, 6, 3, 4, 7, 4, 3, 0,
        ]
    } else {
        vec![0, 1, 2, 3, 0, 2]
    }));
    // Build vertex colors for the quad. One entry per vertex (the corners of the quad)
    let mut vertex_colors: Vec<[f32; 4]> = vec![
        LinearRgba::new(0.95, 0.05, 0.2, alpha).to_f32_array(),
        LinearRgba::new(0.97, 0.0, 0.17, alpha).to_f32_array(),
        LinearRgba::new(0.98, 0.0, 0.1, alpha).to_f32_array(),
        LinearRgba::new(0.92, 0.1, 0.1, alpha).to_f32_array(),
    ];
    if with_inner_vertices {
        vertex_colors.extend([
            LinearRgba::new(0.95, 0.05, 0.2, alpha).to_f32_array(),
            LinearRgba::new(0.97, 0.0, 0.17, alpha).to_f32_array(),
            LinearRgba::new(0.98, 0.0, 0.1, alpha).to_f32_array(),
            LinearRgba::new(0.92, 0.1, 0.1, alpha).to_f32_array(),
        ]);
    }
    mesh.with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors)
}

fn get_colored_triangle_mesh(height: f32, width: f32, t: f32) -> Mesh {
    let half_height = height / 2.0;
    let half_width = width / 2.0;
    let vertices = vec![
        [-half_width, half_height, 0.0],
        [half_width, half_height, 0.0],
        [0.0, -half_height, 0.0],
    ];
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_indices(Indices::U32(vec![0, 1, 2]));
    // Build vertex colors for the triangle. One entry per vertex (the corners of the triangle)

    let color1 = if t >= 0. {
        LinearRgba::new(0.05, 0.2, 0.95, 1.0).mix(&LinearRgba::new(0.05, 0.95, 0.2, 1.0), t)
    } else {
        LinearRgba::new(0.05, 0.2, 0.95, 1.0).mix(&LinearRgba::new(0.95, 0.90, 0.2, 1.0), t.abs())
    };
    let color2 = if t >= 0. {
        LinearRgba::new(0.0, 0.17, 0.97, 1.0).mix(&LinearRgba::new(0.0, 0.97, 0.17, 1.0), t)
    } else {
        LinearRgba::new(0.0, 0.17, 0.97, 1.0).mix(&LinearRgba::new(0.97, 0.95, 0.17, 1.0), t.abs())
    };
    let color3 = if t >= 0. {
        LinearRgba::new(0.0, 0.1, 0.98, 1.0).mix(&LinearRgba::new(0.0, 0.98, 0.1, 1.0), t)
    } else {
        LinearRgba::new(0.0, 0.1, 0.98, 1.0).mix(&LinearRgba::new(0.98, 0.97, 0.3, 1.0), t.abs())
    };

    let vertex_colors: Vec<[f32; 4]> = vec![
        color1.to_f32_array(),
        color2.to_f32_array(),
        color3.to_f32_array(),
    ];
    mesh.with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors)
}
