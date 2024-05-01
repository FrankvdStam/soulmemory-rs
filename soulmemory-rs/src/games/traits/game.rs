use std::any::Any;
use crate::games::dx_version::DxVersion;
use crate::games::traits::buffered_event_flags::BufferedEventFlags;
use crate::games::traits::player_position::PlayerPosition;


pub trait Game
{
    fn refresh(&mut self) -> Result<(), String>;
    fn get_dx_version(&self) -> DxVersion;
    fn player_position(&mut self) -> Option<Box<&mut dyn PlayerPosition>>{ None }
    fn event_flags(&mut self) -> Option<Box<&mut dyn BufferedEventFlags>>{ None }
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct GameExt;

impl GameExt
{
    //pub fn get_game_ref<T: 'static>(game: &dyn Game) -> Option<&T>
    //{
    //    return game.as_any().downcast_ref::<T>();
    //}

    pub fn get_game_mut<T: 'static>(game: &mut dyn Game) -> Option<&mut T>
    {
        return game.as_any_mut().downcast_mut::<T>();
    }
}
