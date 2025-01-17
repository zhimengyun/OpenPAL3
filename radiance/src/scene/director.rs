use super::SceneManager;
use imgui::Ui;
use std::{cell::RefCell, rc::Rc};

pub trait Director: downcast_rs::Downcast {
    fn activate(&mut self, scene_manager: &mut dyn SceneManager);
    fn update(
        &mut self,
        scene_manager: &mut dyn SceneManager,
        ui: &Ui,
        delta_sec: f32,
    ) -> Option<Rc<RefCell<dyn Director>>>;
}

downcast_rs::impl_downcast!(Director);
