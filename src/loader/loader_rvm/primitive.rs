use std::io::Read;

use crate::Error;

use byteorder::{BigEndian, ReadBytesExt};
use log::trace;

/// The data for a single primitive
#[derive(Debug)]
pub enum Primitive {
    Box(BoxData),
    Pyramid(PyramidData),
    RectangularTorus(RectangularTorusData),
    CircularTorus(CircularTorusData),
    EllipticalDish(EllipticalDishData),
    SphericalDish(SphericalDishData),
    Snout(SnoutData),
    Cylinder(CylinderData),
    Sphere(SphereData),
    Line(LineData),
    Polygons(PolygonsData),
}

impl Primitive {
    /// Read the primitive data from the reader.
    ///
    /// # Arguments
    /// * `reader` - The reader to read the data from.
    pub fn from_reader<R: Read>(reader: &mut R, primitive_type: u32) -> Result<Self, Error> {
        match primitive_type {
            1 => PyramidData::from_reader(reader).map(Primitive::Pyramid),
            2 => BoxData::from_reader(reader).map(Primitive::Box),
            3 => RectangularTorusData::from_reader(reader).map(Primitive::RectangularTorus),
            4 => CircularTorusData::from_reader(reader).map(Primitive::CircularTorus),
            5 => EllipticalDishData::from_reader(reader).map(Primitive::EllipticalDish),
            6 => SphericalDishData::from_reader(reader).map(Primitive::SphericalDish),
            7 => SnoutData::from_reader(reader).map(Primitive::Snout),
            8 => CylinderData::from_reader(reader).map(Primitive::Cylinder),
            9 => SphereData::from_reader(reader).map(Primitive::Sphere),
            10 => LineData::from_reader(reader).map(Primitive::Line),
            11 => PolygonsData::from_reader(reader).map(Primitive::Polygons),
            _ => Err(Error::InvalidFormat(format!(
                "Unknown primitive type: {}",
                primitive_type
            ))),
        }
    }

    /// Returns the name of the primitive type.
    pub fn name(&self) -> &str {
        match self {
            Primitive::Box(_) => "Box",
            Primitive::Pyramid(_) => "Pyramid",
            Primitive::RectangularTorus(_) => "RectangularTorus",
            Primitive::CircularTorus(_) => "CircularTorus",
            Primitive::EllipticalDish(_) => "EllipticalDish",
            Primitive::SphericalDish(_) => "SphericalDish",
            Primitive::Snout(_) => "Snout",
            Primitive::Cylinder(_) => "Cylinder",
            Primitive::Sphere(_) => "Sphere",
            Primitive::Line(_) => "Line",
            Primitive::Polygons(_) => "Polygons",
        }
    }
}

/// The trait for all primitive data types.
pub trait PrimitiveData: Default {
    /// Read the primitive data from the reader.
    ///
    /// # Arguments
    /// * `reader` - The reader to read the data from.
    fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error>;
}

/// A box whose center is at the origin with the specified size.
#[derive(Debug, Default)]
pub struct BoxData {
    /// The size of the box along the x, y, and z axes.
    pub inner: [f32; 3],
}

impl BoxData {
    /// Get the size of the box along the x axis.
    #[inline]
    pub fn size_x(&self) -> f32 {
        self.inner[0]
    }

    /// Get the size of the box along the y axis.
    #[inline]
    pub fn size_y(&self) -> f32 {
        self.inner[1]
    }

    /// Get the size of the box along the z axis.
    #[inline]
    pub fn size_z(&self) -> f32 {
        self.inner[2]
    }
}

impl PrimitiveData for BoxData {
    fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut inner = [0.0; 3];
        reader.read_f32_into::<BigEndian>(&mut inner)?;
        Ok(Self { inner })
    }
}

#[derive(Debug, Default)]
pub struct PyramidData {
    /// The values of the pyramid in the following order:
    /// xbottom, ybottom, xtop, ytop, xoffset, yoffset, height.
    pub inner: [f32; 7],
}

impl PyramidData {
    #[inline]
    pub fn xbottom(&self) -> f32 {
        self.inner[0]
    }

    #[inline]
    pub fn ybottom(&self) -> f32 {
        self.inner[1]
    }

