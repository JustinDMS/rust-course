//! # Re-exporting with pub use
//!
//! Concept: `pub use` lets a module present a flat, convenient API over a
//! deep internal structure. Users import from where it is convenient; the
//! code lives where it makes sense.
//!
//! Book: 7.4 "Re-exporting Names with pub use"
//!
//! Your job: the `engine` module tree is fixed and must not change. The
//! tests import everything from `prelude`. Write the `prelude` module so
//! they compile: it should contain **only** `pub use` lines.

mod engine {
    pub mod math {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct Vec3 {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        pub fn dot(a: Vec3, b: Vec3) -> f32 {
            a.x * b.x + a.y * b.y + a.z * b.z
        }
    }

    pub mod render {
        pub mod camera {
            #[derive(Debug, PartialEq)]
            pub struct Camera {
                pub fov: f32,
            }

            impl Camera {
                pub fn new(fov: f32) -> Camera {
                    Camera { fov }
                }
            }
        }
    }

    pub mod audio {
        pub fn play(name: &str) -> String {
            format!("playing {name}")
        }
    }
}

/// Everything a user of the engine normally needs, in one place.
pub mod prelude {
    // TODO: re-export Vec3, dot, Camera and play from the engine modules.
}

use prelude::*;

#[test]
fn vectors_come_from_the_prelude() {
    let a = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    let b = Vec3 { x: 2.0, y: 5.0, z: 0.0 };
    assert_eq!(dot(a, b), 2.0);
}

#[test]
fn cameras_come_from_the_prelude() {
    assert_eq!(Camera::new(90.0), Camera { fov: 90.0 });
}

#[test]
fn audio_comes_from_the_prelude() {
    assert_eq!(play("theme"), "playing theme");
}

#[test]
fn the_original_paths_still_work() {
    assert_eq!(engine::audio::play("x"), prelude::play("x"));
}
