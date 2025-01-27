use gdnative::prelude::*;
use godot_egui::*;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct GodotEguiWindowExample {
    gui: Option<Instance<GodotEgui, Shared>>,
}

#[methods]
impl GodotEguiWindowExample {
    fn new(_: &Control) -> Self { Self { gui: None } }
    #[method]
    #[profiled]
    pub fn _ready(&mut self, #[base] owner: &Control) {
        godot_print!("Initializing godot egui");
        let gui = owner
            .get_node("GodotEgui")
            .and_then(|godot_egui| unsafe { godot_egui.assume_safe() }.cast::<Control>())
            .and_then(|godot_egui| godot_egui.cast_instance::<GodotEgui>())
            .expect("Expected a `GodotEgui` child with the GodotEgui nativescript class.");

        self.gui = Some(gui.claim());
    }

    /// Updates egui from the `_gui_input` callback
    #[method]
    #[profiled]
    pub fn _gui_input(&mut self, #[base] owner: &Control, event: Ref<InputEvent>) {
        let gui = unsafe { self.gui.as_ref().expect("GUI initialized").assume_safe() };
        gui.map_mut(|gui, instance| {
            let instance = instance.as_ref();
            gui.handle_godot_input(instance, event, true);
            if gui.mouse_was_captured(instance) {
                owner.accept_event();
            }
        })
        .expect("map_mut should succeed");
    }
    #[method]
    fn _process(&mut self, #[base] _owner: &Control, _: f64) {
        if let Some(gui) = &self.gui {
            let gui = unsafe { gui.assume_safe() };
            gui.map_mut(|egui, o| {
                egui.update_ctx(o.as_ref(), |ctx| {
                    egui::Window::new("Test Window")
                        .frame(egui::Frame::default())
                        .min_height(100.0)
                        .min_width(50.0)
                        .resizable(true)
                        .show(ctx, |ui| {
                            ui.label("This is a window!");
                        });
                })
            })
            .expect("this should work");
        }
    }
}
