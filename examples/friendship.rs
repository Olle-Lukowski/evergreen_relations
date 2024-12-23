use bevy_ecs::entity::Entity;
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

#[derive(Relatable, Clone, PartialEq, Eq, Debug)]
#[relatable(Friendship, opposite = Self)]
pub struct FriendOf(SmallVec<[Entity; 8]>);

fn main() {}
