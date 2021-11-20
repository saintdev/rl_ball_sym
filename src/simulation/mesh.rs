use std::borrow::Cow;

use super::geometry::Tri;
use crate::linear_algebra::mat::Mat3;
use crate::linear_algebra::math::dot;
use vvec3::Vec3;

#[derive(Clone, Debug)]
pub struct Mesh<'a> {
    pub ids: Cow<'a, [i32]>,
    pub vertices: Cow<'a, [f32]>,
}

impl Default for Mesh<'_> {
    fn default() -> Self {
        Self {
            ids: Cow::default(),
            vertices: Cow::default(),
        }
    }
}

impl Mesh<'_> {
    pub fn from(other_meshes: &[&Mesh]) -> Self {
        let mut id_offset = 0;

        let mut n_ids = 0;
        let mut n_vertices = 0;

        for m in other_meshes {
            n_ids += m.ids.len();
            n_vertices += m.vertices.len();
        }

        let mut ids: Vec<i32> = Vec::with_capacity(n_ids);
        let mut vertices: Vec<f32> = Vec::with_capacity(n_vertices);

        for m in other_meshes {
            for id in m.ids.iter() {
                ids.push(id + id_offset);
            }

            for vertex in m.vertices.iter() {
                vertices.push(*vertex);
            }

            id_offset += (m.vertices.len() / 3) as i32;
        }

        // remove duplicate vertices and their ids
        // let mut i = 0;
        // let mut dups = 0;

        // loop {
        //     let id = (ids[i] * 3) as usize;
        //     match vertices[..id].iter().position(|&x| x == vertices[id + 0]) {
        //         Some(j) => {
        //             if vertices[j + 1] == vertices[id + 1] && vertices[j + 2] == vertices[id + 2] {
        //                 // dups += 1;
        //                 n_ids -= 3;

        //                 vertices.remove(id + 0);
        //                 vertices.remove(id + 1);
        //                 vertices.remove(id + 2);

        //                 ids.remove(i + 0);
        //                 ids.remove(i + 1);
        //                 ids.remove(i + 2);

        //                 for l in i..n_ids {
        //                     ids[l] -= 3;
        //                 }
        //             }
        //         }
        //         None => {}
        //     };

        //     i += 3;
        //     if i >= n_ids {
        //         break;
        //     }
        // }
        // println!("Removed {} duplicate vertices!", dups);

        Self {
            ids: Cow::from(ids),
            vertices: Cow::from(vertices),
        }
    }

    pub fn transform(&self, a: Mat3) -> Self {
        debug_assert_eq!(self.vertices.len() % 3, 0);
        debug_assert_eq!(self.ids.len() % 3, 0);

        let vertices = self
            .vertices
            .chunks(3)
            .flat_map(|vertex| {
                let v = dot(a, Vec3::new(vertex[0], vertex[1], vertex[2]));
                [v.x, v.y, v.z]
            })
            .collect();

        // for transformations that flip things
        // inside-out, change triangle winding
        let ids = if a.det() < 0. {
            self.ids.chunks(3).flat_map(|ids| [ids[1], ids[0], ids[2]]).collect()
        } else {
            self.ids.clone()
        };

        Mesh {
            ids,
            vertices,
        }
    }

    pub fn translate(&self, p: Vec3) -> Self {
        debug_assert_eq!(self.vertices.len() % 3, 0);
        let vertices = self.vertices.chunks(3).flat_map(|vertex| [vertex[0] + p.x, vertex[1] + p.y, vertex[2] + p.z]).collect();

        Self {
            ids: self.ids.clone(),
            vertices,
        }
    }

    #[rustfmt::skip]
    pub fn to_triangles(&self) -> Vec<Tri> {
        let n = self.ids.len() / 3;
        let mut triangles: Vec<Tri> = Vec::with_capacity(n);

        for i in 0..n {
            triangles.push(Tri::default());
            for j in 0..3 {
                let id = (self.ids[i * 3 + j] * 3) as usize;
                triangles[i].p[j].x = self.vertices[id    ] as f32;
                triangles[i].p[j].y = self.vertices[id + 1] as f32;
                triangles[i].p[j].z = self.vertices[id + 2] as f32;
            }
        }

        triangles
    }
}
