use std::any::Any;
use crate::games::dark_souls_prepare_to_die_edition::DarkSoulsPrepareToDieEdition;
use crate::games::dark_souls_remastered::DarkSoulsRemastered;
use crate::games::dx_version::DxVersion;
use crate::games::dx_version::DxVersion::Dx9;
use crate::games::traits::buffered_event_flags::BufferedEventFlags;
use crate::games::traits::player_position::PlayerPosition;


pub trait Game
{
    fn refresh(&mut self) -> Result<(), String>;
    fn get_dx_version(&self) -> DxVersion;
    fn player_position(&mut self) -> Option<Box<&mut dyn PlayerPosition>>{ None }
    fn event_flags(&mut self) -> Option<Box<&mut dyn BufferedEventFlags>>{ None }
    fn as_any(&self) -> &dyn Any;
}
