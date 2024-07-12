use std::ops::DerefMut;
use imgui::{TreeNodeFlags, Ui};
use windows::Win32::UI::Input::KeyboardAndMouse::VK_OEM_5;
use crate::games::{Game, GameExt, Sekiro};
use crate::widgets::misc_widget::MiscWidget;
use crate::widgets::widget::Widget;

pub struct SleepWidget
{
}

impl SleepWidget
{
    pub fn new() -> Self { SleepWidget{}}

    fn render_misc_sekiro(&mut self, sekiro: &mut Sekiro, ui: &Ui)
    {
        if ui.button("quitout") || ui.io().keys_down[VK_OEM_5.0 as usize]
        {
            sekiro.request_quitout();
        }
    }
}

impl Widget for SleepWidget
{
    fn render(&mut self, game: &mut Box<dyn Game>, ui: &Ui)
    {
        if let Some(sekiro) = GameExt::get_game_mut::<Sekiro>(game.deref_mut())
        {
            if ui.collapsing_header("sleep", TreeNodeFlags::FRAMED)
            {
                let _ = ui.input_scalar("delay (ms)", &mut sekiro.increment_igt_delay_ms).build();
            }
        }
    }
}