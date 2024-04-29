use std::any::Any;
use crate::games::dx_version::DxVersion;
use crate::games::traits::buffered_event_flags::BufferedEventFlags;
use crate::games::traits::game::Game;

pub struct MockGame
{

}

impl MockGame
{
    pub fn new() -> Self
    {
        MockGame{}
    }

}

impl Game for MockGame
{
    fn refresh(&mut self) -> Result<(), String> {
        todo!()
    }

    fn get_dx_version(&self) -> DxVersion {
        todo!()
    }
    fn event_flags(&mut self) -> Option<Box<&mut dyn BufferedEventFlags>> { None }

    fn as_any(&self) -> &dyn Any
    {
        self
    }
}