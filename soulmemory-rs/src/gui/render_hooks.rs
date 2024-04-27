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

use hudhook::hooks;
use hudhook::hooks::{ImguiRenderLoop, ImguiRenderLoopFlags};
use hudhook::hooks::dx11::ImguiDx11Hooks;
use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::hooks::dx9::ImguiDx9Hooks;
use imgui::Ui;
use crate::App;
use crate::games::{DxVersion, Game};

pub struct RenderHooks;

impl RenderHooks
{
    pub fn init()
    {
        let instance = App::get_instance();
        let app = instance.lock().unwrap();

        hudhook::lifecycle::global_state::set_module(hudhook::reexports::HINSTANCE(app.hmodule.0));
        let hooks: Box<dyn hooks::Hooks> = match app.game.get_dx_version()
        {
            DxVersion::Dx9  => RenderHooks::new().into_hook::<ImguiDx9Hooks>(),
            DxVersion::Dx11 => RenderHooks::new().into_hook::<ImguiDx11Hooks>(),
            DxVersion::Dx12 => RenderHooks::new().into_hook::<ImguiDx12Hooks>(),
        };
        unsafe { hooks.hook() };
        hudhook::lifecycle::global_state::set_hooks(hooks);
    }

    pub fn new() -> Self { RenderHooks {} }
}

impl ImguiRenderLoop for RenderHooks
{
    fn render(&mut self, ui: &mut Ui, flags: &ImguiRenderLoopFlags)
    {
        let instance = App::get_instance();
        let mut app = instance.lock().unwrap();
        app.render(ui, flags);
    }
}