    #[inline]
    pub fn xtop(&self) -> f32 {
        self.inner[2]
    }

    #[inline]
    pub fn ytop(&self) -> f32 {
        self.inner[3]
    }

    #[inline]
    pub fn xoffset(&self) -> f32 {
        self.inner[4]
    }

    #[inline]
    pub fn yoffset(&self) -> f32 {
        self.inner[5]
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.inner[6]
    }
}

impl PrimitiveData for PyramidData {
    fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut inner = [0.0; 7];
        reader.read_f32_into::<BigEndian>(&mut inner)?;
        Ok(Self { inner })
    }
}

#[derive(Debug, Default)]
pub struct RectangularTorusData {
    pub inner: [f32; 4],
}

impl RectangularTorusData {
    #[inline]
    pub fn rinside(&self) -> f32 {
        self.inner[0]
    }

    #[inline]
    pub fn routside(&self) -> f32 {
        self.inner[1]
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.inner[2]
    }

    #[inline]
    pub fn angle(&self) -> f32 {
        self.inner[3]
    }
}

impl PrimitiveData for RectangularTorusData {
    fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut inner = [0.0; 4];
        reader.read_f32_into::<BigEndian>(&mut inner)?;
        Ok(Self { inner })
    }
}

#[derive(Debug, Default)]
pub struct CircularTorusData {
    pub inner: [f32; 3],
}

impl CircularTorusData {
    #[inline]
    pub fn offset(&self) -> f32 {
        self.inner[0]
    }

    #[inline]
    pub fn radius(&self) -> f32 {
        self.inner[1]
    }

    #[inline]
    pub fn angle(&self) -> f32 {
        self.inner[2]
    }
}

impl PrimitiveData for CircularTorusData {
    fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut inner = [0.0; 3];
        reader.read_f32_into::<BigEndian>(&mut inner)?;
        Ok(Self { inner })
    }
}

#[derive(Debug, Default)]
pub struct EllipticalDishData {
    pub inner: [f32; 2],
}

impl EllipticalDishData {
    #[inline]
    pub fn diameter(&self) -> f32 {
        self.inner[0]
    }

    #[inline]
    pub fn radius(&self) -> f32 {
        self.inner[1]
    }
}

impl PrimitiveData for EllipticalDishData {
    fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut inner = [0.0; 2];
        reader.read_f32_into::<BigEndian>(&mut inner)?;
        Ok(Self { inner })
    }
}

#[derive(Debug, Default)]
pub struct SphericalDishData {
    pub inner: [f32; 2],
}

impl SphericalDishData {
    #[inline]
    pub fn diameter(&self) -> f32 {
        self.inner[0]
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.inner[1]
    }
}

impl PrimitiveData for SphericalDishData {
    fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut inner = [0.0; 2];
        reader.read_f32_into::<BigEndian>(&mut inner)?;
        Ok(Self { inner })
    }
}

#[derive(Debug, Default)]
pub struct SnoutData {
    pub inner: [f32; 9],
}

impl SnoutData {
    #[inline]
    pub fn dbottom(&self) -> f32 {
        self.inner[0]
    }

    #[inline]
    pub fn dtop(&self) -> f32 {
        self.inner[1]
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.inner[2]
    }

    #[inline]
    pub fn xoffset(&self) -> f32 {
        self.inner[3]
    }

    #[inline]
    pub fn yoffset(&self) -> f32 {
        self.inner[4]
    }

    #[inline]
    pub fn xbshear(&self) -> f32 {
        self.inner[5]
    }

    #[inline]
    pub fn ybshear(&self) -> f32 {
        self.inner[6]
    }

    #[inline]
    pub fn xtshear(&self) -> f32 {
        self.inner[7]
    }

    #[inline]
    pub fn ytshear(&self) -> f32 {
        self.inner[8]
    }
}

impl PrimitiveData for SnoutData {
    fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut inner = [0.0; 9];
        reader.read_f32_into::<BigEndian>(&mut inner)?;
        Ok(Self { inner })
    }
}

/// A line along the X axis defined by start and end values.
#[derive(Debug, Default)]
pub struct LineData {
    pub inner: [f32; 2],
}

