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

use std::any::Any;
use std::mem;
use std::sync::{Arc, Mutex};
use retour::static_detour;
use log::info;
use mem_rs::prelude::*;
use crate::games::traits::buffered_event_flags::{BufferedEventFlags, EventFlag};
use crate::games::dx_version::DxVersion;
use crate::games::traits::game::Game;
use crate::widgets::widget::Widget;

static_detour!{ static STATIC_DETOUR_SET_EVENT_FLAG: fn(u64, u32, i32); }

type FnGetEventFlag = fn(event_flag_man: u64, event_flag: u32) -> u8;

pub struct EldenRing
{
    process: Process,

    event_flags: Arc<Mutex<Vec<EventFlag>>>,
    virtual_memory_flag: Pointer,
    fn_get_event_flag: FnGetEventFlag,
}

impl EldenRing
{
    pub fn new() -> Self
    {
        EldenRing
        {
            process: Process::new("eldenring.exe"),

            event_flags: Arc::new(Mutex::new(Vec::new())),
            virtual_memory_flag: Pointer::default(),
            fn_get_event_flag: |_,_|{0},
        }
    }
}

impl BufferedEventFlags for EldenRing
{
    fn access_flag_storage(&self) -> &Arc<Mutex<Vec<EventFlag>>>
    {
        return &self.event_flags;
    }

    fn get_event_flag_state(&self, event_flag: u32) -> bool {
        let result = (self.fn_get_event_flag)(self.virtual_memory_flag.read_u64_rel(None), event_flag);
        return result == 1;
    }
}

impl Game for EldenRing
{
    fn refresh(&mut self) -> Result<(), String> {
        if !self.process.is_attached()
        {
            unsafe
            {
                self.process.refresh()?;

                self.virtual_memory_flag = self.process.scan_rel("VirtualMemoryFlag", "48 8b 3d ? ? ? ? 8b f3 89 5c 24 20 48 85 ff", 3, 7, vec![0])?;

                let set_event_flag_address = self.process.scan_abs("set_event_flag", "48 89 5c 24 08 44 8b 49 1c 44 8b d2 33 d2 41 8b c2 41 f7 f1 41 8b d8 4c 8b d9", 0, Vec::new())?.get_base_address();
                let get_event_flag_address = self.process.scan_abs("get_event_flag", "44 8b 41 1c 44 8b da 33 d2 41 8b c3 41 f7 f0", 0, Vec::new())?.get_base_address();
                self.fn_get_event_flag = mem::transmute(get_event_flag_address);

                let event_flags = Arc::clone(&self.event_flags);
                STATIC_DETOUR_SET_EVENT_FLAG.initialize(mem::transmute(set_event_flag_address), move |rdx: u64, event_flag_id: u32, value: i32|
                {
                    let mut guard = event_flags.lock().unwrap();
                    guard.push(EventFlag::new(chrono::offset::Local::now(), event_flag_id, value == 1));
                    STATIC_DETOUR_SET_EVENT_FLAG.call(rdx, event_flag_id, value);
                }).unwrap().enable().unwrap();

                info!("event_flag_man base address: 0x{:x}", self.virtual_memory_flag.get_base_address());
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
        DxVersion::Dx12
    }
    fn event_flags(&mut self) -> Option<Box<&mut dyn BufferedEventFlags>> { Some(Box::new(self)) }

    fn as_any(&self) -> &dyn Any
    {
        self
    }
}