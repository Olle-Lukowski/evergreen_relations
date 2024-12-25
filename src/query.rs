use bevy_ecs::query::{QueryData, ReadOnlyQueryData};

use crate::{prelude::Relation, related::Related};

/// [`QueryData`] wrapper for fetching the sides of a [`Relation`].
///
/// The `S` and `T` type parameters are [`Selector`]s that determine whether the
/// `Source` and `Target` sides of the relation are required, optional, or not
/// fetched at all, respectively.
#[derive(QueryData)]
pub struct SelectRelated<R: Relation, S: Selector, T: Selector> {
    /// The source side of the relation.
    pub source: S::Item<&'static Related<R::Source>>,
    /// The target side of the relation.
    pub target: T::Item<&'static Related<R::Target>>,
}

/// A [`SelectRelated`] variant that fetches both sides of the relation as-is,
/// requiring both to be present.
pub type BothRelated<R> = SelectRelated<R, Required, Required>;

/// A [`SelectRelated`] variant that fetches both sides of the relation as
/// [`Option`]s, allowing either or both to be absent.
pub type EitherRelated<R> = SelectRelated<R, Optional, Optional>;

/// A trait providing a type function that determines the type of the item
/// fetched by a [`SelectRelated`] query.
pub trait Selector {
    /// A type function that determines the type of the item fetched by the
    /// selector.
    type Item<D: ReadOnlyQueryData>: ReadOnlyQueryData;
}

/// A [`Selector`] that fetches the item as-is, requiring it to be present.
pub struct Required;

impl Selector for Required {
    type Item<D: ReadOnlyQueryData> = D;
}

/// A [`Selector`] that fetches the item as an [`Option`].
pub struct Optional;

impl Selector for Optional {
    type Item<D: ReadOnlyQueryData> = Option<D>;
}

/// A [`Selector`] that fetches no item at all.
pub struct Nothing;

impl Selector for Nothing {
    type Item<D: ReadOnlyQueryData> = ();
}
