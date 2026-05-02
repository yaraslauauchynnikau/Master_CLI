use std::marker::PhantomData;

trait HasParameters {
    type Parameters;
}

pub struct CommandInternals<T: HasParameters> {
    parameters: T,
    _marker: PhantomData<T>
}

impl<T: HasParameters + Default> CommandInternals<T> {
    pub fn new() -> Self {
        Self {
            parameters: T::default(),
            _marker: PhantomData,
        }
    }
}

trait Command<T: HasParameters> {
    fn execute();
}