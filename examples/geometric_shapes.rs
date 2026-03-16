//! Geometric shapes example
//!
//! Run with: cargo run --example geometric_shapes

use dodecet_encoder::geometric::{Point3D, Vector3D, Transform3D, shapes::{Triangle, Box3D}};

fn main() {
    println!("=== Geometric Shapes with Dodecet Encoding ===\n");

    // Working with 3D points
    println!("1. 3D Points:");
    let origin = Point3D::new(0x000, 0x000, 0x000);
    let p1 = Point3D::new(0x100, 0x200, 0x300);
    let p2 = Point3D::new(0x400, 0x500, 0x600);

    println!("   Origin: ({}, {}, {})", origin.x(), origin.y(), origin.z());
    println!("   Point 1: ({}, {}, {})", p1.x(), p1.y(), p1.z());
    println!("   Point 2: ({}, {}, {})", p2.x(), p2.y(), p2.z());

    let dist = p1.distance_to(&p2);
    println!("   Distance: {:.2}\n", dist);

    // Working with vectors
    println!("2. 3D Vectors:");
    let v1 = Vector3D::new(100, 200, 300);
    let v2 = Vector3D::new(400, 500, 600);

    println!("   Vector 1: ({}, {}, {})", v1.x(), v1.y(), v1.z());
    println!("   Vector 2: ({}, {}, {})", v2.x(), v2.y(), v2.z());

    let dot = v1.dot(&v2);
    println!("   Dot product: {}", dot);

    let cross = v1.cross(&v2);
    println!("   Cross product: ({}, {}, {})", cross.x(), cross.y(), cross.z());

    let mag = v1.magnitude();
    println!("   Magnitude of v1: {:.2}\n", mag);

    // Triangle
    println!("3. Triangle:");
    let triangle = Triangle::new(origin, p1, p2);
    let area = triangle.area();
    println!("   Vertices:");
    for (i, v) in triangle.vertices().iter().enumerate() {
        println!("     v{}: ({}, {}, {})", i, v.x(), v.y(), v.z());
    }
    println!("   Area: {:.2}\n", area);

    // 3D Box
    println!("4. 3D Box:");
    let min = Point3D::new(0x000, 0x000, 0x000);
    let max = Point3D::new(0x100, 0x100, 0x100);
    let box3d = Box3D::new(min.clone(), max.clone());

    println!("   Min corner: ({}, {}, {})", min.x(), min.y(), min.z());
    println!("   Max corner: ({}, {}, {})", max.x(), max.y(), max.z());

    let volume = box3d.volume();
    println!("   Volume: {:.2}", volume);

    let test_point = Point3D::new(0x080, 0x080, 0x080);
    println!("   Point ({}, {}, {}) inside: {}",
        test_point.x(), test_point.y(), test_point.z(),
        box3d.contains(&test_point)
    );

    let outside_point = Point3D::new(0x200, 0x080, 0x080);
    println!("   Point ({}, {}, {}) inside: {}\n",
        outside_point.x(), outside_point.y(), outside_point.z(),
        box3d.contains(&outside_point)
    );

    // Transformations
    println!("5. Transformations:");
    let point = Point3D::new(0x100, 0x200, 0x300);

    let identity = Transform3D::identity();
    let transformed = identity.apply(&point);
    println!("   Identity: ({}, {}, {}) -> ({}, {}, {})",
        point.x(), point.y(), point.z(),
        transformed.x(), transformed.y(), transformed.z()
    );

    let translation = Transform3D::translation(100, 200, 300);
    let translated = translation.apply(&point);
    println!("   Translation: ({}, {}, {}) -> ({}, {}, {})",
        point.x(), point.y(), point.z(),
        translated.x(), translated.y(), translated.z()
    );

    let scale = Transform3D::scale(2.0, 2.0, 2.0);
    let scaled = scale.apply(&point);
    println!("   Scale 2x: ({}, {}, {}) -> ({}, {}, {})",
        point.x(), point.y(), point.z(),
        scaled.x(), scaled.y(), scaled.z()
    );

    let rotation = Transform3D::rotation_z(90.0);
    let rotated = rotation.apply(&Point3D::new(0x100, 0x000, 0x000));
    println!("   Rotate 90° around Z: ({}, {}, {}) -> ({}, {}, {})\n",
        0x100, 0x000, 0x000,
        rotated.x(), rotated.y(), rotated.z()
    );

    // Composing transformations
    println!("6. Composed Transformations:");
    let t1 = Transform3D::translation(100, 0, 0);
    let t2 = Transform3D::rotation_z(90.0);
    let composed = t1.compose(&t2);

    let original = Point3D::new(0x100, 0x000, 0x000);
    let result = composed.apply(&original);
    println!("   Original: ({}, {}, {})", original.x(), original.y(), original.z());
    println!("   After translate(100,0,0) then rotate(90°): ({}, {}, {})\n",
        result.x(), result.y(), result.z()
    );

    println!("=== Geometric Examples Complete ===");
}
