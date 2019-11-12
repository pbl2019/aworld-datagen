use crate::context::Context;
use crate::models::terrain::*;
use crate::models::{Object, ObjectId};

impl Context {
    pub fn raycast(&self, x0: f32, y0: f32, angle: f32, distance: f32) -> Option<Obstacle> {
        let x1 = x0 + distance * angle.cos();
        let y1 = y0 + distance * angle.sin();
        let object_ids = self.terrain.object_ids.read();
        let objects = self.fetch_objects(object_ids);
        let mut objects = objects
            .iter()
            .map(|object| match object {
                Object::Character(local) => {
                    let x2 = local.x.read();
                    let y2 = local.y.read();
                    let pos_to_eye = ((x2 - x0).powf(2.0) + (y2 - y0).powf(2.0)).sqrt();
                    let cross = (x1 - x0) * (x2 - x0) + (y1 - y0) * (y2 - y0);
                    if (cross - pos_to_eye).abs() < 0.001 && distance >= pos_to_eye {
                        Some((ObjectId::Character(local.model.id), pos_to_eye))
                    } else {
                        None
                    }
                }
                Object::Item(_local) => unimplemented!(),
            })
            .collect::<Vec<Option<(ObjectId, f32)>>>();
        objects.retain(|op| op.is_some());
        let mut objects: Vec<(ObjectId, f32)> = objects.into_iter().map(|op| op.unwrap()).collect();
        objects.sort_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());
        objects.first().map(|ob| Obstacle::Object(ob.0))
        // TODO: Terrainとの当たり判定
    }
}
