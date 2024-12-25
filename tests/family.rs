use bevy_ecs::{entity::Entity, world::World};
use evergreen_relations::prelude::*;
use smallvec::SmallVec;

/// A directed 1:N relationship between entities.
#[derive(Relation)]
#[relation(source = ChildOf, target = ParentOf)]
pub struct Family;

pub type Parent = Related<ChildOf>;

#[derive(Relatable, Clone, PartialEq, Eq, Debug)]
#[relatable(Entity in Family, opposite = ParentOf)]
pub struct ChildOf;

pub type Children = Related<ParentOf>;

#[derive(Relatable, Clone, PartialEq, Eq, Debug)]
#[relatable(SmallVec<[Entity; 8]> in Family, opposite = ChildOf)]
pub struct ParentOf;

#[test]
fn add_remove() {
    let mut world = World::new();

    let a = world.spawn_empty().id();
    let b = world.spawn(Parent::new(a)).id();
    let c = world.spawn(Parent::new(a)).id();

    world.flush();

    assert_eq!(world.get::<Parent>(a), None);
    assert_eq!(world.get::<Parent>(b), Some(&Parent::new(a)));
    assert_eq!(world.get::<Parent>(c), Some(&Parent::new(a)));

    assert_eq!(
        world
            .get::<Children>(a)
            .map(|children| children.iter().collect::<Vec<_>>()),
        Some(vec![b, c])
    );
    assert_eq!(world.get::<Children>(b), None);
    assert_eq!(world.get::<Children>(c), None);

    world.entity_mut(b).remove::<Parent>();

    world.flush();

    assert_eq!(
        world
            .get::<Children>(a)
            .map(|children| children.iter().collect::<Vec<_>>()),
        Some(vec![c])
    );

    world.entity_mut(a).remove::<Children>();

    world.flush();

    assert_eq!(world.get::<Children>(a), None);
    assert_eq!(world.get::<Parent>(b), None);
    assert_eq!(world.get::<Parent>(c), None);
}