impl LineData {
    #[inline]
    pub fn start(&self) -> f32 {
        self.inner[0]
    }

    #[inline]
    pub fn end(&self) -> f32 {
        self.inner[1]
    }
}

impl PrimitiveData for LineData {
    fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut inner = [0.0; 2];
        reader.read_f32_into::<BigEndian>(&mut inner)?;
        Ok(Self { inner })
    }
}

#[derive(Debug, Default)]
pub struct CylinderData {
    /// The radius and height of the cylinder in millimeters.
    pub inner: [f32; 2],
}

impl CylinderData {
    /// Get the radius of the cylinder in millimeters.
    #[inline]
    pub fn radius(&self) -> f32 {
        self.inner[0]
    }

    /// Get the height of the cylinder in millimeters.
    #[inline]
    pub fn height(&self) -> f32 {
        self.inner[1]
    }
}

impl PrimitiveData for CylinderData {
    fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut inner = [0.0; 2];
        reader.read_f32_into::<BigEndian>(&mut inner)?;
        Ok(Self { inner })
    }
}

#[derive(Debug, Default)]
pub struct SphereData {
    /// The diameter of the sphere in millimeters.
    pub diameter: f32,
}

impl SphereData {
    /// Get the diameter of the sphere in millimeters.
    #[inline]
    pub fn diameter(&self) -> f32 {
        self.diameter
    }
}

impl PrimitiveData for SphereData {
    fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let diameter = reader.read_f32::<BigEndian>()?;
        Ok(Self { diameter })
    }
}

/// A list of facets.
#[derive(Debug, Default)]
pub struct PolygonsData {
    pub inner: Vec<Polygon>,
}

impl PrimitiveData for PolygonsData {
    fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let num_polygons = reader.read_u32::<BigEndian>()? as usize;
        trace!("Number of polygons: {}", num_polygons);

        let mut polygons = Vec::with_capacity(num_polygons);
        for _ in 0..num_polygons {
            let num_contours = reader.read_u32::<BigEndian>()? as usize;
            trace!("Number of contours: {}", num_contours);
            if num_contours == 0 {
                return Err(Error::InvalidFormat(
                    "Number of contours is zero".to_owned(),
                ));
            }

            let mut contours = Vec::with_capacity(num_contours);
            for _ in 0..num_contours {
                let num_vertices = reader.read_u32::<BigEndian>()? as usize;
                trace!("Number of vertices: {}", num_vertices);
                if num_vertices == 0 {
                    return Err(Error::InvalidFormat(
                        "Number of vertices is zero".to_owned(),
                    ));
                }

                let mut vertices = Vec::with_capacity(num_vertices);
                for _ in 0..num_vertices {
                    let mut vertex = [0.0; 6];
                    reader.read_f32_into::<BigEndian>(&mut vertex)?;
                    vertices.push(Vertex { inner: vertex });
                }

                contours.push(Contour { inner: vertices });
            }

            polygons.push(Polygon { contours });
        }

        Ok(Self { inner: polygons })
    }
}

/// A facet defined by a list of loops, where the outer loop is the first loop.
#[derive(Debug, Default)]
pub struct Polygon {
    /// Contours of the polygon, where the first contour is the outer contour.
    pub contours: Vec<Contour>,
}

/// A contour defined by a list of vertices.
#[derive(Debug, Default)]
pub struct Contour {
    pub inner: Vec<Vertex>,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Vertex {
    /// The position as (x, y, z) in millimeters followed by the normal as (nx, ny, nz).
    pub inner: [f32; 6],
}

impl Vertex {
    #[inline]
    pub fn x(&self) -> f32 {
        self.inner[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.inner[1]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.inner[2]
    }

    #[inline]
    pub fn position(&self) -> [f32; 3] {
        [self.x(), self.y(), self.z()]
    }

    #[inline]
    pub fn nx(&self) -> f32 {
        self.inner[3]
    }

    #[inline]
    pub fn ny(&self) -> f32 {
        self.inner[4]
    }

    #[inline]
    pub fn nz(&self) -> f32 {
        self.inner[5]
    }

    #[inline]
    pub fn normal(&self) -> [f32; 3] {
        [self.nx(), self.ny(), self.nz()]
    }
}
