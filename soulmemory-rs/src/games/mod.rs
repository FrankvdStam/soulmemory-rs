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

use std::fmt;
use std::fmt::Display;
use chrono::{DateTime, Local};
use crate::games::armored_core_6::ArmoredCore6;
use crate::games::dark_souls_3::DarkSouls3;
use crate::games::elden_ring::EldenRing;
use crate::games::prepare_to_die_edition::DarkSoulsPrepareToDieEdition;
use crate::games::remastered::DarkSoulsRemastered;
use crate::games::sekiro::{Sekiro};
use crate::gui::widget::Widget;
use crate::util::vector3f::Vector3f;

pub mod remastered;
pub mod prepare_to_die_edition;
pub mod dark_souls_3;
pub mod sekiro;
pub mod elden_ring;
pub mod armored_core_6;

#[allow(dead_code)]
pub enum DxVersion
{
    Dx9,
    Dx11,
    Dx12,
}

#[derive(Clone, Copy)]
pub struct  EventFlag
{
    pub time: DateTime<Local>,
    pub flag: u32,
    pub state: bool,
}

impl Display for EventFlag
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {: >10} - {}", self.time.format("%Y-%m-%d %H:%M:%S%.3f"), self.flag, self.state)
    }
}

impl EventFlag
{
    pub fn new(time: DateTime<Local>, flag: u32, state: bool,) -> Self {EventFlag { time, flag, state } }
}


pub type ChrDbgFlag = (u32, String, bool);

pub trait EventFlagLogger
{
    fn get_buffered_flags(&mut self) -> Vec<EventFlag>;
    fn get_event_flag_state(&self, event_flag: u32) -> bool;
}

pub trait BasicPlayerPosition
{
    fn get_position(&self) -> Vector3f;
    fn set_position(&self, position: &Vector3f);
}

pub trait GetSetChrDbgFlags
{
    fn get_flags(&self) -> Vec<ChrDbgFlag>;
    fn set_flag(&self, flag: u32, value: bool);
}

pub trait Game
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
    Sekiro(Sekiro),
    EldenRing(EldenRing),
    ArmoredCore6(ArmoredCore6),
}

//impl GameEnum
//{
//    pub(crate) fn borrow_sekiro(&self) -> &Sekiro
//    {
//        match self
//        {
//            GameEnum::Sekiro(sekiro) => sekiro,
//            _ => panic!("attempt to borrow sekiro")
//        }
//    }
//}

impl Game for GameEnum
{
    fn refresh(&mut self) -> Result<(), String> {
        match self
        {
            GameEnum::DarkSoulsPrepareToDieEdition(ptde) => ptde.refresh(),
            GameEnum::DarkSoulsRemastered(remastered)    => remastered.refresh(),
            GameEnum::DarkSouls3(ds3)                    => ds3.refresh(),
            GameEnum::Sekiro(sekiro)                     => sekiro.refresh(),
            GameEnum::EldenRing(elden_ring)              => elden_ring.refresh(),
            GameEnum::ArmoredCore6(armored_core_6)       => armored_core_6.refresh(),
        }
    }

    fn get_dx_version(&self) -> DxVersion {
        match self
        {
            GameEnum::DarkSoulsPrepareToDieEdition(ptde) => ptde.get_dx_version(),
            GameEnum::DarkSoulsRemastered(remastered)    => remastered.get_dx_version(),
            GameEnum::DarkSouls3(ds3)                    => ds3.get_dx_version(),
            GameEnum::Sekiro(sekiro)                     => sekiro.get_dx_version(),
            GameEnum::EldenRing(elden_ring)              => elden_ring.get_dx_version(),
            GameEnum::ArmoredCore6(armored_core_6)       => armored_core_6.get_dx_version(),
        }
    }

