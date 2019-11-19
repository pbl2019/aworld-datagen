use crate::init_field;
use crate::models::field::Field;
use crate::models::ObjectId;
use crate::schema::terrains;
use base64;
use chrono::Utc;
use rand::Rng;

#[derive(Queryable, Clone, Debug)]
pub struct Terrain {
    pub id: i64,
    pub content: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Insertable)]
#[table_name = "terrains"]
pub struct NewTerrain {
    pub content: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone)]
pub enum Obstacle {
    Object(ObjectId),
    Terrain(TerrainInfo),
}

#[derive(Clone, Debug)]
pub enum TerrainInfo {
    Floor = 0,
    Wall = 1,
}

#[derive(Debug)]
pub struct TerrainLocal {
    pub entity_id: i64,
    pub model: Terrain,

    pub raw: Field<Vec<u8>>,
    pub object_ids: Field<Vec<ObjectId>>,
}

impl NewTerrain {
    fn generate(width: usize, height: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        // 倍率
        let magnification = 2;
        let width_ = width/magnification;
        let height_ = height/magnification;
        let mut raw = Vec::with_capacity(width * height);
        let mut raw_ = Vec::with_capacity(width_ * height_);
        // 陸が現れる確率変数
        let mut rng_rate;
        let mut top_edge;
        let mut left_edge;
        let mut right_edge;
        let mut bottom_edge;
        for i in 0..width_ * height_ {
            rng_rate = 0.1;
            top_edge = 0;
            left_edge = 0;
            right_edge = 0;
            // 上端ならフラグ
            if i < width_ { top_edge = 1; }
            // 左端ならフラグ
            if (i+1) % width_ == 1 { left_edge = 1; }
            // 右端ならフラグ
            if (i+1) % width_ == 0 { right_edge = 1; }

            if top_edge==0 && left_edge==0 && raw_[i-width_-1]==0 { rng_rate += 0.15; }
            if top_edge==0 && raw_[i-width_]==0 { rng_rate += 0.25; }
            if top_edge==0 && right_edge==0 && raw_[i-width_+1]==0 { rng_rate += 0.15; }
            if left_edge==0 && raw_[i-1]==0 { rng_rate += 0.25; }

            if rng.gen_range(0., 1.) < rng_rate {
                raw_.push(0);
            } else {
                raw_.push(1);
            }

        }

        // rawを拡大(縦*2, 横*2)
        for h in 1..height_+1 {
            // 縦幅が奇数なら最後だけ倍率3倍
            for _ in 0..magnification {
                for w in 1..width_+1 {
                    raw.push(raw_[h*w-1]);
                    raw.push(raw_[h*w-1]);
                }
                // 横幅が奇数なら最後だけ倍率3倍
                if width%2==1 {
                    raw.push(rng.gen_range(0, 2));
                }
            }
        }
        if height%2==1 {
            for _ in 0..width {
                raw.push(rng.gen_range(0, 2));
            }
        }

        // 生成されたterrainを綺麗にするやつ
        for _ in 0..10 {
            for j in 0..width * height {
                rng_rate = 0.;
                top_edge = 0;
                left_edge = 0;
                right_edge = 0;
                bottom_edge = 0;
                // 上端ならフラグ
                if j < width {
                    top_edge = 1;
                    rng_rate += 0.3;
                }
                // 左端ならフラグ
                if (j+1) % width == 1 {
                    left_edge = 1;
                    rng_rate += 0.1;
                }
                // 右端ならフラグ
                if (j+1) % width == 0 {
                    right_edge = 1;
                    rng_rate += 0.1;
                }
                // 下端ならフラグ
                if j >= width * height - width {
                    bottom_edge = 1;
                    rng_rate += 0.3;
                }

                // 左上
                if top_edge==0 && left_edge==0 && raw[j-width-1]!=raw[j] {rng_rate += 0.1;}
                // 上
                if top_edge==0 && raw[j-width]!=raw[j] {rng_rate += 0.1;}
                // 右上
                if top_edge==0 && right_edge==0 && raw[j-width+1]!=raw[j] {rng_rate += 0.1;}
                // 左
                if left_edge==0 && raw[j-1]!=raw[j] {rng_rate += 0.1;}
                // 右
                if right_edge==0 && raw[j+1]!=raw[j] {rng_rate += 0.1;}
                // 左下
                if bottom_edge==0 && left_edge==0 && raw[j+width-1]!=raw[j] {rng_rate += 0.1;}
                // 下
                if bottom_edge==0 && raw[j+width]!=raw[j] {rng_rate += 0.1;}
                // 右下
                if bottom_edge==0 && right_edge==0 && raw[j+width+1]!=raw[j] {rng_rate += 0.1;}

                if rng_rate > 0.4 {
                    if raw[j] == 0 {
                        raw[j] = 1;
                    } else {
                        raw[j] = 0;
                    }
                }
            }
        }
        return raw;
    }
}

impl std::default::Default for NewTerrain {
    fn default() -> Self {

        let mut rng = rand::thread_rng();
        let width = rng.gen_range(40, 81);
        let height = rng.gen_range(40, 81);
        let raw = NewTerrain::generate(width, height);

        Self {
            content: base64::encode(&raw),
            width: width as i32,
            height: height as i32,
        }
    }
}

impl std::convert::From<Terrain> for TerrainLocal {
    fn from(model: Terrain) -> Self {
        let raw = base64::decode(&model.content).unwrap();
        Self {
            entity_id: Utc::now().timestamp(),
            model: model.clone(),

            raw: init_field!(raw),
            object_ids: init_field!(Vec::new()),
        }
    }
}

#[test]
fn create_character() {
    let new_terrain = NewTerrain::default();
    assert!(new_terrain.content.len() > 0);
    let raw = base64::decode(&new_terrain.content).unwrap();
    assert_eq!(raw.len(), (new_terrain.width * new_terrain.height) as usize);
    for i in 0..new_terrain.height {
        for j in 0..new_terrain.width {
            print!("{}", if raw[(i * new_terrain.width + j) as usize] == 0 {"."}else{"#"});
        }
        print!("\n");
    }
}
