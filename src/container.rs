use std::fmt::Debug;

use bevy_ecs::entity::Entity;
use smallvec::{smallvec, SmallVec};

/// A container for entities that this entity (where this container is stored) is related to.
pub trait EntityContainer: Clone + PartialEq + Eq + Debug + Send + Sync + 'static {
    /// Creates a new entity container with the given entity.
    fn new(entity: Entity) -> Self;

    /// Returns `true` if this entity is not related to any other entities.
    fn is_empty(&self) -> bool;

    /// Returns the slice of entities that this entity is related to.
    fn as_slice(&self) -> &[Entity];

    /// Returns `true` if the given entity is related to this entity.
    fn contains(&self, entity: Entity) -> bool {
        self.as_slice().contains(&entity)
    }

    /// Adds the given entity to the list of entities that this entity is related to.
    fn push(&mut self, entity: Entity);

    /// Removes the given entity from the list of entities that this entity is related to.
    fn remove(&mut self, entity: Entity);
}

impl EntityContainer for Entity {
    fn new(entity: Entity) -> Self {
        entity
    }

    fn is_empty(&self) -> bool {
        *self == Entity::PLACEHOLDER
    }

    fn as_slice(&self) -> &[Entity] {
        core::array::from_ref(self)
    }

    fn contains(&self, entity: Entity) -> bool {
        *self == entity
    }

    fn push(&mut self, entity: Entity) {
        *self = entity;
    }

    fn remove(&mut self, entity: Entity) {
        if *self == entity {
            *self = Entity::PLACEHOLDER;
        }
    }
}

impl<const N: usize> EntityContainer for SmallVec<[Entity; N]> {
    fn new(entity: Entity) -> Self {
        smallvec![entity]
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn as_slice(&self) -> &[Entity] {
        self
    }

    fn push(&mut self, entity: Entity) {
        self.push(entity);
    }

    fn remove(&mut self, entity: Entity) {
        self.retain(|&mut id| id != entity);
    }
}

impl EntityContainer for Vec<Entity> {
    fn new(entity: Entity) -> Self {
        vec![entity]
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn as_slice(&self) -> &[Entity] {
        self
    }

    fn push(&mut self, entity: Entity) {
        self.push(entity);
    }

    fn remove(&mut self, entity: Entity) {
        self.retain(|&id| id != entity);
    }
}
