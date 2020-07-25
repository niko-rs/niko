use crate::{Context, Error};

pub trait Game {
    fn update(&mut self, context: &mut Context) -> Result<(), Error>;
    fn draw(&mut self, context: &mut Context) -> Result<(), Error>;
}
