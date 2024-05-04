#[allow(dead_code)]

mod buffered_event_flags;

use std::any::Any;
use std::mem;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use ilhook::x86::{CallbackOption, Hooker, HookFlags, HookPoint, HookType, Registers};
use log::info;
use mem_rs::pointer::Pointer;
use mem_rs::prelude::Process;
use crate::App;
use crate::games::dx_version::DxVersion;
use crate::games::{Game, GameExt};
use crate::games::traits::buffered_event_flags::{BufferedEventFlags, EventFlag};
use crate::util::{get_stack_u32, get_stack_u8};

type FnGetEventFlag = fn(event_flag_man: u32, event_flag: u32) -> u8;

pub struct DarkSouls2Vanilla
{
    process: Process,

    event_flag_man: Pointer,
    event_flags: Arc<Mutex<Vec<EventFlag>>>,
    set_event_flag_hook: Option<HookPoint>,
    fn_get_event_flag: FnGetEventFlag,
}

impl DarkSouls2Vanilla
{
    pub fn new() -> Self
    {
        DarkSouls2Vanilla
        {
            process: Process::new("darksoulsii.exe"),

            event_flag_man: Default::default(),
            event_flags: Arc::new(Mutex::new(vec![])),
            set_event_flag_hook: None,
            fn_get_event_flag: |_,_|{return 0},
        }
    }
}

impl Game for DarkSouls2Vanilla
{
    #[cfg(target_pointer_width = "64")]
    fn refresh(&mut self) -> Result<(), String> { unimplemented!("DarkSouls2Vanilla is only available for x86"); }

    #[cfg(target_pointer_width = "32")]
    fn refresh(&mut self) -> Result<(), String>
    {
        if !self.process.is_attached()
        {
            unsafe
                {
                    self.process.refresh()?;
                    self.event_flag_man = self.process.scan_abs("GameManagerImp", "8B F1 8B 0D ? ? ? 01 8B 01 8B 50 28 FF D2 84 C0 74 0C", 4, vec![0, 0, 0x44, 0x10])?;
                    let get_event_flag_address = self.process.scan_abs("get_event_flag", "55 8b ec 53 56 57 8b 7d 08 b8 ? ? ? ? f7", 0, Vec::new())?.get_base_address();
                    let set_event_flag_address = self.process.scan_abs("set_event_flag", "55 8b ec 83 ec 08 53 56 8b 75 08 b8 ? ? ? ? f7", 0, Vec::new())?.get_base_address();

                    self.fn_get_event_flag = mem::transmute(get_event_flag_address);

                    unsafe extern "cdecl" fn read_event_flag_hook_fn(reg:*mut Registers, _:usize)
                    {
                        let instance = App::get_instance();
                        let app = instance.lock().unwrap();

                        if let Some(vanilla) = GameExt::get_game_ref::<DarkSouls2Vanilla>(app.game.deref())
                        {
                            let value           = get_stack_u8((*reg).esp, 0x8);
                            let event_flag_id   = get_stack_u32((*reg).esp, 0x4);

                            let mut guard = vanilla.event_flags.lock().unwrap();
                            guard.push(EventFlag::new(chrono::offset::Local::now(), event_flag_id, value != 0));
                        }
                    }

                    let h = Hooker::new(set_event_flag_address, HookType::JmpBack(read_event_flag_hook_fn), CallbackOption::None, 0, HookFlags::empty());
                    self.set_event_flag_hook = Some(h.hook().unwrap());

                    info!("event_flag_man base address: 0x{:x}", self.event_flag_man.get_base_address());
                    info!("get event flag address     : 0x{:x}", get_event_flag_address);
                    info!("get event flag address     : 0x{:x}", get_event_flag_address);
                }
        } else {
            self.process.refresh()?;
        }
        Ok(())
    }

    fn event_flags(&mut self) -> Option<Box<&mut dyn BufferedEventFlags>> { Some(Box::new(self)) }

    fn get_dx_version(&self) -> DxVersion { DxVersion::Dx9 }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}