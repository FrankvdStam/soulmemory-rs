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

use std::mem;
use std::sync::{Arc, Mutex};
use detour::static_detour;
use log::info;
use mem_rs::prelude::*;
use crate::games::{DxVersion, Game};
use crate::gui::event_flags::{EventFlag, EventFlagLogger, EventFlagWidget};
use crate::gui::widget::Widget;

static_detour!{ static STATIC_DETOUR_SET_EVENT_FLAG: fn(u64, u32, u8, u8); }

type FnGetEventFlag = fn(event_flag_man: u64, event_flag: u32) -> u8;

pub struct Sekiro
{
    process: Process,

    event_flags: Arc<Mutex<Vec<EventFlag>>>,
    event_flag_man: Pointer,
    fn_get_event_flag: FnGetEventFlag,
}

impl Sekiro
{
    pub fn new() -> Self
    {
        Sekiro
        {
            process: Process::new("sekiro.exe"),

            event_flags: Arc::new(Mutex::new(Vec::new())),
            event_flag_man: Pointer::default(),
            fn_get_event_flag: |_,_|{0},
        }
    }
}

impl EventFlagLogger for Sekiro
{
    fn get_buffered_flags(&mut self) -> Vec<EventFlag>
    {
        let mut event_flags = self.event_flags.lock().unwrap();
        mem::replace(&mut event_flags, Vec::new())
    }

    fn get_event_flag_state(&self, event_flag: u32) -> bool {
        let result = (self.fn_get_event_flag)(self.event_flag_man.read_u64_rel(None), event_flag);
        return result == 1;
    }
}

impl Game for Sekiro
{
    fn refresh(&mut self) -> Result<(), String> {
        if !self.process.is_attached()
        {
            unsafe
                {
                    self.process.refresh()?;

                    self.event_flag_man = self.process.scan_rel("SprjEventFlagMan", "48 8b 0d ? ? ? ? 48 89 5c 24 50 48 89 6c 24 58 48 89 74 24 60", 3, 7, vec![0])?;

                    let set_event_flag_address = self.process.scan_abs("set_event_flag", "40 55 41 54 41 55 41 56 48 83 ec 58 80 b9 28 02 00 00 00 45 0f b6 e1 45 0f b6 e8 44 8b f2 48 8b e9", 0, Vec::new())?.get_base_address();
                    let get_event_flag_address = self.process.scan_abs("get_event_flag", "40 53 48 83 ec 20 80 b9 28 02 00 00 00 8b da", 0, Vec::new())?.get_base_address();
                    self.fn_get_event_flag = mem::transmute(get_event_flag_address);

                    let event_flags = Arc::clone(&self.event_flags);
                    STATIC_DETOUR_SET_EVENT_FLAG.initialize(mem::transmute(set_event_flag_address), move |rdx: u64, event_flag_id: u32, value: u8, r9b: u8|
                    {
                        let mut guard = event_flags.lock().unwrap();
                        guard.push((chrono::offset::Local::now(), event_flag_id, value == 1));
                        STATIC_DETOUR_SET_EVENT_FLAG.call(rdx, event_flag_id, value, r9b);
                    }).unwrap().enable().unwrap();

                    info!("event_flag_man base address: 0x{:x}", self.event_flag_man.get_base_address());
                    info!("set event flag address     : 0x{:x}", set_event_flag_address);
                    info!("get event flag address     : 0x{:x}", get_event_flag_address);
                }
        }
        else
        {
            self.process.refresh()?;
        }
        Ok(())
    }

    fn get_dx_version(&self) -> DxVersion {
        DxVersion::Dx11
    }

    fn get_widgets(&self) -> Vec<Box<dyn Widget>> {
        vec![Box::new(EventFlagWidget::new())]
    }
}