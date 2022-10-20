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
use mem_rs::pointer::Pointer;
use mem_rs::prelude::{Process, ReadWrite};
use crate::games::{DxVersion, Game};
use crate::gui::event_flags::{EventFlag, EventFlagLogger, EventFlagWidget};
use crate::gui::widget::Widget;

static_detour!{ static STATIC_DETOUR_SET_EVENT_FLAG: extern "thiscall" fn(u32, u32, u8); }
static_detour!{ static STATIC_DETOUR_GET_EVENT_FLAG: extern "thiscall" fn(u32, u32) -> u8; }

//type FnGetEventFlag = extern "thiscall" fn(event_flag_man: u32, event_flag: u32) -> u8;

pub struct DarkSoulsPrepareToDieEdition
{
    process: Process,
    event_flag_man: Pointer,
    event_flags: Arc<Mutex<Vec<EventFlag>>>,
}

impl DarkSoulsPrepareToDieEdition
{
    pub fn new() -> Self
    {
        DarkSoulsPrepareToDieEdition
        {
            process: Process::new("darksouls.exe"),

            event_flag_man: Pointer::default(),
            event_flags: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl EventFlagLogger for DarkSoulsPrepareToDieEdition
{
    fn get_buffered_flags(&mut self) -> Vec<EventFlag>
    {
        let mut event_flags = self.event_flags.lock().unwrap();
        mem::replace(&mut event_flags, Vec::new())
    }

    fn get_event_flag_state(&self, event_flag: u32) -> bool
    {
        let flag = STATIC_DETOUR_GET_EVENT_FLAG.call(self.event_flag_man.read_u32_rel(None), event_flag);
        return flag == 1;
    }
}


impl Game for DarkSoulsPrepareToDieEdition
{
    fn refresh(&mut self) -> Result<(), String>
    {
        if !self.process.is_attached()
        {
            unsafe
            {
                self.process.refresh()?;
                self.event_flag_man = self.process.scan_abs("event flags", "56 8B F1 8B 46 1C 50 A1 ? ? ? ? 32 C9", 8, vec![0, 0])?;

                let set_event_flag_address = self.process.scan_abs("set_event_flag", "80 b8 14 01 00 00 00 56 8b 74 24 08 74 ? 57 51 50", 0, Vec::new())?.get_base_address();
                let get_event_flag_address = self.process.scan_abs("set_event_flag", "53 32 db 56 8b 74 24 0c 38 98 14 01 00 00", 0, Vec::new())?.get_base_address();

                //thiscall calling convention is experimental - seems like transmuting a thiscall function to an fn instance doesn't work just yet.
                //thats why I detour the event flag read function even though I don't want to change game behavior - this gives me a detour object I can call() as if it was a fn
                STATIC_DETOUR_GET_EVENT_FLAG.initialize(mem::transmute(get_event_flag_address), |event_flag_man: u32, event_flag: u32|{
                    STATIC_DETOUR_GET_EVENT_FLAG.call(event_flag_man, event_flag)
                }).unwrap().enable().unwrap();

                let event_flags = Arc::clone(&self.event_flags);
                STATIC_DETOUR_SET_EVENT_FLAG.initialize(mem::transmute(set_event_flag_address), move |event_flag_man: u32, event_flag: u32, value: u8|{
                    info!("{} {} {}", event_flag_man, event_flag, value);
                    let mut guard = event_flags.lock().unwrap();
                    guard.push((chrono::offset::Local::now(), event_flag, value == 1));
                    STATIC_DETOUR_SET_EVENT_FLAG.call(event_flag_man, event_flag, value)
                }).unwrap().enable().unwrap();

        //        let event_flags = Arc::clone(&self.event_flags);
        //        //STATIC_DETOUR_SET_EVENT_FLAG.initialize(mem::transmute(set_event_flag_address), move |event_flag_man: u32, event_flag_id: u32, value: u8|
        //        //{
        //        //    let mut guard = event_flags.lock().unwrap();
        //        //    guard.push((chrono::offset::Local::now(), event_flag_id, value == 1));
        //        //    STATIC_DETOUR_SET_EVENT_FLAG.call(event_flag_man, event_flag_id, value);
        //        //}).unwrap().enable().unwrap();

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
        DxVersion::Dx9
    }

    fn get_widgets(&self) -> Vec<Box<dyn Widget>> {
        vec![Box::new(EventFlagWidget::new())]
    }
}

