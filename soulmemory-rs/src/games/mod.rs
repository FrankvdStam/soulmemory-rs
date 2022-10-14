// This file is part of the soulmemory-rs distribution (https://github.com/FrankvdStam/soulmemory-rs).
// Copyright (c) 2022 Frank van der Stam.
// https://github.com/FrankvdStam/soulmemory-rs/blob/main/LICENSE
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use crate::games::dark_souls_3::DarkSouls3;
use crate::games::prepare_to_die_edition::DarkSoulsPrepareToDieEdition;
use crate::games::remastered::DarkSoulsRemastered;
use crate::gui::event_flags::{EventFlag, EventFlagLogger};
use crate::gui::widget::Widget;

pub mod remastered;
pub mod prepare_to_die_edition;
pub mod dark_souls_3;

#[allow(dead_code)]
pub enum DxVersion
{
    Dx9,
    Dx11,
    Dx12,
}

pub trait Game : EventFlagLogger
{
    fn refresh(&mut self) -> Result<(), String>;
    fn get_dx_version(&self) -> DxVersion;
    fn get_widgets(&self) -> Vec<Box<dyn Widget>>;
}

pub enum GameEnum
{
    DarkSoulsPrepareToDieEdition(DarkSoulsPrepareToDieEdition),
    DarkSoulsRemastered(DarkSoulsRemastered),
    DarkSouls3(DarkSouls3),
}



impl Game for GameEnum
{
    fn refresh(&mut self) -> Result<(), String> {
        match self
        {
            GameEnum::DarkSoulsPrepareToDieEdition(ptde) => ptde.refresh(),
            GameEnum::DarkSoulsRemastered(remastered) => remastered.refresh(),
            GameEnum::DarkSouls3(ds3) => ds3.refresh(),
        }
    }

    fn get_dx_version(&self) -> DxVersion {
        match self
        {
            GameEnum::DarkSoulsPrepareToDieEdition(ptde) => ptde.get_dx_version(),
            GameEnum::DarkSoulsRemastered(remastered) => remastered.get_dx_version(),
            GameEnum::DarkSouls3(ds3) => ds3.get_dx_version(),
        }
    }

    fn get_widgets(&self) -> Vec<Box<dyn Widget>> {
        match self
        {
            GameEnum::DarkSoulsPrepareToDieEdition(ptde) => ptde.get_widgets(),
            GameEnum::DarkSoulsRemastered(remastered) => remastered.get_widgets(),
            GameEnum::DarkSouls3(ds3) => ds3.get_widgets(),
        }
    }
}

impl EventFlagLogger for GameEnum {
    fn get_buffered_flags(&mut self) -> Vec<EventFlag> {
        match self
        {
            GameEnum::DarkSoulsPrepareToDieEdition(ptde) => ptde.get_buffered_flags(),
            GameEnum::DarkSoulsRemastered(remastered) => remastered.get_buffered_flags(),
            GameEnum::DarkSouls3(ds3) => ds3.get_buffered_flags(),
        }
    }

    fn get_event_flag_state(&self, event_flag: u32) -> bool {
        match self
        {
            GameEnum::DarkSoulsPrepareToDieEdition(ptde) => ptde.get_event_flag_state(event_flag),
            GameEnum::DarkSoulsRemastered(remastered) => remastered.get_event_flag_state(event_flag),
            GameEnum::DarkSouls3(ds3) => ds3.get_event_flag_state(event_flag),
        }
    }
}