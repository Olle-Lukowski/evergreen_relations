pub mod container;
pub mod event;
pub mod query;
pub mod related;
pub mod relation;

pub mod prelude {
    //! Re-exports the most commonly used traits and types.

    pub use crate::{
        container::EntityContainer,
        event::RelationEvent,
        query::{BothRelated, EitherRelated, SelectRelated, SelectRelatedItem},
        related::Related,
        relation::{Relatable, Relation},
    };
}
