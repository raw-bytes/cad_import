use std::{collections::HashMap, rc::Rc};

use crate::{
    structure::{Material, PhongMaterialData},
    RGB,
};

/// The material manager manages the materials of the RVM file format.
#[derive(Default, Clone)]
pub struct RVMMaterialManager {
    materials: HashMap<u8, Rc<Material>>,
}

impl RVMMaterialManager {
    /// Creates a new material manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the material of the given index. If the material does not exist, it will be created
    /// based on the RVM color palette.
    ///
    /// # Arguments
    /// * `index` - The index of the material to return.
    pub fn create_material(&mut self, index: u8) -> Rc<Material> {
        // Check if the material already exists and return it.
        if let Some(material) = self.materials.get(&index) {
            return material.clone();
        }

        // Create the material based on the RVM color palette.
        let color = RVM_COLORS[index as usize];
        let phong_data = PhongMaterialData {
            diffuse_color: RGB::new(
                color[0] as f32 / 255f32,
                color[1] as f32 / 255f32,
                color[2] as f32 / 255f32,
            ),
            ..PhongMaterialData::default()
        };

        let material = Rc::new(Material::PhongMaterial(phong_data));
        self.materials.insert(index, material.clone());

        material
    }
}

/// The RGB colors of the Navisworks color palette.
const RVM_COLORS: [[u8; 3]; 256] = [
    [80, 80, 80],    // unknown
    [0, 0, 0],       // Black
    [80, 0, 0],      //Red
    [93, 60, 0],     // Orange
    [80, 80, 0],     // Yellow
    [0, 80, 0],      // Green
    [0, 93, 93],     // Cyan
    [0, 0, 80],      // Blue
    [87, 0, 87],     // Magenta
    [80, 17, 17],    // Brown
    [100, 100, 100], // White
    [98, 50, 44],    // Salmon
    [75, 75, 75],    // LightGrey
    [66, 66, 66],    // Grey
    [55, 40, 55],    // Plum
    [96, 96, 96],    // WhiteSmoke
    [56, 14, 42],    // Maroon
    [0, 100, 50],    // SpringGreen
    [96, 87, 70],    // Wheat
    [93, 79, 20],    // Gold
    [28, 46, 100],   // RoyalBlue
    [93, 91, 67],    // LightGold
    [93, 7, 54],     // DeepPink
    [14, 56, 14],    // ForestGreen
    [100, 65, 0],    // BrightOrange
    [93, 93, 88],    // Ivory
    [93, 46, 13],    // Chocolate
    [28, 51, 71],    // SteelBlue
    [100, 100, 100], // White
    [18, 18, 31],    // Midnight
    [0, 0, 50],      // NavyBlue
    [80, 57, 62],    // Pink
    [80, 36, 27],    // CoralRed
    [0, 0, 0],       // Black
    [80, 0, 0],      // Red
    [93, 60, 0],     // Orange
    [80, 80, 0],     // Yellow
    [0, 80, 0],      // Green
    [0, 93, 93],     // Cyan
    [0, 0, 80],      // Blue
    [87, 0, 87],     // Magenta
    [80, 17, 17],    // Brown
    [100, 100, 100], // White
    [98, 50, 44],    // Salmon
    [75, 75, 75],    // LightGrey
    [66, 66, 66],    // Grey
    [55, 40, 55],    // Plum
    [96, 96, 96],    // WhiteSmoke
    [56, 14, 42],    // Maroon
    [0, 100, 50],    // SpringGreen
    [96, 87, 70],    // Wheat
    [93, 79, 20],    // Gold
    [28, 46, 100],   // RoyalBlue
    [93, 91, 67],    // LightGold
    [93, 7, 54],     // DeepPink
    [14, 56, 14],    // ForestGreen
    [100, 65, 0],    // BrightOrange
    [93, 93, 88],    // Ivory
    [93, 46, 13],    // Chocolate
    [28, 51, 71],    // SteelBlue
    [100, 100, 100], // White
    [18, 18, 31],    // Midnight
    [0, 0, 50],      // NavyBlue
    [80, 57, 62],    // Pink
    [80, 36, 27],    // CoralRed
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [80, 80, 80],    // unknown
    [0, 0, 0],       // Black
    [100, 100, 100], // White
    [96, 96, 96],    // WhiteSmoke
    [93, 93, 88],    // Ivory
    [66, 66, 66],    // Grey
    [75, 75, 75],    // LightGrey
    [32, 55, 55],    // DarkGrey
    [18, 31, 31],    // DarkSlate
    [80, 0, 0],      // Red
    [100, 0, 0],     // BrightRed
    [80, 36, 27],    // CoralRed
    [100, 39, 28],   // Tomato
    [55, 40, 55],    // Plum
    [93, 7, 54],     // DeepPink
    [80, 57, 62],    // Pink
    [98, 50, 44],    // Salmon
    [93, 60, 0],     // Orange
    [100, 65, 0],    // BrightOrange
    [100, 50, 0],    // OrangeRed
    [56, 14, 42],    // Maroon
    [80, 80, 0],     // Yellow
    [93, 79, 20],    // Gold
    [93, 93, 82],    // LightYellow
    [93, 91, 67],    // LightGold
    [60, 80, 20],    // YellowGreen
    [0, 100, 50],    // SpringGreen
    [0, 80, 0],      // Green
    [14, 56, 14],    // ForestGreen
    [18, 31, 18],    // DarkGreen
    [0, 93, 93],     // Cyan
    [0, 75, 80],     // Turquoise
    [46, 93, 78],    // Aquamarine
    [0, 0, 80],      // Blue
    [28, 46, 100],   // RoyalBlue
    [0, 0, 50],      // NavyBlue
    [69, 88, 90],    // PowderBlue
    [18, 18, 31],    // Midnight
    [28, 51, 71],    // SteelBlue
    [20, 0, 40],     // Indigo
    [40, 0, 60],     // Mauve
    [93, 51, 93],    // Violet
    [87, 0, 87],     // Magenta
    [96, 96, 86],    // Beige
    [96, 87, 70],    // Wheat
    [86, 58, 44],    // Tan
    [96, 65, 37],    // SandyBrown
    [80, 17, 17],    // Brown
    [62, 62, 37],    // Khaki
    [93, 46, 13],    // Chocolate
    [55, 27, 8],     // DarkBrown
];
