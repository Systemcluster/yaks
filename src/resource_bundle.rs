use std::{
    any::{type_name, TypeId},
    marker::PhantomData,
};

use crate::{Resource, ResourceRef, ResourceRefMut, SystemBorrows, World};

pub struct Immutable;

pub struct Mutable;

pub trait Mutability: Send + Sync {}

impl Mutability for Immutable {}

impl Mutability for Mutable {}

pub struct ResourceEffector<M, R>
where
    M: Mutability,
    R: Resource + Send + Sync,
{
    phantom_data: PhantomData<(M, R)>,
}

impl<M, R> ResourceEffector<M, R>
where
    M: Mutability,
    R: Resource + Send + Sync,
{
    pub(crate) fn new() -> Self {
        Self {
            phantom_data: PhantomData,
        }
    }
}

pub trait ResourceSingle: Send + Sync {
    type Effector;

    fn effector() -> Self::Effector;

    fn write_borrows(borrows: &mut SystemBorrows);
}

impl<R> ResourceSingle for &'_ R
where
    R: Resource,
{
    type Effector = ResourceEffector<Immutable, R>;

    fn effector() -> Self::Effector {
        ResourceEffector::new()
    }

    fn write_borrows(borrows: &mut SystemBorrows) {
        borrows.resources_immutable.insert(TypeId::of::<R>());
    }
}

impl<R> ResourceSingle for &'_ mut R
where
    R: Resource,
{
    type Effector = ResourceEffector<Mutable, R>;

    fn effector() -> Self::Effector {
        ResourceEffector::new()
    }

    fn write_borrows(borrows: &mut SystemBorrows) {
        borrows.resources_mutable.insert(TypeId::of::<R>());
    }
}

pub trait ResourceBundle: Send + Sync {
    type Effectors;

    fn effectors() -> Self::Effectors;

    fn write_borrows(borrows: &mut SystemBorrows);
}

impl ResourceBundle for () {
    type Effectors = ();

    fn effectors() -> Self::Effectors {}

    fn write_borrows(_: &mut SystemBorrows) {}
}

impl<R> ResourceBundle for R
where
    R: ResourceSingle,
{
    type Effectors = R::Effector;

    fn effectors() -> Self::Effectors {
        R::effector()
    }

    fn write_borrows(borrows: &mut SystemBorrows) {
        R::write_borrows(borrows)
    }
}

pub trait Fetch<'a> {
    type Refs;

    fn fetch(&self, world: &'a World) -> Self::Refs;
}

impl<'a> Fetch<'a> for () {
    type Refs = ();

    fn fetch(&self, _: &'a World) -> Self::Refs {}
}

impl<'a, R> Fetch<'a> for ResourceEffector<Immutable, R>
where
    R: Resource,
{
    type Refs = ResourceRef<'a, R>;

    fn fetch(&self, world: &'a World) -> Self::Refs {
        world
            .resource()
            .unwrap_or_else(|error| panic!("cannot fetch {}: {}", type_name::<R>(), error))
    }
}

impl<'a, R> Fetch<'a> for ResourceEffector<Mutable, R>
where
    R: Resource,
{
    type Refs = ResourceRefMut<'a, R>;

    fn fetch(&self, world: &'a World) -> Self::Refs {
        world
            .resource_mut()
            .unwrap_or_else(|error| panic!("cannot fetch {}: {}", type_name::<R>(), error))
    }
}
