use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use log::debug;

use super::IActivity;

#[derive(Clone)]
pub struct Navigation(Arc<Mutex<InnerNavigation>>);


impl Navigation {
    pub fn new() -> Self {
        let inner_navigation = InnerNavigation::new();
        Navigation(Arc::new(Mutex::new(inner_navigation)))
    }

    pub fn init_page(&self, target: String) {}


    pub fn register(&self, page: String, p_type: i32) {}
    pub fn reset(&self, page: String, show: bool) {}

    pub fn navigate(&self, target: String) {}
}


#[derive(Default, Clone, Eq, Hash, PartialEq)]
pub struct ActivityKey(String);

impl Display for ActivityKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl ActivityKey {
    pub fn new(name: &str) -> Self {
        ActivityKey(name.to_string())
    }
}


pub enum ActivityType {
    Full,
    Part,
}


pub struct ActivityState {}


#[derive(Clone)]
pub struct Lifecycle {
    pub on_create: bool,
    pub on_resume: bool,
    pub on_pause: bool,
    pub on_destroy: bool,
}

impl Lifecycle {
    pub fn new() -> Self {
        Lifecycle {
            on_create: true,
            on_resume: false,
            on_pause: false,
            on_destroy: false,
        }
    }
}

// todo stack  manager
pub struct InnerNavigation {
    pub list_lifecycles: HashMap<ActivityKey, Lifecycle>,
    pub list: Vec<ActivityKey>,
    pub current_page: Option<ActivityKey>,
    pub prev_page: Option<ActivityKey>,
    pub boot_page: ActivityKey,
}


impl InnerNavigation {
    pub fn new() -> Self {
        let list_states: HashMap<ActivityKey, Lifecycle> = HashMap::new();
        let list: Vec<ActivityKey> = Vec::new();
        InnerNavigation {
            list_lifecycles: list_states,
            list,
            prev_page: Default::default(),
            current_page: Default::default(),
            boot_page: Default::default(),
        }
    }


    pub fn info(&self) {
        if let Some(current) = &self.current_page {
            let lif = self.list_lifecycles.get(current).unwrap();
            println!("{} on_create {} on_resume {} on_pause {}", current, lif.on_create, lif.on_resume, lif.on_pause);
        }
        if let Some(prev) = &self.prev_page {
            let lif = self.list_lifecycles.get(prev).unwrap();
            println!("{} on_create {} on_resume {} on_pause {}", prev, lif.on_create, lif.on_resume, lif.on_pause);
        }
        println!("------------------")
    }


    pub fn init_activity(&mut self, target: ActivityKey) -> Result<()> {
        if let Some(value) = self.current_page.clone() {
            return Err(anyhow!("init activity had registered"));
        }
        self.current_page = Some(target.clone());
        self.boot_page = target.clone();
        self.list.push(target.clone());
        self.list_lifecycles.insert(target.clone(), Lifecycle {
            on_create: true,
            on_resume: true,
            on_pause: false,
            on_destroy: false,
        });
        return Ok(());
    }

    pub fn register(&mut self, target: ActivityKey) -> Result<()> {
        let tmp_target = target.clone();
        for item in &self.list {
            if tmp_target.0 == item.0 {
                return Err(anyhow!("{} have registered",tmp_target));
            }
        }
        self.list.push(target.clone());
        self.list_lifecycles.insert(target.clone(), Lifecycle::new());
        return Ok(());
    }

    pub fn get_lifecycle(&mut self, target: &ActivityKey) -> &mut Lifecycle {
        self.list_lifecycles.get_mut(target).unwrap()
    }

    pub fn get_current_activity_lifecycle(&mut self) -> &mut Lifecycle {
        let activity_key = self.current_page.clone().unwrap();
        self.list_lifecycles.get_mut(&activity_key).unwrap()
    }


    pub fn get_current_activity(&self) -> Option<ActivityKey> {
        self.current_page.clone()
    }


    pub fn get_prev_activity(&self)->Option<ActivityKey>{
        self.prev_page.clone()
    }


    pub fn navigate(&mut self, target: ActivityKey) {
        self.list_lifecycles.entry((target.clone())).and_modify(|lif| {
            lif.on_resume = true;
            lif.on_pause = false;
        });
        self.prev_page = self.current_page.clone();
        self.current_page = Some(target);
        if let Some(prev) = self.prev_page.clone() {
            self.list_lifecycles.entry(prev).and_modify(|lif| {
                lif.on_pause = true;
                lif.on_resume = false;
                if lif.on_create {
                    lif.on_create = false;
                }
            });
        }
    }
}

#[cfg(test)]
mod test {
    use crate::navigation::{ActivityKey, InnerNavigation};

    #[test]
    fn test() {
        let mut navigation = InnerNavigation::new();
        navigation.init_activity(ActivityKey::new("weclome")).unwrap();
        navigation.register(ActivityKey::new("password")).unwrap();
        navigation.register(ActivityKey::new("phrase")).unwrap();
        navigation.register(ActivityKey::new("home")).unwrap();
        navigation.info();
        navigation.navigate(ActivityKey::new("password"));
        navigation.info();
        navigation.navigate(ActivityKey::new("phrase"));
        navigation.info();
        navigation.navigate(ActivityKey::new("home"));
        navigation.info();
        navigation.navigate(ActivityKey::new("phrase"));
        navigation.info();
    }
}
