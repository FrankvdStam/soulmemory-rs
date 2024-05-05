use std::sync::{Arc, Mutex};
use mem_rs::prelude::ReadWrite;
use crate::games::DarkSouls2Vanilla;
use crate::games::traits::buffered_event_flags::{BufferedEventFlags, EventFlag};

impl BufferedEventFlags for DarkSouls2Vanilla
{
    fn access_flag_storage(&self) -> &Arc<Mutex<Vec<EventFlag>>>
    {
        return &self.event_flags;
    }

    fn get_event_flag_state(&self, event_flag: u32) -> bool
    {
        let event_flag_man_address = self.event_flag_man.read_u32_rel(None);
        let result = unsafe { (self.fn_get_event_flag)(event_flag_man_address, event_flag) };
        return result == 1;
    }
}