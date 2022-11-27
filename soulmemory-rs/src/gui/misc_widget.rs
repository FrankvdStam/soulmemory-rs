use crate::gui::widget::Widget;
use crate::games::GameEnum;
use imgui::{TreeNodeFlags, Ui};
use hudhook::hooks::ImguiRenderLoopFlags;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_OEM_5;
use crate::games::sekiro::Sekiro;

pub struct MiscWidget
{

}

impl MiscWidget
{
    pub fn new() -> Self { MiscWidget{}}

    fn render_misc_sekiro(&mut self, sekiro: &mut Sekiro, ui: &Ui)
    {
        if ui.button("quitout") || ui.io().keys_down[VK_OEM_5.0 as usize]
        {
            sekiro.request_quitout();
        }
    }
}

impl Widget for MiscWidget
{
    fn render(&mut self, game: &mut GameEnum, ui: &Ui, _flags: &ImguiRenderLoopFlags)
    {
        if ui.collapsing_header("misc", TreeNodeFlags::FRAMED)
        {
            if let GameEnum::Sekiro(sekiro) = game
            {
                self.render_misc_sekiro(sekiro, ui);
            }
        }
    }
}