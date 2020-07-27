use crate::{Context, Error};

pub trait Game {
    fn initialize(&mut self, context: &mut Context) -> Result<(), Error>;
    fn update(&mut self, context: &mut Context) -> Result<(), Error>;
    fn draw(&mut self, context: &mut Context) -> Result<(), Error>;
}
