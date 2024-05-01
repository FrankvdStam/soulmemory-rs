use imgui::{Condition, TreeNodeFlags, Ui};
use windows::Win32::Foundation::HINSTANCE;
use rand::random;
use soulmemory_rs::{App, MockGame};

mod support;

fn main() {
    App::init(&String::from("mockgame.exe"), HINSTANCE(0));

    let system = support::init("test window");
    system.main_loop(move |_run, ui|
    {
        let instance = App::get_instance();
        let mut app = instance.lock().unwrap();

        app.refresh().unwrap();
        app.render(ui);

        //ui.show_demo_window(run);
        draw_controls(ui, &mut app);
    });
}

fn draw_controls(ui: &mut Ui, app: &mut App)
{
    ui.window("Controls")
        .size([500.0,800.0], Condition::Appearing)
        .position([500.0f32, 50.0f32], Condition::Appearing)
        .build(|| {
        if ui.collapsing_header("position", TreeNodeFlags::empty())
        {
            ui.text("x:");
            ui.text("y:");
            ui.text("z:");
        }

        if ui.collapsing_header("event flags", TreeNodeFlags::DEFAULT_OPEN)
        {
            if let Some(mock_game) = app.game.as_any().downcast_ref::<MockGame>()
            {
                if ui.button("raise random event flag")
                {
                    mock_game.raise_event_flag(random::<u32>(), random::<u32>() % 2 == 0);
                }
            }
        }
    });
}