pub mod container;
pub mod event;
pub mod related;
pub mod relation;

pub mod prelude {
    //! Re-exports the most commonly used traits and types.

    pub use crate::{
        container::EntityContainer,
        event::RelationEvent,
        related::Related,
        relation::{Relatable, Relation},
    };
}
