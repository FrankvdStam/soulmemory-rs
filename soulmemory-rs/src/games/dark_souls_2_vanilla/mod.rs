mod buffered_event_flags;

use std::any::Any;
use std::mem;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use ilhook::x86::{CallbackOption, Hooker, HookFlags, HookPoint, HookType, Registers};
use log::info;
use mem_rs::prelude::*;
use crate::App;
use crate::games::dx_version::DxVersion;
use crate::games::{Game, GameExt};
use crate::games::traits::buffered_event_flags::{BufferedEventFlags, EventFlag};
use crate::util::{get_stack_u32, get_stack_u8};

type FnGetEventFlag = unsafe extern "thiscall" fn(event_flag_man: u32, event_flag: u32) -> u8;

pub struct DarkSouls2Vanilla
{
    process: Process,

    event_flag_man: Pointer,
    event_flags: Arc<Mutex<Vec<EventFlag>>>,
    set_event_flag_hook: Option<HookPoint>,
    fn_get_event_flag: FnGetEventFlag,

    send_morpheme_message_hook: Option<HookPoint>,
}

impl DarkSouls2Vanilla
{
    pub fn new() -> Self
    {
        unsafe extern "thiscall" fn empty(_: u32, _: u32) -> u8 { 0 }


        DarkSouls2Vanilla
        {
            process: Process::new("darksoulsii.exe"),

            event_flag_man: Default::default(),
            event_flags: Arc::new(Mutex::new(vec![])),
            set_event_flag_hook: None,
            fn_get_event_flag: empty,

            send_morpheme_message_hook: None,
        }
    }
}

impl Game for DarkSouls2Vanilla
{
    fn refresh(&mut self) -> Result<(), String>
    {
        if !self.process.is_attached()
        {
            unsafe
                {
                    self.process.refresh()?;
                    self.event_flag_man = self.process.scan_abs("GameManagerImp", "56 ff d2 c7 05 ? ? ? ? 00 00 00 00 5e", 5, vec![0, 0, 0x44, 0x10])?;
                    let get_event_flag_address = self.process.scan_abs("get_event_flag", "55 8b ec 53 56 57 8b 7d 08 b8 ? ? ? ? f7", 0, Vec::new())?.get_base_address();
                    let set_event_flag_address = self.process.scan_abs("set_event_flag", "55 8b ec 83 ec 08 53 56 8b 75 08 b8 ? ? ? ? f7", 0, Vec::new())?.get_base_address();
                    let send_morpheme_message_address = self.process.scan_abs("send_morpheme_message", "55 8b ec 51 56 8b f1 8b 4d 08 8b 51 08", 0, Vec::new())?.get_base_address();

                    self.fn_get_event_flag = mem::transmute(get_event_flag_address);

                    unsafe extern "cdecl" fn set_event_flag_hook_fn(reg:*mut Registers, _:usize)
                    {
                        let instance = App::get_instance();
                        let app = instance.lock().unwrap();

                        if let Some(vanilla) = GameExt::get_game_ref::<DarkSouls2Vanilla>(app.game.deref())
                        {
                            let value            = get_stack_u8((*reg).esp, 0x8);
                            let event_flag_id   = get_stack_u32((*reg).esp, 0x4);

                            let mut guard = vanilla.event_flags.lock().unwrap();
                            guard.push(EventFlag::new(chrono::offset::Local::now(), event_flag_id, value != 0));
                        }
                    }

                    let h = Hooker::new(set_event_flag_address, HookType::JmpBack(set_event_flag_hook_fn), CallbackOption::None, 0, HookFlags::empty());
                    self.set_event_flag_hook = Some(h.hook().unwrap());

                    unsafe extern "cdecl" fn send_morpheme_message_hook_fn(registers: *mut Registers, _:usize)
                    {
                        let instance = App::get_instance();
                        let app = instance.lock().unwrap();
                        if let Some(vanilla) = GameExt::get_game_ref::<DarkSouls2Vanilla>(app.game.deref())
                        {
                            let network_ptr = (*registers).ecx;
                            let morpheme_message   = get_stack_u32((*registers).esp, 0x4);


                            let mut buffer = [0; 4];
                            vanilla.process.read_memory_abs((morpheme_message + 0x8) as usize, &mut buffer);
                            let message_id = u32::from_ne_bytes(buffer);

                            let temp = vanilla.process.create_pointer(network_ptr as usize, vec![0xc]).read_u32_rel(Some(0x6c978));
                            info!("temp {}", temp);

                            let network = vanilla.process.create_pointer(network_ptr as usize, vec![0xc, 0x6c978]);
                            info!("network 0x{:x} message_id {} eventActionCategory {}", network_ptr, message_id, network.read_u32_rel(Some(0x10)));
                        }


                    }

                    let h = Hooker::new(send_morpheme_message_address, HookType::JmpBack(send_morpheme_message_hook_fn), CallbackOption::None, 0, HookFlags::empty());
                    self.send_morpheme_message_hook = Some(h.hook().unwrap());

                    info!("event_flag_man base address          : 0x{:x}", self.event_flag_man.get_base_address());
                    info!("get event flag address               : 0x{:x}", get_event_flag_address);
                    info!("set event flag address               : 0x{:x}", set_event_flag_address);
                    info!("send_morpheme_message_address address: 0x{:x}", send_morpheme_message_address);
                }
        } else {
            self.process.refresh()?;
        }
        Ok(())
    }

    fn get_dx_version(&self) -> DxVersion { DxVersion::Dx9 }

    fn event_flags(&mut self) -> Option<Box<&mut dyn BufferedEventFlags>> { Some(Box::new(self)) }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}