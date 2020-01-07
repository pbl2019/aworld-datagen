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
        let mut objects = objects
            .iter()
            .map(|object| match object {
                Object::Character(local) => {
                    let x2 = local.x.read();
                    let y2 = local.y.read();
                    if let Some(d) = intersects_circle_with_line(x2, y2, 1.0, x0, y0, x1, y1) {
                        if d < 1.0 {
                            println!("distance: {}", d);
                            Some((ObjectId::Character(local.entity_id), d))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Object::Item(local) => {
                    if local.is_dropped.read() {
                        let x2 = local.x.read();
                        let y2 = local.y.read();
                        if let Some(d) = intersects_circle_with_line(x2, y2, 1.0, x0, y0, x1, y1) {
                            println!("item distance: {}", d);
                            Some((ObjectId::Item(local.entity_id), d))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            })
            .collect::<Vec<Option<(ObjectId, f32)>>>();
        objects.retain(|op| op.is_some());
        let mut objects: Vec<(ObjectId, f32)> = objects.into_iter().map(|op| op.unwrap()).collect();
        objects.sort_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());
        let mut obstacles: Vec<(Obstacle, f32)> = Vec::new();
        if let Some(ob) = objects.first() {
            obstacles.push((Obstacle::Object(ob.0), ob.1))
        }
        {
            let num_iter = distance * 100.0 + 10.0;
            let delta_x = (x1 - x0) / num_iter;
            let delta_y = (y1 - y0) / num_iter;
            let mut cur_x = x0;
            let mut cur_y = y0;
            let width = self.terrain.model.width as isize;
            let height = self.terrain.model.height as isize;
            let raw = self.terrain.raw.read();
            for i in 0..(num_iter as usize) + 1 {
                if cur_x < 0. || width as f32 <= cur_x || cur_y < 0. || height as f32 <= cur_y {
                    println!("overflow: {}, {}", cur_x, cur_y);
                    obstacles.push((
                        Obstacle::Terrain(TerrainInfo::Wall),
                        ((x0 - cur_x).powi(2) + (y0 - cur_y).powi(2)).sqrt(),
                    ));
                    break;
                }
                let ix = cur_x.floor() as isize;
                let iy = cur_y.floor() as isize;
                // tackle to world bounds
                if raw[(ix + iy * width) as usize] == TerrainInfo::Wall as u8 {
                    obstacles.push((
                        Obstacle::Terrain(TerrainInfo::Wall),
                        ((x0 - cur_x.floor()).powi(2) + (y0 - cur_y.floor()).powi(2)).sqrt(),
                    ));
                    break;
                }
                cur_x += delta_x;
                cur_y += delta_y;
            }
        }
        obstacles.sort_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());
        obstacles.get(0).map(|ob| ob.0)
    }
}
