use crate::context::Context;
use crate::models::terrain::*;
use crate::models::{Object, ObjectId};
use crate::utils::intersects_circle_with_line;

impl Context {
    pub fn raycast(&self, x0: f32, y0: f32, angle: f32, distance: f32) -> Option<Obstacle> {
        let x1 = x0 + distance * angle.cos();
        let y1 = y0 + distance * angle.sin();
        // let object_ids = self.terrain.object_ids.read();
        // let objects = self.fetch_objects(object_ids);
        let objects = self.get_objects();
        println!("{:?}", objects);
        let mut objects = objects
            .iter()
            .map(|object| match object {
                Object::Character(local) => {
                    let x2 = local.x.read();
                    let y2 = local.y.read();
                    if let Some(d) = intersects_circle_with_line(x2, y2, 10.0, x0, y0, x1, y1) {
                        println!("distance: {}", d);
                        Some((ObjectId::Character(local.model.id), d))
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
