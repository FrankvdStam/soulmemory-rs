use std::collections::HashMap;
use hudhook::hooks::ImguiRenderLoopFlags;
use imgui::{TreeNodeFlags, Ui};
use crate::games::GameEnum;
use crate::games::sekiro::SekiroChrDbgFlag;
use crate::gui::widget::Widget;

pub struct SekiroChrDbgFlagsWidget
{
    flags: HashMap<SekiroChrDbgFlag, bool>
}

impl SekiroChrDbgFlagsWidget
{
    pub fn new() -> Self{ SekiroChrDbgFlagsWidget{ flags: HashMap::new()} }
}

impl Widget for SekiroChrDbgFlagsWidget
{
    fn render(&mut self, game: &mut GameEnum, ui: &Ui, _flags: &ImguiRenderLoopFlags)
    {
        let sekiro = game.borrow_sekiro();
        if self.flags.len() != 14
        {
            self.flags = sekiro.get_chr_dbg_flags();
        }

        if self.flags.len() != 14
        {
            return;
        }

        if ui.collapsing_header("chr dbg", TreeNodeFlags::FRAMED)
        {
            let mut changed_flag = None;

            if ui.checkbox("Player No Dead", &mut self.flags.get_mut(&SekiroChrDbgFlag::PlayerNoDead).unwrap()) {
                changed_flag = Some(SekiroChrDbgFlag::PlayerNoDead);
            }
            if ui.checkbox("Player Exterminate", &mut self.flags.get_mut(&SekiroChrDbgFlag::PlayerExterminate).unwrap()) {
                changed_flag = Some(SekiroChrDbgFlag::PlayerExterminate);
            }
            if ui.checkbox("Player Exterminate Stamina", &mut self.flags.get_mut(&SekiroChrDbgFlag::PlayerExterminateStamina).unwrap()) {
                changed_flag = Some(SekiroChrDbgFlag::PlayerExterminateStamina);
            }
            if ui.checkbox("Player No Goods Consume", &mut self.flags.get_mut(&SekiroChrDbgFlag::PlayerNoGoodsConsume).unwrap()) {
                changed_flag = Some(SekiroChrDbgFlag::PlayerNoGoodsConsume);
            }
            if ui.checkbox("Player No Resource Item Consume", &mut self.flags.get_mut(&SekiroChrDbgFlag::PlayerNoResourceItemConsume).unwrap()) {
                changed_flag = Some(SekiroChrDbgFlag::PlayerNoResourceItemConsume);
            }
            if ui.checkbox("Player No Revival Consume", &mut self.flags.get_mut(&SekiroChrDbgFlag::PlayerNoRevivalConsume).unwrap()) {
                changed_flag = Some(SekiroChrDbgFlag::PlayerNoRevivalConsume);
            }
            if ui.checkbox("Player Hide", &mut self.flags.get_mut(&SekiroChrDbgFlag::PlayerHide).unwrap()) {
                changed_flag = Some(SekiroChrDbgFlag::PlayerHide);
            }
            if ui.checkbox("Player Silenced", &mut self.flags.get_mut(&SekiroChrDbgFlag::PlayerSilenced).unwrap()) {
                changed_flag = Some(SekiroChrDbgFlag::PlayerSilenced);
            }
            if ui.checkbox("All No Dead", &mut self.flags.get_mut(&SekiroChrDbgFlag::AllNoDead).unwrap()) {
                changed_flag = Some(SekiroChrDbgFlag::AllNoDead);
            }
            if ui.checkbox("All No Damage", &mut self.flags.get_mut(&SekiroChrDbgFlag::AllNoDamage).unwrap()) {
                changed_flag = Some(SekiroChrDbgFlag::AllNoDamage);
            }
            if ui.checkbox("All No Hit", &mut self.flags.get_mut(&SekiroChrDbgFlag::AllNoHit).unwrap()) {
                changed_flag = Some(SekiroChrDbgFlag::AllNoHit);
            }
            if ui.checkbox("All No Attack", &mut self.flags.get_mut(&SekiroChrDbgFlag::AllNoAttack).unwrap()) {
                changed_flag = Some(SekiroChrDbgFlag::AllNoAttack);
            }
            if ui.checkbox("All No Move", &mut self.flags.get_mut(&SekiroChrDbgFlag::AllNoMove).unwrap()) {
                changed_flag = Some(SekiroChrDbgFlag::AllNoMove);
            }
            if ui.checkbox("All No Update Ai", &mut self.flags.get_mut(&SekiroChrDbgFlag::AllNoUpdateAi).unwrap()) {
                changed_flag = Some(SekiroChrDbgFlag::AllNoUpdateAi);
            }

            if let Some(flag) = changed_flag
            {
                sekiro.set_chr_dbg_flag(flag, self.flags.get(&flag).unwrap().clone());
            }
        }
    }
}
