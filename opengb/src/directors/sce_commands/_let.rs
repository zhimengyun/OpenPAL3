use crate::directors::sce_vm::{SceCommand, SceState};
use imgui::Ui;
use radiance::scene::SceneManager;

#[derive(Debug, Clone)]
pub struct SceCommandLet {
    var: i16,
    value: i32,
}

impl SceCommand for SceCommandLet {
    fn update(
        &mut self,
        scene_manager: &mut dyn SceneManager,
        ui: &Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        state
            .global_state_mut()
            .persistent_state_mut()
            .set_global(self.var, self.value);
        true
    }
}

impl SceCommandLet {
    pub fn new(var: i16, value: i32) -> Self {
        Self { var, value }
    }
}
