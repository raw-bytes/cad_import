use std::rc::Rc;

use log::{debug, error, trace, warn};

use nalgebra_glm::{Mat3, Mat4, Vec3};

use crate::{
    loader::{loader_rvm::tessellate::Tessellate, TessellationOptions},
    structure::{CADData, NodeId, Shape, ShapePart, Tree},
    Length,
};

use super::{
    material::RVMMaterialManager,
    primitive::Primitive,
    rvm_parser::{RVMHeader, RVMInterpreter, RVMModelHeader},
};

/// The CAD creator creates the cad data structure based on the provided read events.
pub struct CADDataCreator {
    node_stack: Vec<NodeId>,
    tessellation_options: TessellationOptions,
    tree: Tree,
    shape: Option<Shape>,
    material_map: RVMMaterialManager,
}

impl CADDataCreator {
    pub fn new(tessellation_options: TessellationOptions) -> Self {
        Self {
            node_stack: Vec::new(),
            tessellation_options,
            tree: Tree::new(),
            shape: None,
            material_map: RVMMaterialManager::new(),
        }
    }

    /// Transforms the cad data creator to an cad data object.
    pub fn to_cad_data(self) -> CADData {
        let mut cad_data = CADData::new(self.tree);
        cad_data.change_length_unit(Length::MILLIMETER);

        cad_data
    }
}

impl RVMInterpreter for CADDataCreator {
    fn header(&mut self, header: RVMHeader) {
        debug!("Header: {:?}", header);
    }

    fn model(&mut self, header: RVMModelHeader) {
        debug!("Model: {:?}", header);

        let root_id = self.tree.create_node(header.model_name.clone());
        self.node_stack.push(root_id);
    }

    fn primitive(&mut self, primitive: Primitive, transform: &Mat3, translation: &Vec3) {
        trace!(
            "Tessellate Primitive {}: {:?}, M={:?}, T={:?}",
            primitive.name(),
            primitive,
            transform,
            translation
        );

        let result = match primitive {
            Primitive::Box(box_data) => {
                Some(box_data.tessellate(&self.tessellation_options, transform, translation))
            }
            Primitive::Cylinder(cylinder_data) => {
                Some(cylinder_data.tessellate(&self.tessellation_options, transform, translation))
            }
            Primitive::Sphere(sphere_data) => {
                Some(sphere_data.tessellate(&self.tessellation_options, transform, translation))
            }
            Primitive::Polygons(polygons_data) => {
                Some(polygons_data.tessellate(&self.tessellation_options, transform, translation))
            }
            Primitive::Pyramid(pyramid_data) => {
                Some(pyramid_data.tessellate(&self.tessellation_options, transform, translation))
            }
            _ => {
                warn!("Primitive type {} not supported", primitive.name());
                None
            }
        };

        if let Some(mesh) = result {
            match mesh {
                Ok(mesh) => {
                    let mut shape = if let Some(shape) = self.shape.take() {
                        shape
                    } else {
                        Shape::new()
                    };

                    let shape_part = ShapePart::new(Rc::new(mesh));
                    shape.add_part(shape_part);

                    self.shape = Some(shape);
                }
                Err(err) => {
                    error!("Tessellation failed: {:?}", err);
                }
            }
        }
    }

    fn begin_group(&mut self, group_name: String, translation: Vec3, material_id: usize) {
        trace!(
            "Group: {:?}, {:?}, {:?}",
            group_name,
            translation,
            material_id
        );

        assert!(
            material_id < 256,
            "Material id is out of range. It should be between 0 and 255, but is {}",
            material_id
        );
        let material = self.material_map.create_material(material_id as u8);

        let parent_id = *self.node_stack.last().expect("No parent node found");
        let new_id = self.tree.create_node_with_parent(group_name, parent_id);
        let node = self.tree.get_node_mut(new_id).unwrap();

        let translation_mat: Mat4 = nalgebra_glm::translation(&translation);
        node.set_transform(translation_mat);
        node.set_material(material);

        self.node_stack.push(new_id);
    }

    fn end_group(&mut self) {
        assert!(self.node_stack.len() > 1);

        let node_id = self.node_stack.pop().unwrap();

        // Check if there is a shape to attach for the current group node.
        if let Some(shape) = self.shape.take() {
            let node = self.tree.get_node_mut(node_id).unwrap();

            node.attach_shape(Rc::new(shape));

            self.shape = None;
        }

        trace!("End group");
    }
}
