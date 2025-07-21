use std::collections::{HashMap, LinkedList};

use crate::evaluation::{RuntimeError, RuntimeValue};

const INITIAL_INSTANCE_CAPACITY: usize = 500;

pub type ClassInstance = HashMap<String, RuntimeValue>;

pub struct InstanceManager {
    instances: Vec<Option<(String, ClassInstance)>>,
    available: LinkedList<usize>,
}

impl InstanceManager {
    pub fn new() -> Result<Self, RuntimeError> {
        let mut instances = Vec::new();
        instances.try_reserve_exact(INITIAL_INSTANCE_CAPACITY)?;
        instances.resize(INITIAL_INSTANCE_CAPACITY, None);

        let mut available = LinkedList::new();
        for i in 0..INITIAL_INSTANCE_CAPACITY {
            available.push_back(i);
        }

        Ok(Self {
            instances,
            available,
        })
    }

    pub fn initialize_instance(&mut self, class_name: String) -> Result<usize, RuntimeError> {
        let mut available = self.available.pop_front();
        if available.is_none() {
            let current_capacity = self.instances.len();
            self.instances.resize(2 * current_capacity, None);
            available = Some(current_capacity);
            for i in current_capacity..2 * current_capacity {
                self.available.push_back(i);
            }
        }
        let available = available.ok_or(RuntimeError::NotEnoughSpaceToAllocateNewInstance)?;

        self.instances[available] = Some((class_name, ClassInstance::new()));

        Ok(available)
    }

    pub fn get_instance_value(
        &self,
        index: usize,
        key: &str,
    ) -> Result<Option<RuntimeValue>, RuntimeError> {
        self.instances
            .get(index)
            .map(|o| o.as_ref())
            .flatten()
            .ok_or(RuntimeError::InstanceNotFound(index))
            .map(|o| o.1.get(key).cloned())
    }

    pub fn set_instance_value(
        &mut self,
        index: usize,
        key: &str,
        value: RuntimeValue,
    ) -> Result<(), RuntimeError> {
        let instance = self
            .instances
            .get_mut(index)
            .map(|o| o.as_mut())
            .flatten()
            .ok_or(RuntimeError::InstanceNotFound(index))?;
        instance.1.insert(key.to_string(), value);
        Ok(())
    }

    pub fn get_class_name(&self, index: usize) -> Result<&str, RuntimeError> {
        self.instances
            .get(index)
            .map(|o| o.as_ref())
            .flatten()
            .ok_or(RuntimeError::InstanceNotFound(index))
            .map(|o| o.0.as_str())
    }
}
