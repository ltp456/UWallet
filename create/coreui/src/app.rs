use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};

use log::debug;

use crate::{
    AppState,
    executor::{*},
    IActivity,
    lifecycle::{*},
};

pub struct App {
    activities: HashMap<ActName, Box<dyn IActivity>>,
    lifecycle_manager: LifecycleManager,
    promise: Receiver<ActName>,
    pub state: AppState,

}

impl App {
    pub fn new(ctx: egui::Context, state: AppState) -> Self {
        let (sender,receiver) = std::sync::mpsc::channel::<ActName>();
        init_global_navigate(Navigate::new(sender, ctx));
        Self {
            activities: HashMap::new(),
            lifecycle_manager: LifecycleManager::new(),
            promise: receiver,
            state,
        }
    }

    pub fn register(&mut self, activity_key: &ActName, activity: impl IActivity + 'static) {
        self.lifecycle_manager.register(activity_key).unwrap();
        self.activities.insert(activity_key.clone(), Box::new(activity));
    }

    pub fn boot_act(&mut self, activity_key: &ActName, activity: impl IActivity + 'static) {
        self.lifecycle_manager.boot_act(activity_key).unwrap();
        self.activities.insert(activity_key.clone(), Box::new(activity));
    }

    pub fn update_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (current, prev) = self.lifecycle_manager.current();
        let current_activity = self.activities.get_mut(&current).unwrap();
        current_activity.set_view(ctx, _frame, &self.state);
        let lifecycle = self.lifecycle_manager.lifecycle(&current).unwrap();
        if lifecycle.on_create {
            current_activity.on_create(ctx, &self.state);
        }
        if lifecycle.on_resume {
            current_activity.on_resume(ctx, &self.state);
        }
        if let Some(prev_activity) = prev {
            let prev_lifecycle = self.lifecycle_manager.lifecycle(&prev_activity).unwrap();
            let prev_activity = self.activities.get_mut(&prev_activity).unwrap();
            if prev_lifecycle.on_pause {
                prev_activity.on_pause(ctx, &self.state);
            }
        }
        self.lifecycle_manager.reset_lifecycle();
        if let Ok(result) = self.promise.try_recv() {
            debug!("update activity to {}",result);
            self.lifecycle_manager.start_act(result).unwrap();
            ctx.request_repaint();
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}

