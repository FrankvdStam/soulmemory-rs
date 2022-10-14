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

use std::{mem};
use std::sync::{Arc, Mutex};
use detour::static_detour;
use log::info;
use mem_rs::prelude::*;
use windows::Win32::UI::Input::XboxController::XINPUT_STATE;
use crate::App;
use crate::games::{DxVersion, Game, GameEnum};
use crate::gui::event_flags::{EventFlag, EventFlagLogger, EventFlagWidget};
use crate::gui::widget::Widget;
use crate::gui::ai_toggle::AiToggle;
use crate::tas::tas::{get_xinput_get_state_fn, tas_ai_toggle};
use crate::tas::toggle_mode::ToggleMode;

static_detour!{ static STATIC_DETOUR_UPDATE_IGT: unsafe extern "C" fn(f32); }
static_detour!{ static STATIC_DETOUR_SET_EVENT_FLAG: fn(u64, u32, u8, u8); }
static_detour!{ static STATIC_DETOUR_XINPUT_GET_STATE: unsafe extern "system" fn(u32, *mut XINPUT_STATE) -> u32; }

type FnGetEventFlag = fn(event_flag_man: u64, event_flag: u32) -> u8;

pub struct DarkSoulsRemastered
{
    process: Process,

    ai_timer: Pointer,
    game_data_man: Pointer,

    event_flag_man: Pointer,
    fn_get_event_flag: FnGetEventFlag,
    event_flags: Arc<Mutex<Vec<EventFlag>>>,


    pub ai_timer_toggle_threshold: f32,
    pub ai_timer_toggle_mode: ToggleMode,
}

impl DarkSoulsRemastered
{
    pub fn new() -> Self
    {
        DarkSoulsRemastered
        {
            process: Process::new("DarkSoulsRemastered.exe"),

            ai_timer: Pointer::default(),
            game_data_man: Pointer::default(),

            event_flag_man: Pointer::default(),
            fn_get_event_flag: |_,_|{return 0},
            event_flags: Arc::new(Mutex::new(Vec::new())),

            ai_timer_toggle_threshold: 4.8f32,
            ai_timer_toggle_mode: ToggleMode::None,
        }
    }

    #[allow(dead_code)]
    pub fn get_in_game_time_milliseconds(&self) -> u32
    {
        return self.game_data_man.read_u32_rel(Some(0xa4));
    }

    pub fn get_ai_timer_value(&self) -> f32
    {
        self.ai_timer.read_f32_rel(Some(0x24))
    }
}

impl EventFlagLogger for DarkSoulsRemastered
{
    fn get_buffered_flags(&mut self) -> Vec<EventFlag>
    {
        let mut event_flags = self.event_flags.lock().unwrap();
        mem::replace(&mut event_flags, Vec::new())
    }

    fn get_event_flag_state(&self, event_flag: u32) -> bool
    {
        let event_flag_man_address = self.event_flag_man.read_u32_rel(None) as u64; //Bit memes because DSR is 64bit, compiled with 32bit wide pointers
        let result = (self.fn_get_event_flag)(event_flag_man_address, event_flag);
        return result == 1;
    }
}

impl Game for DarkSoulsRemastered
{
    fn refresh(&mut self) -> Result<(), String>
    {
        if !self.process.is_attached()
        {
            unsafe
            {
                self.process.refresh()?;
                self.game_data_man  = self.process.scan_rel("GameDataMan", "48 8b 05 ? ? ? ? 48 8b 50 10 48 89 54 24 60", 3, 7, vec![0])?;
                self.ai_timer       = self.process.scan_rel("ai timer", "48 8b 0d ? ? ? ? 48 85 c9 74 0e 48 83 c1 28", 3, 7, vec![0])?;
                self.event_flag_man = self.process.scan_rel("event flags", "48 8B 0D ? ? ? ? 99 33 C2 45 33 C0 2B C2 8D 50 F6", 3, 7, vec![0])?;



                let update_igt_address     = self.process.scan_abs("update_igt", "40 57 48 83 ec 40 48 c7 44 24 20 fe ff ff ff 48 89 5c 24 50 0f 29 74 24 30 0f 28 f0", 0, Vec::new())?.get_base_address();
                let set_event_flag_address = self.process.scan_abs("set_event_flag", "48 89 5c 24 08 57 48 83 ec 20 80 b9 24 02 00 00 00 41 0f b6 f8", 0, Vec::new())?.get_base_address();
                let get_event_flag_address = self.process.scan_abs("get_event_flag", "40 53 48 83 ec 20 80 b9 24 02 00 00 00 8b da 74 4d", 0, Vec::new())?.get_base_address();
                self.fn_get_event_flag = mem::transmute(get_event_flag_address);

                STATIC_DETOUR_UPDATE_IGT.initialize(mem::transmute(update_igt_address), detour_update_igt).unwrap().enable().unwrap();

                let event_flags = Arc::clone(&self.event_flags);
                STATIC_DETOUR_SET_EVENT_FLAG.initialize(mem::transmute(set_event_flag_address), move |rdx: u64, event_flag_id: u32, value: u8, r9b: u8|
                {
                    let mut guard = event_flags.lock().unwrap();
                    guard.push((chrono::offset::Local::now(), event_flag_id, value == 1));
                    STATIC_DETOUR_SET_EVENT_FLAG.call(rdx, event_flag_id, value, r9b);
                }).unwrap().enable().unwrap();

                STATIC_DETOUR_XINPUT_GET_STATE.initialize(get_xinput_get_state_fn(), detour_xinput_get_state).unwrap().enable().unwrap();

                info!("game_data_man base address : 0x{:x}", self.game_data_man.get_base_address());
                info!("ai_timer base address      : 0x{:x}", self.ai_timer.get_base_address());
                info!("event_flag_man base address: 0x{:x}", self.event_flag_man.get_base_address());
                info!("update igt address         : 0x{:x}", update_igt_address);
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

    fn get_widgets(&self) -> Vec<Box<dyn Widget>>
    {
        vec![Box::new(EventFlagWidget::new()), Box::new(AiToggle::new())]
    }
}

fn detour_xinput_get_state(dw_user_index: u32, xinput_state: *mut XINPUT_STATE) -> u32
{
    let instance = App::get_instance();
    let app = instance.lock().unwrap();

    if let GameEnum::DarkSoulsRemastered(dsr) = &app.game
    {
        let res = unsafe{ STATIC_DETOUR_XINPUT_GET_STATE.call(dw_user_index, xinput_state) };

        tas_ai_toggle(dsr.ai_timer_toggle_mode, dsr.get_ai_timer_value(), dsr.ai_timer_toggle_threshold, xinput_state);
        return res;
    }
    else
    {
        panic!("remastered detour alled but game instance ")
    }
}


pub fn detour_update_igt(delta: f32)
{
    unsafe{ STATIC_DETOUR_UPDATE_IGT.call(delta) };
}
