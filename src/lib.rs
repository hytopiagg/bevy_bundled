use bevy::prelude::*;
pub use bevy_bundled_derive::*;

/// Represents a struct containing multiple `Resource`s
pub trait ResourceBundle {
    fn insert_self_app(&self, app: &mut App);
    fn insert_self_commands(&self, app: &mut Commands);
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for bevy::prelude::App {}
    impl Sealed for bevy::prelude::Commands<'_, '_> {}
}

pub trait AppExtension: sealed::Sealed {
    fn insert_resource_bundle<T: ResourceBundle>(&mut self, t: T) -> &mut Self;
    fn init_resource_bundle<T: ResourceBundle + Default>(&mut self) -> &mut Self {
        self.insert_resource_bundle(T::default());
        self
    }
}

impl AppExtension for App {
    fn insert_resource_bundle<T: ResourceBundle>(&mut self, t: T) -> &mut Self {
        t.insert_self_app(self);
        self
    }
}

impl AppExtension for Commands<'_, '_> {
    fn insert_resource_bundle<T: ResourceBundle>(&mut self, t: T) -> &mut Self {
        t.insert_self_commands(self);
        self
    }
}
