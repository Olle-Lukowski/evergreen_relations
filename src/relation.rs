use crate::container::EntityContainer;

pub use evergreen_relations_macros::{Relatable, Relation};

/// Trait for types that represent a relationship between entities.
pub trait Relation {
    /// The "source" node of the relation.
    type Source: Relatable<Relation = Self, Opposite = Self::Target>;

    /// The "target" node of the relation.
    type Target: Relatable<Relation = Self, Opposite = Self::Source>;
}

/// Trait for types that represent a node in a relationship.
pub trait Relatable: Sized + 'static {
    /// The relation type that this node is part of.
    type Relation: Relation;

    /// The opposite side of this node's [`Relation`].
    type Opposite: Relatable<Relation = Self::Relation, Opposite = Self>;

    /// The [`Symmetry`] of this node as it corresponds to its [`Relation`].
    type Symmetry: Symmetry<Self>;

    /// The container type that holds the related entities.
    type Container: EntityContainer;
}

pub trait Symmetry<N: Relatable>: sealed::Symmetry<N> {
    const SYMMETRIC: bool;
}

pub struct Symmetric;

impl<R: Relation<Source = N, Target = N>, N: Relatable<Relation = R, Opposite = N>> Symmetry<N>
    for Symmetric
{
    const SYMMETRIC: bool = true;
}

pub struct Asymmetric;

impl<N: Relatable> Symmetry<N> for Asymmetric {
    const SYMMETRIC: bool = false;
}

mod sealed {
    pub trait Symmetry<N: super::Relatable> {}
    impl<R: super::Relation<Source = N, Target = N>, N: super::Relatable<Relation = R>> Symmetry<N>
        for super::Symmetric
    {
    }
    impl<N: super::Relatable> Symmetry<N> for super::Asymmetric {}
}
