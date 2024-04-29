use imgui::{Condition, TreeNodeFlags, Ui};

mod support;

fn main() {
    let system = support::init("test window");
    system.main_loop(move |run, ui|
    {
        ui.show_demo_window(run);
        draw_controls(ui);
    });
}

fn draw_controls(ui: &mut Ui)
{
    ui.window("Controls").size([500.0,500.0], Condition::FirstUseEver).build(|| {
        if ui.collapsing_header("position", TreeNodeFlags::empty())
        {
            ui.text("x:");
            ui.text("y:");
            ui.text("z:");
        }
    });
}