use crate::container::EntityContainer;

pub use evergreen_relations_macros::{Relatable, Relation};

/// Trait for types that represent a relationship between entities.
///
/// Entity pointer data is stored in the [`Related`] component.
///
/// [`Related`]: crate::related::Related
pub trait Relation {
    /// The "source" node of the relation.
    type Source: Relatable<Relation = Self, Opposite = Self::Target>;

    /// The "target" node of the relation.
    type Target: Relatable<Relation = Self, Opposite = Self::Source>;
}

/// Trait for types that represent a node in a relationship.
///
/// Entity pointer data is stored in the [`Related`] component.
///
/// [`Related`]: crate::related::Related
pub trait Relatable: 'static {
    /// The relation type that this node is part of.
    type Relation: Relation;

    /// The opposite side of this node's [`Relation`].
    type Opposite: Relatable<Relation = Self::Relation, Opposite = Self>;

    /// The container type that holds the related entities.
    type Container: EntityContainer;
}
