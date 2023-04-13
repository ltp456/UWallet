use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};

use anyhow::{anyhow, Result};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct ActName(String);

impl ActName {
    pub fn new(name: &str) -> Self {
        ActName(name.to_owned())
    }
}


impl Display for ActName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Clone)]
pub struct Lifecycle {
    pub on_create: bool,
    pub on_resume: bool,
    pub on_pause: bool,
    pub on_destroy: bool,
}

impl Lifecycle {
    pub fn new() -> Self {
        Self {
            on_create: true,
            on_resume: false,
            on_pause: false,
            on_destroy: false,
        }
    }
}


pub enum IntentFlag {
    Normal,
    Finish,
}


pub struct StackManager {
    pub act_list: Vec<ActName>,
    pub lifecycles: HashMap<ActName, Lifecycle>,
    pub current: ActName,
    pub prev: Option<ActName>,
}


impl StackManager {
    pub fn new() -> Self {
        Self {
            act_list: Vec::new(),
            lifecycles: HashMap::new(),
            current: ActName::new(""),
            prev: None,

        }
    }

    pub fn contains(&mut self, act_name: &ActName) -> bool {
        if self.lifecycles.contains_key(act_name) {
            return true;
        }
        return false;
    }


    pub fn current(&self) -> (ActName, Option<ActName>) {
        (self.current.clone(), self.prev.clone())
    }


    pub fn start_act(&mut self, act_name: ActName) -> Result<()> {
        if !self.contains(&act_name) {
            return Err(anyhow!("{} haven't registered",act_name));
        }
        if self.current == act_name {
            return Ok(());
        }
        if let Some(lifecycle) = self.lifecycles.get_mut(&act_name) {
            lifecycle.on_resume = true;
            lifecycle.on_pause = false;
            lifecycle.on_destroy = false;
        } else {
            return Err(anyhow!("get {} error",act_name));
        }
        self.prev = Some(self.current.clone());
        if let Some(prev_act) = &self.prev {
            if let Some(lifecycle) = self.lifecycles.get_mut(prev_act) {
                lifecycle.on_create = false;
                lifecycle.on_resume = false;
                lifecycle.on_pause = true;
                lifecycle.on_destroy = false;
            }
        }
        self.current = act_name.clone();
        return Ok(());
    }

    pub fn reset_lifecycle(&mut self) {
        if let Some(lifecycle) = self.lifecycles.get_mut(&self.current) {
            lifecycle.on_resume = false;
            lifecycle.on_create = false;
        }

        if let Some(prev_act) = &self.prev {
            if let Some(lifecycle) = self.lifecycles.get_mut(prev_act) {
                lifecycle.on_pause = false;
            }
        }
    }


    pub fn info(&self) {
        let life = self.lifecycle(&self.current).unwrap();
        println!("current: name: {}, on_create: {},on_resume: {},on_pause: {},on_destroy: {}", self.current, life.on_create, life.on_resume, life.on_pause, life.on_destroy);
        if let Some(prev) = &self.prev {
            let life = self.lifecycle(prev).unwrap();
            println!("prev: name: {}, on_create: {},on_resume: {},on_pause: {},on_destroy: {}", prev, life.on_create, life.on_resume, life.on_pause, life.on_destroy);
        }
        println!("---------------------");
    }


    pub fn lifecycle(&self, act_name: &ActName) -> Option<&Lifecycle> {
        self.lifecycles.get(act_name)
    }


    pub fn mut_lifecycle(&mut self, act_name: &ActName) -> Option<&mut Lifecycle> {
        self.lifecycles.get_mut(act_name)
    }

    pub fn boot_act(&mut self, act_name: &ActName) -> Result<()> {
        self.register(act_name)?;
        self.current = act_name.clone();
        self.prev = None;
        return Ok(());
    }


    pub fn register(&mut self, act_name: &ActName) -> Result<()> {
        if self.contains(act_name) {
            return Err(anyhow!("{} activity have registered", act_name));
        }
        self.act_list.push(act_name.clone());
        self.lifecycles.insert(act_name.clone(), Lifecycle::new());
        return Ok(());
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut stack_manager = StackManager::new();
        stack_manager.boot_act(&ActName::new("zero")).unwrap();
        stack_manager.register(&ActName::new("one")).unwrap();
        stack_manager.register(&ActName::new("two")).unwrap();
        stack_manager.register(&ActName::new("three")).unwrap();

        stack_manager.start_act(ActName::new("one"), IntentFlag::Normal).unwrap();
        stack_manager.info();
        stack_manager.reset_lifecycle();

        stack_manager.start_act(ActName::new("two"), IntentFlag::Normal).unwrap();
        stack_manager.info();
        stack_manager.reset_lifecycle();


        stack_manager.start_act(ActName::new("three"), IntentFlag::Normal).unwrap();
        stack_manager.info();
        stack_manager.reset_lifecycle();

        stack_manager.start_act(ActName::new("zero"), IntentFlag::Normal).unwrap();
        stack_manager.info();
        stack_manager.reset_lifecycle();

        stack_manager.start_act(ActName::new("one"), IntentFlag::Normal).unwrap();
        stack_manager.info();
        stack_manager.reset_lifecycle();
    }
}