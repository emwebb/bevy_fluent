use bevy::prelude::*;
use bevy_fluent::{components::Snapshot, utils::BundleExt, FluentPlugin, FluentSettings, Request};
use unic_langid::langid;

pub fn main() {
    App::build()
        .insert_resource(FluentSettings::default().with_default_locale(langid!("ru-RU")))
        .add_plugins(DefaultPlugins)
        .add_plugin(FluentPlugin)
        .add_system(localized_hello_world.system())
        .run();
}

// [Bevy hello world example](https://github.com/bevyengine/bevy/blob/main/examples/hello_world.rs)
fn localized_hello_world(
    fluent_settings: Res<FluentSettings>,
    snapshot: Option<Res<Snapshot>>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    if let Some(snapshot) = snapshot {
        let request = Request::builder().id("hello-world").build();
        let hello_world = snapshot[&fluent_settings.default_locale]
            .content(&request)
            .unwrap();
        println!("{}", hello_world);
        *done = true;
    }
}
