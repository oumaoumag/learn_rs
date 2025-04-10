pub mod areas_volumes;
pub use areas_volumes::{GeometricalShapes, GeometricalVolumes};


pub fn area_fit(
    x: usize,
    y: usize,
    objects: areas_volumes::GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    // The area of the rectanglar space
    let space_area = x * y;
    
    // Area of a single object based on its tyoe
    let single_object_area = match objects {
        areas_volumes::GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        areas_volumes::GeometricalShapes::Circle => areas_volumes::circle_area(a),
        areas_volumes::GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) as f64,
        areas_volumes::GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b),
    };

    // The total area needed for all objects
    let total_objects_area = single_object_area * times as f64;

    // Check if the objects can fit in the space
    total_objects_area <= space_area as f64 
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    // Volume of the box
    let box_volume = x * y * z;

    // Volume of a single object based on its type
    let single_object_volume = match objects {
        areas_volumes::GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        areas_volumes::GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
        areas_volumes::GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
        areas_volumes::GeometricalVolumes::Pyramid => areas_volumes::triangular_pyramid_volume( a as f64, b),
        areas_volumes::GeometricalVolumes::Parallelepiped => areas_volumes:: parallelepiped_volume(a, b, c) as f64,
    };

    // Total volume needed for all abjects
    let total_objects_volume = single_object_volume * times as f64;

    // check if the objects can fit in the box
    total_objects_volume <= box_volume as f64
}