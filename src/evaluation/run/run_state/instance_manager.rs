use std::collections::{HashMap, LinkedList};

use crate::evaluation::{RuntimeError, RuntimeValue};

const INITIAL_INSTANCE_CAPACITY: usize = 500;

pub type ClassInstance = HashMap<String, RuntimeValue>;
type SuperClassPointer = Option<usize>;

pub struct InstanceManager {
    instances: Vec<Option<(String, ClassInstance, SuperClassPointer)>>,
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

    pub fn initialize_instance(
        &mut self,
        class_name: String,
        super_class: SuperClassPointer,
    ) -> Result<usize, RuntimeError> {
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

        self.instances[available] = Some((class_name, ClassInstance::new(), super_class));

        Ok(available)
    }

    pub fn get_instance_value(
        &self,
        index: usize,
        key: &str,
        max_depth: Option<usize>,
        min_depth: Option<usize>,
    ) -> Result<Option<RuntimeValue>, RuntimeError> {
        if let Some(max_depth) = max_depth {
            if max_depth == 0 {
                return Ok(None);
            }
        }

        if let Some(min_depth) = min_depth {
            if min_depth > 0 {
                let super_class = self.instances[index]
                    .as_ref()
                    .map(|(_, _, super_class)| super_class.as_ref().cloned())
                    .flatten();
                if let Some(super_class) = super_class {
                    return self.get_instance_value(
                        super_class,
                        key,
                        max_depth,
                        Some(min_depth - 1),
                    );
                }
                return Ok(None);
            }
        }

        let current = self
            .instances
            .get(index)
            .map(|o| o.as_ref())
            .flatten()
            .ok_or(RuntimeError::InstanceNotFound(index))
            .map(|(_, instance, _)| instance.get(key).cloned());
        if let Ok(None) = current {
            if let Some((_, _, super_class)) = self.instances[index].as_ref() {
                return super_class
                    .map(|super_class| {
                        self.get_instance_value(
                            super_class,
                            key,
                            max_depth.map(|d| d - 1),
                            min_depth,
                        )
                    })
                    .transpose()
                    .map(|o| o.flatten());
            }
        }
        current
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

    pub fn map_this_pointer(
        &mut self,
        index: usize,
        this_pointer: usize,
    ) -> Result<(), RuntimeError> {
        let (_, _, super_class) = self
            .instances
            .get_mut(index)
            .map(|o| o.as_mut())
            .flatten()
            .ok_or(RuntimeError::InstanceNotFound(index))?;

        let mut super_class = *super_class;

        while let Some(some_super_class) = super_class {
            let (_, instance, inner_super_class) = self
                .instances
                .get_mut(some_super_class)
                .map(|o| o.as_mut())
                .flatten()
                .ok_or(RuntimeError::InstanceNotFound(some_super_class))?;
            instance.iter_mut().for_each(|(_, v)| {
                v.map_this_pointer(this_pointer);
            });
            super_class = inner_super_class.clone();
        }

        Ok(())
    }
}
