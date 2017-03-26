#[macro_use]
extern crate anterofit;

use anterofit::{JsonAdapter};
use anterofit::net::response::{TryWithRaw};
use std::marker::PhantomData;

pub struct Foo<T> {
    phantom: PhantomData<T>,
    adapter: JsonAdapter,
}


service! {
    pub trait MyService[T] {
        fn get_next(&self) -> TryWithRaw<Foo<T>> {
            GET("")
        }
    }

    impl for Foo<T> {
        |this| &this.adapter
    }

    impl[U] for U [where U: AsRef<Foo<T>>] {
        |this| &this.as_ref().adapter
    }
}
