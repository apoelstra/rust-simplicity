// SPDX-License-Identifier: CC0-1.0

use core::convert::Infallible;
use core::marker::PhantomData;

pub struct CoreEnv<T>(PhantomData<T>);

impl CoreEnv<Infallible> {
    pub const EMPTY: Self = Self(PhantomData);
}
