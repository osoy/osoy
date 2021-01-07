use crate::Config;

pub trait Exec {
    fn exec(self, config: Config) -> i32;
}
