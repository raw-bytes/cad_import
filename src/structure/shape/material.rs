use crate::basic_types::{Color, RGB};

/// The material of a shape
pub enum Material {
    /// No material is assigned to the shape
    None,
    /// A Phong material is assigned to the shape
    PhongMaterial(PhongMaterialData),
}

impl Default for Material {
    fn default() -> Self {
        Material::None
    }
}

pub struct PhongMaterialData {
    /// The transparency field specifies how "clear" an object is, with 1.0 being completely
    /// transparent, and 0.0 completely opaque.
    pub transparency: f32,

    /// The specular color and shininess fields determine the specular highlights
    /// (e.g., the shiny spots on an apple).
    /// When the angle from the light to the surface is close to the angle from the surface to
    /// the viewer, the specularColor is added to the diffuse and ambient color calculations.
    /// Lower shininess values produce soft glows, while higher values result in sharper,
    /// smaller highlights.
    pub specular_color: RGB,
    pub shininess: f32,

    /// The emissive color field models "glowing" objects. This can be useful for displaying
    /// pre-lit models (where the light energy of the room is computed explicitly), or for
    /// displaying scientific data.
    pub emissive_color: RGB,

    /// The diffuse color field reflects all light sources depending on the angle of the surface
    /// with respect to the light source. The more directly the surface faces the light, the more
    /// diffuse light reflects.
    pub diffuse_color: RGB,

    /// The ambient intensity field specifies how much ambient light from light sources this
    /// surface shall reflect. Ambient light is omnidirectional and depends only on the number
    /// of light sources, not their positions with respect to the surface. Ambient color is
    /// calculated as ambientIntensity × diffuse color.
    pub ambient_intensity: f32,
}

impl Default for PhongMaterialData {
    fn default() -> Self {
        Self {
            ambient_intensity: 0.2,
            diffuse_color: RGB::new(0.8, 0.8, 0.8),
            emissive_color: RGB::black(),
            shininess: 0.2,
            specular_color: RGB::black(),
            transparency: 0f32,
        }
    }
}
