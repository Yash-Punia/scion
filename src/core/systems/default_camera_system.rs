use crate::core::components::maths::{
    camera::{Camera, DefaultCamera},
    transform::Transform,
};
use crate::core::world::{GameData, World};

/// System responsible of adding a Camera on each entity with a DefaultCamera component
pub(crate) fn default_camera_system(data: &mut GameData) {
    let default_entity =
        data.query::<&DefaultCamera>().without::<&Camera>().iter().map(|(e, _d)| e).next();

    let (subworld, resources) = data.split();

    if let Some(e) = default_entity {
        let window = resources.window();
        let mut camera = Camera::new(
            window.width() as f32,
            window.height() as f32,
        );
        camera.dpi = window.dpi();
        let _r = subworld.add_components(e, (camera,));
        let _r = subworld.add_components(e, (Transform::default(),));
    }
}

/// System responsible of applying dpi to each camera
pub(crate) fn camera_dpi_system(data: &mut GameData) {
    let (subworld, resources) = data.split();
    let window = resources.window();
    for (_, camera) in subworld.query_mut::<&mut Camera>() {
        camera.dpi = window.dpi();
    }
}
