use bevy::prelude::*;
//use bevy_retrograde::prelude::*;

/*pub struct map_manager {
    map_raw_file: Vec2,
    // bloc_list: Vec<Vec<bloc::Bloc,
impl map_manager {
    pub fn new() -> Self {
    }
    pub fn load_map(&mut self) {}
}*/

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(WindowDescriptor {
            title: "Rusty caves".into(),
            ..Default::default()
        });
    }
}
