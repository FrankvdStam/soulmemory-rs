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

use hudhook::HINSTANCE as HUDHOOK_HINSTANCE;
use hudhook::Hudhook;
use hudhook::hooks::{ImguiRenderLoop};
use hudhook::hooks::dx11::ImguiDx11Hooks;
use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::hooks::dx9::ImguiDx9Hooks;
use imgui::Ui;
use crate::App;
use crate::games::dx_version::DxVersion;

pub struct RenderHooks;

impl RenderHooks
{
    pub fn init()
    {
        let instance = App::get_instance();
        let app = instance.lock().unwrap();

        let mut builder = Hudhook::builder();

        builder = match app.game.get_dx_version()
        {
            DxVersion::Dx9  =>  builder.with(RenderHooks::new().into_hook::<ImguiDx9Hooks>()),
            DxVersion::Dx11 =>  builder.with(RenderHooks::new().into_hook::<ImguiDx11Hooks>()),
            DxVersion::Dx12 =>  builder.with(RenderHooks::new().into_hook::<ImguiDx12Hooks>()),
        };


        if let Err(e) = builder.with_hmodule(HUDHOOK_HINSTANCE(app.hmodule.0))
            .build()
            .apply()
        {
            panic!("{:?}", e)
        }
    }

    pub fn new() -> Self { RenderHooks {} }
}

impl ImguiRenderLoop for RenderHooks
{
    fn render(&mut self, ui: &mut Ui)
    {
        let instance = App::get_instance();
        let mut app = instance.lock().unwrap();
        app.render(ui);
    }
}