//! Shape

use super::*;

/// A shape.
pub struct Shape {
    pub(crate) dimensions: u32,
    pub(crate) components: u32,
    pub(crate) stride: u32,
    pub(crate) indices: Vec<u32>,
    pub(crate) vertices: Vec<f32>,
}

/// A shape builder.
pub struct ShapeBuilder<'a> {
    shader: &'a mut Shader,
    indices: Vec<u32>,
    vertices: Vec<f32>,
    temp_vertices: Vec<f32>,
}

impl<'a> ShapeBuilder<'a> {
    /// Create a new `ShapeBuilder` for a specific `Shader`.
    pub fn new(shader: &'a mut Shader) -> ShapeBuilder<'a> {
        ShapeBuilder {
            shader,
            indices: Vec::new(),
            vertices: Vec::new(),
            temp_vertices: Vec::new(),
        }
    }

    /// Set vertices for shape.
    pub fn vert(mut self, vertices: &[f32]) -> Self {
        self.temp_vertices = vertices.to_vec();
        self
    }

    /// Add a face to the shape.
    pub fn face(mut self, transform: Transform) -> Self {
        let dimensions = if self.shader.0.depth() {
            3
        } else {
            2
        };
        let components = if self.shader.0.blending() { 4 } else { 3 };
        let stride = dimensions
            + if self.shader.0.gradient() {
                components
            } else {
                0
            }
            + if self.shader.0.graphic() { 2 } else { 0 };
        let mut index = 0;
        // Loop through vertices.
        'v: loop {
            // Break out of loop.
            if index == self.temp_vertices.len() {
                break 'v;
            }
            // Read vertex position.
            let vertex = if dimensions == 2 {
                [
                    self.temp_vertices[index],
                    self.temp_vertices[index + 1],
                    0.0,
                ]
            } else {
                [
                    self.temp_vertices[index],
                    self.temp_vertices[index + 1],
                    self.temp_vertices[index + 2],
                ]
            };
            // Transform vertex position.
            let vertex = transform * vertex;
            // Find index to push to index buffer.
            let mut jndex = 0;
            self.indices.push('l: loop {
                // Haven't found the vertex, add to shader's vertex list.
                if jndex == self.vertices.len() {
                    let rtn = jndex / stride;
                    // Push transformed coordinates
                    for k in vertex.iter().take(dimensions) {
                        self.vertices.push(*k)
                    }
                    // Don't transform the data.
                    for k in dimensions..stride {
                        self.vertices.push(self.temp_vertices[index + k]);
                    }
                    break 'l rtn as u32;
                }

                // Test to see if vertex already exists.
                let mut equal = true;
                'b: for (k, _) in vertex.iter().enumerate().take(dimensions) {
                    if !nearly_equal(vertex[k], self.vertices[jndex + k]) {
                        equal = false;
                        break 'b;
                    }
                }
                'c: for k in dimensions..stride {
                    if !nearly_equal(
                        self.temp_vertices[index + k],
                        self.vertices[jndex + k],
                    ) {
                        equal = false;
                        break 'c;
                    }
                }
                if equal {
                    break 'l (jndex / stride) as u32;
                }
                jndex += stride;
            });

            index += stride;
        }

        self
    }

    /// Finish building the shape.
    pub fn finish(self) -> Shape {
        let dimensions = if self.shader.0.depth() {
            3
        } else {
            2
        };
        let components = if self.shader.0.gradient() {
            if self.shader.0.blending() {
                4
            } else {
                3
            }
        } else {
            0
        };
        let stride = dimensions
            + components
            + if self.shader.0.graphic() { 2 } else { 0 };

        Shape {
            indices: self.indices,
            vertices: self.vertices,
            dimensions,
            components,
            stride,
        }
    }
}
