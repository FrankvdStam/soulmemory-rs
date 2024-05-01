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

use imgui::{TableFlags, TreeNodeFlags, Ui};
use crate::games::traits::buffered_event_flags::EventFlag;
use crate::games::*;
use crate::widgets::widget::Widget;

const EVENT_FLAG_SCROLL_REGION_HEIGHT: f32 = 400.0f32;

pub struct EventFlagWidget
{
    selected_log_mode_index: u32,
    unique_event_flags: Vec<EventFlag>,

    event_flags: Vec<EventFlag>,

    excluded_flags: Vec<u32>,
    exclusion_flag_input: String,

    watched_flags: Vec<u32>,
    watch_flag_input: String,
}

impl EventFlagWidget
{
    pub fn new() -> Self{
        EventFlagWidget
        {
            selected_log_mode_index: 1, //Select unqiue flags by default
            unique_event_flags: Vec::new(),
            event_flags: Vec::new(),

            excluded_flags: Vec::new(),
            exclusion_flag_input: String::new(),

            watched_flags: Vec::new(),
            watch_flag_input: String::new(),
        }
    }

    fn tab_event_flag_log(&mut self, ui: &Ui, _game: &mut Box<dyn Game>)
    {
        if let Some(log) = ui.tab_item("Log")
        {
            if ui.button("clear")
            {
                self.event_flags.clear();
            }

            ui.child_window("log_event_flags_scrollable")
                .size([ui.content_region_avail()[0], EVENT_FLAG_SCROLL_REGION_HEIGHT])
                .build(||
            {
                if let Some(_table_token) = ui.begin_table_with_flags("event flags", 3, TableFlags::RESIZABLE)
                {
                    ui.table_setup_column("time");
                    ui.table_setup_column("flag");
                    ui.table_setup_column("value");
                    ui.table_headers_row();

                    for f in self.event_flags.iter()
                    {
                        ui.table_next_column();
                        if ui.selectable_config(format!("{}", f.time.format("%H:%M:%S")))
                            .span_all_columns(true)
                            .build()
                        {
                            ui.set_clipboard_text(f.flag.to_string());
                        }

                        ui.table_next_column();
                        ui.text(f.flag.to_string());

                        ui.table_next_column();
                        if f.state
                        {
                            ui.text_colored([0.0f32, 1.0f32, 0.0f32, 1.0f32], "true")
                        }
                        else
                        {
                            ui.text_colored([1.0f32, 0.0f32, 0.0f32, 1.0f32], "false")
                        }
                    }
                }
            });

            log.end();
        }
    }

    fn tab_exclusions(&mut self, ui: &Ui, _game: &mut Box<dyn Game>)
    {
        if let Some(exclusions) = ui.tab_item("exclusions")
        {
            Self::flag_input_to_vec(ui, &mut self.exclusion_flag_input, &mut self.excluded_flags);

            ui.child_window("exclusions_event_flags_scrollable")
                .size([ui.content_region_avail()[0], EVENT_FLAG_SCROLL_REGION_HEIGHT])
                .build(||
            {
                //Draw excluded flags with delete option
                let mut delete_flag_index = None;
                for i in 0..self.excluded_flags.len()
                {
                    ui.text(format!("{: >10}", self.excluded_flags[i].to_string()));
                    ui.same_line();

                    let id = ui.push_id(i.to_string());
                    if ui.button("delete")
                    {
                        delete_flag_index = Some(i);
                    }
                    id.end();
                }

                if let Some(index) = delete_flag_index
                {
                    self.excluded_flags.remove(index);
                }
            });

            exclusions.end();
        }
    }

    fn tab_watch_event_flags(&mut self, ui: &Ui, game:  &mut Box<dyn Game>)
    {
        //Watch event flags
        if let Some(watch) = ui.tab_item("watch")
        {
            Self::flag_input_to_vec(ui, &mut self.watch_flag_input, &mut self.watched_flags);

            ui.child_window("watch_event_flags_scrollable")
                .size([ui.content_region_avail()[0], EVENT_FLAG_SCROLL_REGION_HEIGHT])
                .build(||
            {
                let mut delete_flag_index = None;
                for i in 0..self.watched_flags.len()
                {
                    ui.text(format!("{: >10}", self.watched_flags[i].to_string()));
                    ui.same_line();

                    if let Some(buffered_event_flags) = game.event_flags()
                    {
                        let flag_val = buffered_event_flags.get_event_flag_state(self.watched_flags[i]);
                        ui.text(format!("{: >5}", flag_val));
                        ui.same_line();

                        let id = ui.push_id(i.to_string());
                        if ui.button("delete")
                        {
                            delete_flag_index = Some(i);
                        }
                        id.end();
                    }

                }

                if let Some(index) = delete_flag_index
                {
                    self.watched_flags.remove(index);
                }
            });

            watch.end();
        }
    }

    fn flag_input_to_vec(ui: &Ui, input_string: &mut String, vec: &mut Vec<u32>)
    {
        ui.input_text("flag", input_string).build();
        ui.same_line();

        let mut disabled = false;
        let mut event_flag = 0;
        if let Ok(flag) = input_string.parse::<u32>()
        {
            event_flag = flag;
        }
        else
        {
            disabled = true;
        }

        ui.disabled(disabled, ||
        {
            if ui.button("add")
            {
                vec.push(event_flag);
                input_string.clear();
            }
        });
    }
}

impl Widget for EventFlagWidget
{
    fn render(&mut self, game: &mut Box<dyn Game>, ui: &Ui)
    {
        if let Some(event_flags) = game.event_flags()
        {
            let new_flags = event_flags.get_buffered_flags();
            for f in new_flags
            {
                match self.selected_log_mode_index
                {
                    0 => //let everything through
                    {
                        self.event_flags.push(f);
                    }

                    //Unique flags
                    1 =>
                    {
                        if self.unique_event_flags.iter().find(|p| p.flag == f.flag).is_none()
                        {
                            self.unique_event_flags.push(f);
                            self.event_flags.push(f);
                        }
                    }

                    //Exclusion list
                    2 =>
                    {
                        if self.excluded_flags.iter().find(|p| **p == f.flag).is_none()
                        {
                            self.event_flags.push(f);
                        }
                    }

                    _ => {}
                }
            }

            while self.event_flags.len() > 100
            {
                self.event_flags.remove(0);
            }

            if ui.collapsing_header("event flags", TreeNodeFlags::FRAMED)
            {
                ui.text("Log mode:");
                ui.radio_button("All", &mut self.selected_log_mode_index, 0);

                ui.radio_button("Unique", &mut self.selected_log_mode_index, 1);
                ui.same_line();
                if ui.button("clear unique list")
                {
                    self.unique_event_flags.clear();
                }

                ui.radio_button("Use exclusions", &mut self.selected_log_mode_index, 2);

                if let Some(tab_bar) = ui.tab_bar("event_flags")
                {
                    self.tab_event_flag_log(ui, game);
                    self.tab_exclusions(ui, game);
                    self.tab_watch_event_flags(ui, game);
                    tab_bar.end();
                };
            }
        }
    }
}