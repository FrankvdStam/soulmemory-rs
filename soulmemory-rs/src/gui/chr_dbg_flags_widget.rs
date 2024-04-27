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

use imgui::{TreeNodeFlags, Ui};
use crate::games::{GameEnum, GetSetChrDbgFlags};
use crate::gui::widget::Widget;

pub struct ChrDbgFlagsWidget
{
    flags: Vec<(u32, String, bool)>,
    init: bool,
}

impl ChrDbgFlagsWidget
{
    pub fn new() -> Self{ ChrDbgFlagsWidget { flags: Vec::new(), init: false} }
}

impl Widget for ChrDbgFlagsWidget
{
    fn render(&mut self, game: &mut GameEnum, ui: &Ui)
    {
        if !self.init
        {
            self.flags = game.get_flags();
            self.init = true;
        }

        if ui.collapsing_header("chr dbg", TreeNodeFlags::FRAMED)
        {
            for f in self.flags.iter_mut()
            {

                if ui.checkbox(&f.1, &mut f.2)
                {
                    game.set_flag(f.0, f.2);
                }
            }
        }
    }
}
