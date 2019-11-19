#[macro_export]
macro_rules! define_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident { $($variant:ident = $val:expr,)* }
    ) => {
        use diesel::sql_types::Integer;
        use diesel::serialize::ToSql;
        use diesel::deserialize::FromSql;

        // 元の enum を必要な derive とともに定義
        $(#[$meta])*
        #[derive(FromSqlRow, AsExpression)]
        #[sql_type = "Integer"]
        pub enum $name {
            $($variant = $val,)*
        }

        // `ToSql`を定義
        impl<DB: diesel::backend::Backend> ToSql<Integer, DB> for $name {
            fn to_sql<W: std::io::Write>(
                &self,
                out: &mut diesel::serialize::Output<W, DB>,
            ) -> Result<diesel::serialize::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                ToSql::<Integer, DB>::to_sql(&(*self as i32), out)
            }
        }

        // `FromSql`を定義
        impl<DB: diesel::backend::Backend> FromSql<Integer, DB> for $name
        where
            i32: FromSql<Integer, DB>,
        {
            fn from_sql(
                bytes: Option<&DB::RawValue>,
            ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                use self::$name::*;

                match <i32 as FromSql<Integer, DB>>::from_sql(bytes)? {
                    $($val => Ok($variant),)*
                    s => Err(format!("invalid {} value: {}", stringify!($name), s).into()),
                }
            }
        }
    }
}

use rand::{seq::IteratorRandom, thread_rng, Rng};
static KATAKANAS: &str =
    "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン";

pub fn generate_random_name(len: usize) -> String {
    let mut rng = thread_rng();
    let len = if len == 0 { rng.gen_range(2, 8) } else { len };
    KATAKANAS
        .chars()
        .choose_multiple(&mut rng, len)
        .into_iter()
        .collect()
}

#[macro_export]
macro_rules! dbg {
    () => (eprint!("\x1b[1;33m[DEBUG  ]\x1b[0m \n"));
    ($fmt:expr) => (eprint!(concat!("\x1b[1;33m[DEBUG  ]\x1b[0;33m ", $fmt, "\x1b[0m\n")));
    ($fmt:expr, $($arg:tt)*) => (eprint!(concat!("\x1b[1;33m[DEBUG  ]\x1b[0;33m ", $fmt, "\x1b[0m\n"), $($arg)*));
}

#[macro_export]
macro_rules! err {
    () => (eprint!("\x1b[1;31m[ERROR  ]\x1b[0m \n"));
    ($fmt:expr) => (eprint!(concat!("\x1b[1;31m[ERROR  ]\x1b[0;31m ", $fmt, "\x1b[0m\n")));
    ($fmt:expr, $($arg:tt)*) => (eprint!(concat!("\x1b[1;31m[ERROR  ]\x1b[0;31m ", $fmt, "\x1b[0m\n"), $($arg)*));
}

#[macro_export]
macro_rules! log {
    ($placeholder:expr) => (eprint!("\x1b[1;32m[{:7}]\x1b[0m \n", $placeholder));
    ($placeholder:expr, $fmt:expr) => (eprint!(concat!("\x1b[1;32m[{:7}]\x1b[0;32m ", $fmt, "\x1b[0m\n"), $placeholder));
    ($placeholder:expr, $fmt:expr, $($arg:tt)*) => (eprint!(concat!("\x1b[1;32m[{:7}]\x1b[0;32m ", $fmt, "\x1b[0m\n"), $placeholder, $($arg)*));
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

impl Vec2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normal(&self) -> Self {
        let len = self.len();
        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }

    pub fn inner(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Self) -> f32 {
        self.x * other.y - other.x * self.y
    }
}

impl std::ops::Sub for Vec2D {
    type Output = Vec2D;
    fn sub(self, right: Vec2D) -> Self::Output {
        Vec2D {
            x: right.x - self.x,
            y: right.y - self.y,
        }
    }
}

pub fn intersects_circle(x0: f32, y0: f32, r0: f32, x1: f32, y1: f32, r1: f32) -> bool {
    let d = ((x0 - x1) * (x0 - x1) + (y0 - y1) * (y0 - y1)).sqrt();
    d < r0 + r1
}

pub fn intersects_circle_with_line(
    x0: f32,
    y0: f32,
    r0: f32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
) -> Option<f32> {
    let p = Vec2D::new(x0, y0);
    let a = Vec2D::new(x1, y1);
    let b = Vec2D::new(x2, y2);

    let v_ab = b - a;
    let v_ap = p - a;

    let n1 = v_ab.inner(&v_ap);

    if n1 < 0.0 {
        return if v_ap.len() < r0 {
            Some(v_ap.len() - r0)
        } else {
            None
        };
    }

    let n2 = v_ab.inner(&v_ab);

    if n1 > n2 {
        let len = (p - b).len().powi(2);
        return if len < r0.powi(2) {
            Some(r0.powi(2) - len)
        } else {
            None
        };
    } else {
        let n3 = v_ap.inner(&v_ap);
        return if n3 - (n1 / n2) * n1 < r0.powi(2) {
            Some(r0.powi(2) - (n3 - (n1 / n2) * n1))
        } else {
            None
        };
    }
}