    fn get_widgets(&self) -> Vec<Box<dyn Widget>> {
        match self
        {
            GameEnum::DarkSoulsPrepareToDieEdition(ptde) => ptde.get_widgets(),
            GameEnum::DarkSoulsRemastered(remastered)    => remastered.get_widgets(),
            GameEnum::DarkSouls3(ds3)                    => ds3.get_widgets(),
            GameEnum::Sekiro(sekiro)                     => sekiro.get_widgets(),
            GameEnum::EldenRing(elden_ring)              => elden_ring.get_widgets(),
            GameEnum::ArmoredCore6(armored_core_6)       => armored_core_6.get_widgets(),
        }
    }
}

impl EventFlagLogger for GameEnum {
    fn get_buffered_flags(&mut self) -> Vec<EventFlag> {
        match self
        {
            GameEnum::DarkSoulsPrepareToDieEdition(ptde) => ptde.get_buffered_flags(),
            GameEnum::DarkSoulsRemastered(remastered)    => remastered.get_buffered_flags(),
            GameEnum::DarkSouls3(ds3)                    => ds3.get_buffered_flags(),
            GameEnum::Sekiro(sekiro)                     => sekiro.get_buffered_flags(),
            GameEnum::EldenRing(elden_ring)              => elden_ring.get_buffered_flags(),
            GameEnum::ArmoredCore6(armored_core_6)       => armored_core_6.get_buffered_flags(),
        }
    }

    fn get_event_flag_state(&self, event_flag: u32) -> bool {
        match self
        {
            GameEnum::DarkSoulsPrepareToDieEdition(ptde) => ptde.get_event_flag_state(event_flag),
            GameEnum::DarkSoulsRemastered(remastered)    => remastered.get_event_flag_state(event_flag),
            GameEnum::DarkSouls3(ds3)                    => ds3.get_event_flag_state(event_flag),
            GameEnum::Sekiro(sekiro)                     => sekiro.get_event_flag_state(event_flag),
            GameEnum::EldenRing(elden_ring)              => elden_ring.get_event_flag_state(event_flag),
            GameEnum::ArmoredCore6(armored_core_6)       => armored_core_6.get_event_flag_state(event_flag),
        }
    }
}

impl BasicPlayerPosition for GameEnum
{
    fn get_position(&self) -> Vector3f {
        match self
        {
            GameEnum::DarkSoulsPrepareToDieEdition(_)    => panic!("BasicPlayerPosition not available in PTDE"),
            GameEnum::DarkSoulsRemastered(_)             => panic!("BasicPlayerPosition not available in remastered"),
            GameEnum::DarkSouls3(_)                      => panic!("BasicPlayerPosition not available in ds3"),
            GameEnum::Sekiro(sekiro)                     => sekiro.get_position(),
            GameEnum::EldenRing(_)                       => panic!("BasicPlayerPosition not available in elden ring"),
            GameEnum::ArmoredCore6(_)                    => panic!("BasicPlayerPosition not available in elden ring"),
        }
    }

    fn set_position(&self, position: &Vector3f) {
        match self
        {
            GameEnum::DarkSoulsPrepareToDieEdition(_)    => panic!("BasicPlayerPosition not available in PTDE"),
            GameEnum::DarkSoulsRemastered(_)             => panic!("BasicPlayerPosition not available in remastered"),
            GameEnum::DarkSouls3(_)                      => panic!("BasicPlayerPosition not available in ds3"),
            GameEnum::Sekiro(sekiro)                     => sekiro.set_position(position),
            GameEnum::EldenRing(_)                       => panic!("BasicPlayerPosition not available in elden ring"),
            GameEnum::ArmoredCore6(_)                    => panic!("BasicPlayerPosition not available in elden ring"),
        }
    }
}

impl GetSetChrDbgFlags for GameEnum
{
    fn get_flags(&self) -> Vec<ChrDbgFlag> {
        match self
        {
            GameEnum::Sekiro(sekiro) => sekiro.get_flags(),
            _ => panic!("GetSetChrDbgFlags not supported."),
        }
    }

    fn set_flag(&self, flag: u32, value: bool) {
        match self
        {
            GameEnum::Sekiro(sekiro) => sekiro.set_flag(flag, value),
            _ => panic!("GetSetChrDbgFlags not supported."),
        }
    }
}
