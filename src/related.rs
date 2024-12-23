use std::marker::PhantomData;

use bevy_ecs::{
    component::{Component, ComponentHooks, ComponentId, Immutable, StorageType},
    entity::Entity,
    event::Events,
    world::{DeferredWorld, World},
};

use crate::{container::EntityContainer, event::RelationEvent, relation::Relatable};

/// [`Component`] used to store [`Relation`] data for a given side of a relationship,
/// i.e. the [`Relatable`].
///
/// [`Relation`]: crate::relation::Relation
pub struct Related<N: Relatable> {
    pub(crate) container: N::Container,
}

impl<N: Relatable> Component for Related<N> {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Immutable;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(associate::<N>);
        hooks.on_replace(disassociate::<N>);
        hooks.on_remove(disassociate::<N>);
    }
}

impl<N: Relatable> Related<N> {
    pub fn new(node: impl Into<N::Container>) -> Self {
        Self {
            container: node.into(),
        }
    }

    pub fn as_slice(&self) -> &[Entity] {
        self.container.as_slice()
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.container.contains(entity)
    }
}

impl<N: Relatable> Clone for Related<N> {
    fn clone(&self) -> Self {
        Self {
            container: self.container.clone(),
        }
    }
}

impl<N: Relatable> PartialEq for Related<N> {
    fn eq(&self, other: &Self) -> bool {
        self.container == other.container
    }
}

impl<N: Relatable> Eq for Related<N> {}

impl<N: Relatable> std::fmt::Debug for Related<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Related")
            .field("container", &self.container)
            .finish()
    }
}

impl<N: Relatable> From<Entity> for Related<N> {
    fn from(entity: Entity) -> Self {
        Self {
            container: N::Container::new(entity),
        }
    }
}

fn associate<N: Relatable>(mut world: DeferredWorld, a_id: Entity, _: ComponentId) {
    world.commands().queue(move |world: &mut World| {
        // Get the IDs of the entities that this entity is related to.
        let Some(b_ids) = world.get::<Related<N>>(a_id).cloned() else {
            return;
        };

        // For each related entity, associate it with this entity.
        for &b_id in b_ids.as_slice() {
            let Ok(mut b) = world.get_entity_mut(b_id) else {
                continue;
            };

            let b_related = unsafe { b.get_mut_assume_mutable::<Related<N::Opposite>>() };

            let b_points_to_a = b_related.as_ref().is_some_and(|b| b.contains(a_id));
            if !b_points_to_a {
                if let Some(mut b_related) = b_related {
                    // The other entity is already related to some entities, so add this entity to the list.
                    b_related.container.push(a_id);
                } else {
                    // The other entity is not yet related to any entities, so relate it to this entity.
                    b.insert(Related::<N::Opposite>::from(a_id));
                }

                if let Some(mut events) =
                    world.get_resource_mut::<Events<RelationEvent<N::Relation>>>()
                {
                    events.send(RelationEvent::Added(a_id, b_id, PhantomData));
                }
            }
        }
    });
}

fn disassociate<N: Relatable>(mut world: DeferredWorld, a_id: Entity, _: ComponentId) {
    // Gets the IDs of the entities that this entity is no longer related to.
    let Some(b_ids) = world.get::<Related<N>>(a_id).cloned() else {
        return;
    };

    world.commands().queue(move |world: &mut World| {
        // For each related entity, disassociate it from this entity.
        for &b_id in b_ids.as_slice() {
            let Ok(mut b) = world.get_entity_mut(b_id) else {
                continue;
            };

            let b_related = unsafe { b.get_mut_assume_mutable::<Related<N::Opposite>>() };

            let b_points_to_a = b_related.as_ref().is_some_and(|b| b.contains(a_id));
            if b_points_to_a {
                if let Some(mut b_related) = b_related {
                    // The other entity is related to some entities, so make sure this entity is removed from the list.
                    b_related.container.remove(a_id);

                    // If the other entity is no longer related to any entities, remove the component.
                    if b_related.container.is_empty() {
                        b.remove::<Related<N::Opposite>>();
                    }
                }

                if let Some(mut events) =
                    world.get_resource_mut::<Events<RelationEvent<N::Relation>>>()
                {
                    events.send(RelationEvent::Removed(a_id, b_id, PhantomData));
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use bevy_ecs::{entity::Entity, world::World};

    use crate::{
        related::Related,
        relation::{Relatable, Relation},
    };

    pub struct Symmetric1t1;

    impl Relation for Symmetric1t1 {
        type Source = SymmetricNode;
        type Target = SymmetricNode;
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct SymmetricNode(Entity);

    pub type S1T1 = Related<SymmetricNode>;

    impl Relatable for SymmetricNode {
        type Relation = Symmetric1t1;
        type Opposite = Self;
        type Container = Entity;
    }

    #[test]
    fn symmetric_one_to_one() {
        let mut world = World::new();

        let a = world.spawn_empty().id();
        let b = world.spawn(S1T1::new(a)).id();

        world.flush();

        assert_eq!(world.get::<S1T1>(a), Some(&S1T1::new(b)));
        assert_eq!(world.get::<S1T1>(b), Some(&S1T1::new(a)));

        world.entity_mut(b).remove::<S1T1>();
        world.flush();

        assert_eq!(world.get::<S1T1>(a), None);
        assert_eq!(world.get::<S1T1>(b), None);
    }
}
