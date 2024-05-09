// Copyright 2024 Golem Cloud
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::generate_for_tuples;

pub fn call_compensation_function<In, Out, Err>(
    f: impl CompensationFunction<In, Out, Err>,
    result: impl TupleOrUnit<Out>,
    input: impl TupleOrUnit<In>,
) -> Result<(), Err> {
    f.call(result, input)
}

pub trait TupleOrUnit<T> {
    fn into(self) -> T;
}

pub trait CompensationFunction<In, Out, Err> {
    fn call(self, result: impl TupleOrUnit<Out>, input: impl TupleOrUnit<In>) -> Result<(), Err>;
}

impl<F, Err> CompensationFunction<(), (), (Err,)> for F
where
    F: FnOnce() -> Result<(), Err>,
{
    fn call(
        self,
        _result: impl TupleOrUnit<()>,
        _input: impl TupleOrUnit<()>,
    ) -> Result<(), (Err,)> {
        self().map_err(|e| (e,))?;
        Ok(())
    }
}

impl<F, Out, Err> CompensationFunction<(), (Out,), (Err,)> for F
where
    F: FnOnce(Out) -> Result<(), Err>,
{
    fn call(
        self,
        out: impl TupleOrUnit<(Out,)>,
        _input: impl TupleOrUnit<()>,
    ) -> Result<(), (Err,)> {
        let (out,) = out.into();
        self(out).map_err(|err| (err,))
    }
}

impl<T> TupleOrUnit<()> for T {
    fn into(self) {}
}

macro_rules! compensation_function {
    ($($ty:ident),*) => {
        impl<F, $($ty),*, Out, Err> CompensationFunction<($($ty),*,), (Out,), (Err,)> for F
        where
            F: FnOnce(Out, $($ty),*) -> Result<(), Err>,
        {
            fn call(
                self,
                out: impl TupleOrUnit<(Out,)>,
                input: impl TupleOrUnit<($($ty),*,)>,
            ) -> Result<(), (Err,)> {
                #[allow(non_snake_case)]
                let ( $($ty,)+ ) = input.into();
                let (out,) = out.into();
                self(out, $($ty),*).map_err(|err| (err,))
            }
        }
    }
}

macro_rules! tuple_or_unit {
    ($($ty:ident),*) => {
        impl<$($ty),*> TupleOrUnit<($($ty,)*)> for ($($ty,)*) {
            fn into(self) -> ($($ty,)*) {
                self
            }
        }
    }
}

generate_for_tuples!(tuple_or_unit);
generate_for_tuples!(compensation_function);
