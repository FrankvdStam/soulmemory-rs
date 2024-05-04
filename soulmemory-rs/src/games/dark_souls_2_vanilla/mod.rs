use std::any::Any;
use crate::games::dx_version::DxVersion;
use crate::games::Game;

pub struct DarkSouls2Vanilla
{

}

impl DarkSouls2Vanilla
{
    pub fn new() -> Self
    {
        DarkSouls2Vanilla{}
    }
}

impl Game for DarkSouls2Vanilla
{
    fn refresh(&mut self) -> Result<(), String> {
        todo!()
    }

    fn get_dx_version(&self) -> DxVersion { DxVersion::Dx11 }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}