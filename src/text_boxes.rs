use bevy::{
    color::palettes::css::{BLACK, GHOST_WHITE},
    prelude::*,
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TextBox;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TextBox>();
}

pub const TEXTBOX_OFFSET_FROM_CENTER_Y: f32 = -150.;
pub const TEXTBOX_BG_ALPHA: f32 = 1.0;
pub const TEXTBOX_BG_SHADOW_ALPHA: f32 = 0.2;
pub const TEXTBOX_BG_SHADOW_OFFSET: f32 = 8.0;

pub const TEXTBOX_WIDTH: f32 = 700.;
pub const TEXTBOX_HEIGHT: f32 = 200.;

pub const BOX_BG_Z: f32 = 1.;
pub const BOX_BG_SHADOW_Z: f32 = 0.9;
pub const TEXT_SHADOW_Z: f32 = 1.1;
pub const TEXT_Z: f32 = 1.2;

pub const TEXT_SHADOW_OFFSET: f32 = 2.0;
pub const TEXT_FONT_SIZE: f32 = 25.0;

pub fn text_box(
    text: String,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    // default quad mesh
    let mut main_bg_mesh = Mesh::from(Rectangle::default());
    // Build vertex colors for the quad. One entry per vertex (the corners of the quad)
    let vertex_colors: Vec<[f32; 4]> = vec![
        LinearRgba::new(0.95, 0.05, 0.2, TEXTBOX_BG_ALPHA).to_f32_array(),
        LinearRgba::new(0.97, 0.0, 0.17, TEXTBOX_BG_ALPHA).to_f32_array(),
        LinearRgba::new(0.98, 0.0, 0.1, TEXTBOX_BG_ALPHA).to_f32_array(),
        LinearRgba::new(0.92, 0.1, 0.1, TEXTBOX_BG_ALPHA).to_f32_array(),
    ];

    // Insert the vertex colors as an attribute
    main_bg_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);

    let mut bg_shadow_mesh = Mesh::from(Rectangle::default());
    let bg_shadow_vertex_colors: Vec<[f32; 4]> = vec![
        LinearRgba::new(0.55, 0.0, 0.1, TEXTBOX_BG_SHADOW_ALPHA).to_f32_array(),
        LinearRgba::new(0.57, 0.0, 0.08, TEXTBOX_BG_SHADOW_ALPHA).to_f32_array(),
        LinearRgba::new(0.58, 0.0, 0.05, TEXTBOX_BG_SHADOW_ALPHA).to_f32_array(),
        LinearRgba::new(0.52, 0.05, 0.05, TEXTBOX_BG_SHADOW_ALPHA).to_f32_array(),
    ];
    bg_shadow_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, bg_shadow_vertex_colors);

    let main_bg_mesh_handle = meshes.add(main_bg_mesh);
    let bg_shadow_mesh_handle = meshes.add(bg_shadow_mesh);

    (
        TextBox,
        children![
            (
                Mesh2d(bg_shadow_mesh_handle),
                MeshMaterial2d(materials.add(ColorMaterial::default())),
                Transform::from_translation(Vec3::new(
                    TEXTBOX_BG_SHADOW_OFFSET,
                    TEXTBOX_OFFSET_FROM_CENTER_Y - TEXTBOX_BG_SHADOW_OFFSET,
                    BOX_BG_SHADOW_Z
                ))
                .with_scale(Vec3::new(TEXTBOX_WIDTH, TEXTBOX_HEIGHT, 1.)),
            ),
            (
                Mesh2d(main_bg_mesh_handle),
                MeshMaterial2d(materials.add(ColorMaterial::default())),
                Transform::from_translation(Vec3::new(0., TEXTBOX_OFFSET_FROM_CENTER_Y, BOX_BG_Z))
                    .with_scale(Vec3::new(TEXTBOX_WIDTH, TEXTBOX_HEIGHT, 1.)),
            ),
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
            ),
        ],
    )
}
