use bevy_ecs::{entity::Entity, world::World};
use evergreen_relations::{
    related::Related,
    relation::{Relatable, Relation},
};
use smallvec::SmallVec;

/// An undirected N:M relationship between entities.
#[derive(Relation)]
#[relation(source = FriendOf, target = FriendOf)]
pub struct Friendship;

pub type Friend = Related<FriendOf>;

#[derive(Relatable)]
#[relatable(SmallVec<[Entity; 8]> in Friendship, opposite = Self)]
pub struct FriendOf;

#[test]
fn add_remove() {
    let mut world = World::new();

    let a = world.spawn_empty().id();
    let b = world.spawn(Friend::from_iter([a])).id();
    let c = world.spawn(Friend::from_iter([a, b])).id();

    world.flush();

    assert_eq!(world.get::<Friend>(a), Some(&Friend::from_iter([b, c])));
    assert_eq!(world.get::<Friend>(b), Some(&Friend::from_iter([a, c])));
    assert_eq!(world.get::<Friend>(c), Some(&Friend::from_iter([a, b])));

    world.entity_mut(b).remove::<Friend>();

    world.flush();

    assert_eq!(world.get::<Friend>(a), Some(&Friend::from_iter([c])));
    assert_eq!(world.get::<Friend>(b), None);
    assert_eq!(world.get::<Friend>(c), Some(&Friend::from_iter([a])));
}
