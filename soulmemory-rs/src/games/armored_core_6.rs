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
use crate::games::{DxVersion, EventFlag, EventFlagLogger, Game};
use crate::gui::event_flag_widget::EventFlagWidget;
use crate::gui::widget::Widget;

static_detour!{ static STATIC_DETOUR_SET_EVENT_FLAG: fn(u64, u32, i32); }

type FnGetEventFlag = fn(event_flag_man: u64, event_flag: u32) -> u8;

pub struct ArmoredCore6
{
    process: Process,

    event_flags: Arc<Mutex<Vec<EventFlag>>>,
    virtual_memory_flag: Pointer,
    fn_get_event_flag: FnGetEventFlag,
    set_event_flag_address: usize,
}

impl ArmoredCore6
{
    pub fn new() -> Self
    {
        ArmoredCore6
        {
            process: Process::new("armoredcore6.exe"),

            event_flags: Arc::new(Mutex::new(Vec::new())),
            virtual_memory_flag: Pointer::default(),
            fn_get_event_flag: |_,_|{0},
            set_event_flag_address: 0,
        }
    }
}

impl EventFlagLogger for ArmoredCore6
{
    fn get_buffered_flags(&mut self) -> Vec<EventFlag>
    {
        let mut event_flags = self.event_flags.lock().unwrap();
        mem::replace(&mut event_flags, Vec::new())
    }

    fn get_event_flag_state(&self, event_flag: u32) -> bool {
        let result = (self.fn_get_event_flag)(self.virtual_memory_flag.read_u64_rel(None), event_flag);
        return result == 1;
    }
}




impl Game for ArmoredCore6
{
    fn refresh(&mut self) -> Result<(), String>
    {
        unsafe
        {
            if !self.process.is_attached()
            {
                self.process.refresh()?;

                self.virtual_memory_flag = self.process.scan_rel("CSEventFlagMan", "48 8b 35 ? ? ? ? 83 f8 ff 0f 44 c1", 3, 7, vec![0])?;

                self.set_event_flag_address = self.process.scan_abs("set_event_flag", "48 89 5c 24 18 56 41 56 41 57 48 83 ec 20 44 8b 49 1c 44 8b f2", 0, Vec::new())?.get_base_address();
                let get_event_flag_address = self.process.scan_abs("get_event_flag", "44 8b 41 1c 44 8b da 33 d2 41 8b c3 41 f7 f0 4c 8b d1 45 33 c9 44 0f af c0", 0, Vec::new())?.get_base_address();
                self.fn_get_event_flag = mem::transmute(get_event_flag_address);

                let event_flags = Arc::clone(&self.event_flags);
                STATIC_DETOUR_SET_EVENT_FLAG.initialize(mem::transmute(self.set_event_flag_address), move |rdx: u64, event_flag_id: u32, value: i32|
                {
                    let mut guard = event_flags.lock().unwrap();
                    guard.push((chrono::offset::Local::now(), event_flag_id, value == 1));
                    STATIC_DETOUR_SET_EVENT_FLAG.call(rdx, event_flag_id, value);
                }).unwrap().enable().unwrap();

                info!("event_flag_man base address: 0x{:x}", self.virtual_memory_flag.get_base_address());
                info!("set event flag address     : 0x{:x}", self.set_event_flag_address);
                info!("get event flag address     : 0x{:x}", get_event_flag_address);
            }
            else
            {
                let mut buffer: [u8; 1] = [0x0];
                self.process.read_memory_abs(self.set_event_flag_address, &mut buffer);
                let byte = buffer[0];
                if byte == 0x48
                {
                    info!("re-hook set event flag");
                    STATIC_DETOUR_SET_EVENT_FLAG.disable().unwrap();
                    STATIC_DETOUR_SET_EVENT_FLAG.enable().unwrap();
                }

                self.process.refresh()?;
            }
            Ok(())
        }
    }

    fn get_dx_version(&self) -> DxVersion {
        DxVersion::Dx12
    }

    fn get_widgets(&self) -> Vec<Box<dyn Widget>> {
        vec![Box::new(EventFlagWidget::new())]
    }
}
