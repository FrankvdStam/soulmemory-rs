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

use std::sync::{Arc, Mutex};
use windows::Win32::Foundation::HINSTANCE;
use imgui::{Condition, Ui};
use crate::games::dark_souls_3::DarkSouls3;
use crate::games::sekiro::Sekiro;
use crate::games::elden_ring::EldenRing;
use crate::games::armored_core_6::ArmoredCore6;
use crate::games::dark_souls_prepare_to_die_edition::DarkSoulsPrepareToDieEdition;
use crate::games::dark_souls_remastered::DarkSoulsRemastered;
use crate::games::mock_game::MockGame;
use crate::games::traits::game::Game;
use crate::widgets::widget::Widget;
use crate::util::server::Server;
use crate::widgets::event_flag_widget::EventFlagWidget;

pub struct App
{
    pub game: Box<dyn Game>,
    pub hmodule: HINSTANCE,
    #[allow(dead_code)]
    server: Server,
    widgets: Vec<Box<dyn Widget>>,
}

impl App
{
    pub fn init(process_name: &String, hmodule: HINSTANCE)
    {
        unsafe
        {
            if APP.is_some()
            {
                panic!("init called on app while it is already instantiated.");
            }
            APP = Some(Arc::new(Mutex::new(App::new(process_name, hmodule))));
        };
    }

    pub fn get_instance() -> Arc<Mutex<App>>
    {
        unsafe
        {
            if APP.is_none()
            {
                panic!("get_instance called on app while it is not instantiated.");
            }
            return Arc::clone(APP.as_mut().unwrap());
        };
    }

    pub fn new(process_name: &String, hmodule: HINSTANCE) -> Self
    {
        //Init the game we're injected in
        let game: Box<dyn Game> = match process_name.to_lowercase().as_str()
        {
            "mockgame.exe"              => Box::new(MockGame::new()),
            "darksouls.exe"             => Box::new(DarkSoulsPrepareToDieEdition::new()),
            "darksoulsremastered.exe"   => Box::new(DarkSoulsRemastered::new()),
            "darksoulsiii.exe"          => Box::new(DarkSouls3::new()),
            "sekiro.exe"                => Box::new(Sekiro::new()),
            "eldenring.exe"             => Box::new(EldenRing::new()),
            "armoredcore6.exe"          => Box::new(ArmoredCore6::new()),
            _                           => panic!("unsupported process: {}", process_name.to_lowercase()),
        };

        //get drawable widgets
        //let widgets = game.get_widgets();

        App
        {
            game,
            hmodule,
            server: Server::new(String::from("127.0.0.1:54345")),
            widgets: vec!{Box::new(EventFlagWidget::new())}
        }
    }

    pub fn refresh(&mut self) -> Result<(), String>
    {
        self.game.refresh()?;
        Ok(())
    }

    pub fn render(&mut self, ui: &mut Ui)
    {
        ui.window("soulmemory-rs").size([350.0, 800.0], Condition::FirstUseEver).build(||
        {
            for w in &mut self.widgets
            {
                w.render(&mut self.game, ui);
            }
        });
        //ui.show_demo_window(&mut true);
    }
}




impl Default for App
{
    fn default() -> Self
    {
        App
        {
            //TODO: default mock game
            game: Box::new(MockGame{}),
            hmodule: HINSTANCE(0),
            server: Server::default(),
            widgets: Vec::new(),
        }
    }
}

static mut APP: Option<Arc<Mutex<App>>> = None;
