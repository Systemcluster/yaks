// TODO uncomment #![warn(missing_docs)]

pub use hecs::{
    Bundle as ComponentBundle, Component, ComponentError, DynamicBundle as DynamicComponentBundle,
    Entity, EntityRef as Components, MissingComponent, NoSuchEntity, Query, QueryBorrow,
    Ref as ComponentRef, RefMut as ComponentRefMut,
};
pub use resources::{
    CantGetResource as ResourceError, Entry as ResourceEntry, NoSuchResource, Ref as ResourceRef,
    RefMut as ResourceRefMut, Resource,
};

mod borrows;
mod executor;
#[allow(dead_code)]
mod executor_arch_disjoint;
mod impls_for_tuple;
mod query_bundle;
mod resource_bundle;
mod system;
mod world;

pub use borrows::SystemBorrows;
pub use executor::Executor;
pub use system::{System, SystemBuilder};
pub use world::World;
