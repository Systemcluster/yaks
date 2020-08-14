use super::{AtomicBorrow, ResourceCellMut, ResourceCellRef};

trait WrappableSingle {
    type Wrapped: Send + Sync;

    fn wrap(self, borrow: &mut AtomicBorrow) -> Self::Wrapped;
}

impl<R0> WrappableSingle for &'_ R0
where
    R0: Send + Sync,
{
    type Wrapped = ResourceCellRef<R0>;

    fn wrap(self, borrow: &mut AtomicBorrow) -> Self::Wrapped {
        ResourceCellRef::new(self, borrow)
    }
}

impl<R0> WrappableSingle for &'_ mut R0
where
    R0: Send + Sync,
{
    type Wrapped = ResourceCellMut<R0>;

    fn wrap(self, borrow: &mut AtomicBorrow) -> Self::Wrapped {
        ResourceCellMut::new(self, borrow)
    }
}

pub trait Wrappable {
    type Wrapped: Send + Sync;
    type BorrowTuple: Send + Sync;

    fn wrap(self, borrows: &mut Self::BorrowTuple) -> Self::Wrapped;
}

impl<R0> Wrappable for &'_ R0
where
    R0: Send + Sync,
{
    type Wrapped = (ResourceCellRef<R0>,);
    type BorrowTuple = (AtomicBorrow,);

    fn wrap(self, borrows: &mut Self::BorrowTuple) -> Self::Wrapped {
        (ResourceCellRef::new(self, &mut borrows.0),)
    }
}

impl<R0> Wrappable for &'_ mut R0
where
    R0: Send + Sync,
{
    type Wrapped = (ResourceCellMut<R0>,);
    type BorrowTuple = (AtomicBorrow,);

    fn wrap(self, borrows: &mut Self::BorrowTuple) -> Self::Wrapped {
        (ResourceCellMut::new(self, &mut borrows.0),)
    }
}

impl Wrappable for () {
    type Wrapped = ();
    type BorrowTuple = ();

    fn wrap(self, _: &mut Self::BorrowTuple) -> Self::Wrapped {}
}

impl<R0> Wrappable for (R0,)
where
    R0: WrappableSingle,
{
    type Wrapped = (R0::Wrapped,);
    type BorrowTuple = (AtomicBorrow,);

    fn wrap(self, borrows: &mut Self::BorrowTuple) -> Self::Wrapped {
        (self.0.wrap(&mut borrows.0),)
    }
}

impl<R0, R1> Wrappable for (R0, R1)
where
    R0: WrappableSingle,
    R1: WrappableSingle,
{
    type Wrapped = (R0::Wrapped, R1::Wrapped);
    type BorrowTuple = (AtomicBorrow, AtomicBorrow);

    fn wrap(self, borrows: &mut Self::BorrowTuple) -> Self::Wrapped {
        (self.0.wrap(&mut borrows.0), self.1.wrap(&mut borrows.1))
    }
}

/*macro_rules! swap_to_atomic_borrow {
    ($anything:tt) => {
        AtomicBorrow
    };
    (new $anything:tt) => {
        AtomicBorrow::new()
    };
}

macro_rules! impl_resource_wrap {
    ($($letter:ident),*) => {
        paste::item! {
            impl<$($letter),*> ResourceWrap for ($(&'_ mut $letter,)*)
            where
                $($letter: Send + Sync,)*
            {
                type Wrapped = ($(ResourceCell<$letter>,)*);
                type BorrowTuple = ($(swap_to_atomic_borrow!($letter),)*);

                #[allow(non_snake_case)]
                fn wrap(&mut self, borrows: &mut Self::BorrowTuple) -> Self::Wrapped {
                    let ($([<S $letter>],)*) = self;
                    let ($([<B $letter>],)*) = borrows;
                    ($( ResourceCell::new([<S $letter>], [<B $letter>]) ,)*)
                }
            }
        }
    }
}

impl_for_tuples!(impl_resource_wrap);*/
