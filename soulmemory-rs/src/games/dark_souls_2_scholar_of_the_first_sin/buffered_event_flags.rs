use std::sync::{Arc, Mutex};
use mem_rs::prelude::ReadWrite;
use crate::games::{DarkSouls2ScholarOfTheFirstSin};
use crate::games::traits::buffered_event_flags::{BufferedEventFlags, EventFlag};

impl BufferedEventFlags for DarkSouls2ScholarOfTheFirstSin
{
    fn access_flag_storage(&self) -> &Arc<Mutex<Vec<EventFlag>>>
    {
        return &self.event_flags;
    }

    fn get_event_flag_state(&self, event_flag: u32) -> bool
    {
        let event_flag_man_address = self.event_flag_man.read_u32_rel(None) as u64; //Bit memes because DSR is 64bit, compiled with 32bit wide pointers
        let result = (self.fn_get_event_flag)(event_flag_man_address, event_flag);
        return result == 1;
    }
}