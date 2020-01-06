use std::{any::TypeId, marker::PhantomData};

use crate::{metadata::ArchetypeSet, Component, Query, QueryBorrow, SystemMetadata, World};

pub struct QueryEffector<Q>
where
    Q: Query + Send + Sync,
{
    phantom_data: PhantomData<Q>,
}

impl<Q> QueryEffector<Q>
where
    Q: Query + Send + Sync,
{
    pub(crate) fn new() -> Self {
        Self {
            phantom_data: PhantomData,
        }
    }

    pub fn query<'a>(&self, world: &'a World) -> QueryBorrow<'a, Q> {
        world.query()
    }
}

pub trait QuerySingle: Send + Sync {
    type Effector;

    fn effector() -> Self::Effector;

    fn write_metadata(metadata: &mut SystemMetadata);

    fn write_touched_archetypes(world: &World, set: &mut ArchetypeSet);
}

impl<C> QuerySingle for &'_ C
where
    C: Component,
{
    type Effector = QueryEffector<Self>;

    fn effector() -> Self::Effector {
        QueryEffector::new()
    }

    fn write_metadata(metadata: &mut SystemMetadata) {
        metadata.components_immutable.insert(TypeId::of::<C>());
    }

    fn write_touched_archetypes(world: &World, set: &mut ArchetypeSet) {
        world.write_touched_archetypes::<Self>(set);
    }
}

impl<C> QuerySingle for &'_ mut C
where
    C: Component,
{
    type Effector = QueryEffector<Self>;

    fn effector() -> Self::Effector {
        QueryEffector::new()
    }

    fn write_metadata(metadata: &mut SystemMetadata) {
        metadata.components_mutable.insert(TypeId::of::<C>());
    }

    fn write_touched_archetypes(world: &World, set: &mut ArchetypeSet) {
        world.write_touched_archetypes::<Self>(set);
    }
}

impl<Q> QuerySingle for Option<Q>
where
    Q: QuerySingle,
    Self: Query,
{
    type Effector = QueryEffector<Self>;

    fn effector() -> Self::Effector {
        QueryEffector::new()
    }

    fn write_metadata(metadata: &mut SystemMetadata) {
        Q::write_metadata(metadata);
    }

    fn write_touched_archetypes(world: &World, set: &mut ArchetypeSet) {
        world.write_touched_archetypes::<Self>(set);
    }
}

pub trait QueryBundle: Send + Sync {
    type Effectors;

    fn effectors() -> Self::Effectors;

    fn write_metadata(metadata: &mut SystemMetadata);

    fn write_touched_archetypes(world: &World, set: &mut ArchetypeSet);
}

impl<C> QueryBundle for &'_ C
where
    C: Component,
    Self: QuerySingle,
{
    type Effectors = <Self as QuerySingle>::Effector;

    fn effectors() -> Self::Effectors {
        <Self as QuerySingle>::effector()
    }

    fn write_metadata(metadata: &mut SystemMetadata) {
        <Self as QuerySingle>::write_metadata(metadata);
    }

    fn write_touched_archetypes(world: &World, set: &mut ArchetypeSet) {
        <Self as QuerySingle>::write_touched_archetypes(world, set);
    }
}

impl<C> QueryBundle for &'_ mut C
where
    C: Component,
    Self: QuerySingle,
{
    type Effectors = <Self as QuerySingle>::Effector;

    fn effectors() -> Self::Effectors {
        <Self as QuerySingle>::effector()
    }

    fn write_metadata(metadata: &mut SystemMetadata) {
        <Self as QuerySingle>::write_metadata(metadata);
    }

    fn write_touched_archetypes(world: &World, set: &mut ArchetypeSet) {
        <Self as QuerySingle>::write_touched_archetypes(world, set);
    }
}

impl<Q> QueryBundle for Option<Q>
where
    Q: QuerySingle,
    Self: QuerySingle,
{
    type Effectors = <Self as QuerySingle>::Effector;

    fn effectors() -> Self::Effectors {
        <Self as QuerySingle>::effector()
    }

    fn write_metadata(metadata: &mut SystemMetadata) {
        <Self as QuerySingle>::write_metadata(metadata);
    }

    fn write_touched_archetypes(world: &World, set: &mut ArchetypeSet) {
        <Self as QuerySingle>::write_touched_archetypes(world, set);
    }
}

impl QueryBundle for () {
    type Effectors = ();

    fn effectors() -> Self::Effectors {}

    fn write_metadata(_: &mut SystemMetadata) {}

    fn write_touched_archetypes(_: &World, _: &mut ArchetypeSet) {}
}
