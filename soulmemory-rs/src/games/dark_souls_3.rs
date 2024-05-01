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
use crate::games::dx_version::DxVersion;
use crate::games::traits::buffered_event_flags::{BufferedEventFlags, EventFlag};
use crate::games::game::Game;

static_detour!{ static STATIC_DETOUR_SET_EVENT_FLAG: fn(u64, u32, u8, u8); }

type FnGetEventFlag = fn(event_flag_man: u64, event_flag: u32) -> u8;

pub struct DarkSouls3
{
    process: Process,

    event_flags: Arc<Mutex<Vec<EventFlag>>>,
    event_flag_man: Pointer,
    fn_get_event_flag: FnGetEventFlag,
}

impl DarkSouls3
{
    pub fn new() -> Self
    {
        DarkSouls3
        {
            process: Process::new("darksoulsiii.exe"),

            event_flags: Arc::new(Mutex::new(Vec::new())),
            event_flag_man: Pointer::default(),
            fn_get_event_flag: |_,_|{0},
        }
    }
}

impl BufferedEventFlags for DarkSouls3
{
    fn access_flag_storage(&self) -> &Arc<Mutex<Vec<EventFlag>>>
    {
        return &self.event_flags;
    }

    fn get_event_flag_state(&self, event_flag: u32) -> bool {
        let result = (self.fn_get_event_flag)(self.event_flag_man.read_u64_rel(None), event_flag);
        return result == 1;
    }
}

impl Game for DarkSouls3
{
    fn refresh(&mut self) -> Result<(), String> {
        if !self.process.is_attached()
        {
            unsafe
            {
                self.process.refresh()?;


                self.event_flag_man = self.process.scan_rel("SprjEventFlagMan", "48 c7 05 ? ? ? ? 00 00 00 00 48 8b 7c 24 38 c7 46 54 ff ff ff ff 48 83 c4 20 5e c3", 3, 11, vec![0])?;
                //.ScanRelative("playerIns", "48 8b 0d ? ? ? ? 45 33 c0 48 8d 55 e7 e8 ? ? ? ? 0f 2f 73 70 72 0d f3 ? ? ? ? ? ? ? ? 0f 11 43 70", 3, 7)
                //.CreatePointer(out _playerIns, 0, 0x80)
                //.CreatePointer(out _sprjChrPhysicsModule, 0, 0x40, 0x28) -> position
                //_sprjChrPhysicsModule.ReadFloat(0x80),
                //_sprjChrPhysicsModule.ReadFloat(0x84),
                //_sprjChrPhysicsModule.ReadFloat(0x88)

                let set_event_flag_address = self.process.scan_abs("set_event_flag", "40 55 57 41 54 41 57 48 83 ec 58 80 b9 28 02 00 00 00 45 0f b6 f9 45 0f b6 e0 8b ea 48 8b f9", 0, Vec::new())?.get_base_address();
                let get_event_flag_address = self.process.scan_abs("get_event_flag", "40 53 48 83 ec 20 80 b9 28 02 00 00 00 8b da 74 4d", 0, Vec::new())?.get_base_address();
                self.fn_get_event_flag = mem::transmute(get_event_flag_address);

                let event_flags = Arc::clone(&self.event_flags);
                STATIC_DETOUR_SET_EVENT_FLAG.initialize(mem::transmute(set_event_flag_address), move |rdx: u64, event_flag_id: u32, value: u8, r9b: u8|
                {
                    let mut guard = event_flags.lock().unwrap();
                    guard.push(EventFlag::new(chrono::offset::Local::now(), event_flag_id, value == 1));
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
    fn event_flags(&mut self) -> Option<Box<&mut dyn BufferedEventFlags>> { Some(Box::new(self)) }

    fn as_any(&self) -> &dyn Any
    {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}