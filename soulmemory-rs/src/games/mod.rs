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

use std::fmt::Display;
use crate::widgets::widget::Widget;
use crate::util::vector3f::Vector3f;

pub mod traits;
pub mod dark_souls_remastered;
pub mod dark_souls_prepare_to_die_edition;
pub mod dark_souls_3;
pub mod sekiro;
pub mod elden_ring;
pub mod armored_core_6;
pub mod mock_game;
pub mod dx_version;




pub type ChrDbgFlag = (u32, String, bool);

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
