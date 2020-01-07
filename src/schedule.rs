use crate::batches::hunger::hunger;
use crate::batches::charge_down::charge_down;
use crate::context::Context;
use std::time::Instant;

type ResultType = Result<Vec<u64>, String>;

pub struct Schedule {
    pub func: fn(&mut Context) -> ResultType,
    pub msec: u128,
    timer: Instant,
}

impl Schedule {
    fn new(func: fn(&mut Context) -> ResultType, msec: u128) -> Self {
        Self {
            func,
            msec,
            timer: Instant::now(),
        }
    }
    pub fn exec(&mut self, context: &mut Context) -> Option<ResultType> {
        let elapsed = self.timer.elapsed().as_millis();
        if elapsed > self.msec {
            self.timer = Instant::now();
            Some((self.func)(context))
        } else {
            None
        }
    }
}

pub fn make_schedules() -> Vec<Schedule> {
    vec![
        //Schedule::new(hunger, 50)
        Schedule::new(charge_down, 10)
        ]
}
