use bevy_ecs::entity::Entity;
use smallvec::SmallVec;

use evergreen_relations::{
    related::Related,
    relation::{Relatable, Relation},
};

/// A directed 1:N relationship between entities.
#[derive(Relation)]
#[relation(source = ChildOf, target = ParentOf)]
pub struct Family;

pub type Parent = Related<ChildOf>;

#[derive(Relatable, Clone, PartialEq, Eq, Debug)]
#[relatable(Family, opposite = ParentOf)]
pub struct ChildOf(Entity);

pub type Children = Related<ParentOf>;

#[derive(Relatable, Clone, PartialEq, Eq, Debug)]
#[relatable(Family, opposite = ChildOf)]
pub struct ParentOf(SmallVec<[Entity; 8]>);

fn main() {}
