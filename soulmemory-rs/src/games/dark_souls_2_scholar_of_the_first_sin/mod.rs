mod buffered_event_flags;

use std::any::Any;
use std::mem;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use ilhook::x64::{CallbackOption, Hooker, HookFlags, HookPoint, HookType, Registers};
use log::info;
use mem_rs::pointer::Pointer;
use mem_rs::prelude::Process;
use crate::App;
use crate::games::dx_version::DxVersion;
use crate::games::{Game, GameExt};
use crate::games::traits::buffered_event_flags::{BufferedEventFlags, EventFlag};

type FnGetEventFlag = fn(event_flag_man: u64, event_flag: u32) -> u8;

pub struct DarkSouls2ScholarOfTheFirstSin
{
    process: Process,

    event_flag_man: Pointer,
    event_flags: Arc<Mutex<Vec<EventFlag>>>,
    set_event_flag_hook: Option<HookPoint>,
    fn_get_event_flag: FnGetEventFlag,
}

impl DarkSouls2ScholarOfTheFirstSin
{
    pub fn new() -> Self
    {
        DarkSouls2ScholarOfTheFirstSin
        {
            process: Process::new("darksoulsii.exe"),

            event_flag_man: Default::default(),
            event_flags: Arc::new(Mutex::new(vec![])),
            set_event_flag_hook: None,
            fn_get_event_flag: |_,_|{return 0},
        }
    }
}

impl Game for DarkSouls2ScholarOfTheFirstSin
{
    #[cfg(target_pointer_width = "32")]
    fn refresh(&mut self) -> Result<(), String> { unimplemented!("DarkSouls2ScholarOfTheFirstSin is only available for x64"); }

    #[cfg(target_pointer_width = "64")]
    fn refresh(&mut self) -> Result<(), String>
    {

        if !self.process.is_attached()
        {
            unsafe
            {
                self.process.refresh()?;
                self.event_flag_man = self.process.scan_rel("GameDataMan" , "48 8b 35 ? ? ? ? 48 8b e9 48 85 f6", 3, 7, vec![0, 0x70, 0x20])?;
                let get_event_flag_address = self.process.scan_abs("get_event_flag" , "44 8b d2 b8 ? ? ? ? f7 e2 44 8b ca", 0,  Vec::new())?.get_base_address();
                let set_event_flag_address = self.process.scan_abs("set_event_flag" , "48 89 74 24 10 57 48 83 ec 20 8b fa 45 0f b6 d8", 0,  Vec::new())?.get_base_address();

                self.fn_get_event_flag = mem::transmute(get_event_flag_address);
                unsafe extern "win64" fn read_event_flag_hook_fn(registers: *mut Registers, _:usize)
                {
                    let instance = App::get_instance();
                    let app = instance.lock().unwrap();

                    if let Some(scholar) = GameExt::get_game_ref::<DarkSouls2ScholarOfTheFirstSin>(app.game.deref())
                    {
                        let event_flag_id = (*registers).rdx as u32;
                        let value = (*registers).r8 as u8;

                        let mut guard = scholar.event_flags.lock().unwrap();
                        guard.push(EventFlag::new(chrono::offset::Local::now(), event_flag_id, value != 0));
                    }
                }

                let h = Hooker::new(set_event_flag_address, HookType::JmpBack(read_event_flag_hook_fn), CallbackOption::None, 0, HookFlags::empty());
                self.set_event_flag_hook = Some(h.hook().unwrap());

                info!("event_flag_man base address: 0x{:x}", self.event_flag_man.get_base_address());
                info!("get event flag address     : 0x{:x}", get_event_flag_address);
                info!("get event flag address     : 0x{:x}", get_event_flag_address);
            }
        }
        else
        {
            self.process.refresh()?;
        }
        Ok(())
    }

    fn event_flags(&mut self) -> Option<Box<&mut dyn BufferedEventFlags>> { Some(Box::new(self)) }

    fn get_dx_version(&self) -> DxVersion { DxVersion::Dx11 }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}