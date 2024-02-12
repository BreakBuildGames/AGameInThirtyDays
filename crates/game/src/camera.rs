use glam::Vec4Swizzles;

use glam::{Mat4, Quat, Vec3};

pub struct Projection {
    matrix: Mat4,
    dirty: bool,

    near_plane: f32,
    far_plane: f32,
    mode: ProjectionMode,
}

pub enum ProjectionMode {
    Perspective {
        field_of_view: f32,
        aspect_ratio: f32,
    },
    Orthographic {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
}

//TODO: Lerp and stuff
pub struct Orbit {
    projection: Projection,
    view: Mat4,
    view_projection: Mat4,

    current_target: Vec3,
    current_rotation: Vec3,
    dirty: bool,
    distance: f32,
}

impl Default for Projection {
    fn default() -> Self {
        Self {
            matrix: Mat4::IDENTITY,
            dirty: true,
            near_plane: 0.1,
            far_plane: 100.0,
            mode: ProjectionMode::Orthographic {
                x: -0.5,
                y: 0.5,
                width: 1.0,
                height: -1.0,
            },
        }
    }
}

impl Default for Orbit {
    fn default() -> Self {
        Self {
            projection: Projection::default(),
            view: Mat4::IDENTITY,
            view_projection: Mat4::IDENTITY,
            dirty: true,
            distance: 3.0,
            current_target: Vec3::ZERO,
            current_rotation: Vec3::ZERO,
        }
    }
}

impl Projection {
    #[must_use]
    pub const fn get(&self) -> Mat4 {
        self.matrix
    }

    #[must_use]
    pub fn with_perspective(
        field_of_view: f32,
        aspect_ratio: f32,
        near_plane: f32,
        far_plane: f32,
    ) -> Self {
        let mut camera = Self {
            near_plane,
            far_plane,
            mode: ProjectionMode::Perspective {
                field_of_view,
                aspect_ratio,
            },
            ..Default::default()
        };

        camera.update_projection();
        camera
    }

    #[must_use]
    pub fn with_orthographic(
        x: f32,
        y: f32,
        widht: f32,
        height: f32,
        near_plane: f32,
        far_plane: f32,
    ) -> Self {
        Self {
            near_plane,
            far_plane,
            mode: ProjectionMode::Orthographic {
                x,
                y,
                width: widht,
                height,
            },
            ..Default::default()
        }
    }

    fn update_projection(&mut self) {
        self.matrix = match self.mode {
            ProjectionMode::Perspective {
                field_of_view,
                aspect_ratio,
            } => Mat4::perspective_rh(field_of_view, aspect_ratio, self.near_plane, self.far_plane),
            ProjectionMode::Orthographic {
                x,
                y,
                width,
                height,
            } => {
                Mat4::orthographic_rh(x, x + width, y + height, y, self.near_plane, self.far_plane)
            }
        };
        self.dirty = true;
    }
}

impl Orbit {
    #[must_use]
    pub fn new(target: Vec3, distance: f32) -> Self {
        let mut camera = Self {
            distance,
            current_target: target,
            ..Self::default()
        };

        camera.update_view();
        camera
    }

    #[must_use]
    pub fn with_camera(camera: Projection) -> Self {
        Self {
            projection: camera,
            ..Self::default()
        }
    }

    pub fn look_at(&mut self, target: Vec3, euler_rotation: Vec3, distance: f32) {
        self.current_rotation = euler_rotation;
        self.distance = distance;
        self.current_target = target;
        self.dirty = true;
    }

    pub fn rotate(&mut self, rotation: Vec3) {
        self.current_rotation += rotation;
        self.dirty = true;
    }

    pub fn zoom(&mut self, delta: f32) {
        self.distance += delta;
        self.dirty = true;
    }

    pub fn target(&mut self, target: Vec3) {
        self.current_target = target;
        self.dirty = true;
    }

    pub fn pivot(&mut self, delta: Vec3) {
        self.target(self.current_target + delta);
    }

    pub fn view_projection(&mut self) -> Mat4 {
        if self.dirty {
            self.update_view();
            self.view_projection = self.projection.get() * self.view;
        }

        self.view_projection
    }

    fn update_view(&mut self) {
        let rotation = Quat::from_euler(
            glam::EulerRot::YXZ,
            self.current_rotation.y,
            self.current_rotation.x,
            self.current_rotation.z,
        );

        //TODO: Use scaled forward vector for translation and distance instead?
        let translation = Mat4::from_translation(self.current_target);
        let distance = Mat4::from_translation(self.distance * Vec3::Z);
        let rotation = Mat4::from_quat(rotation);

        self.view = (translation * rotation * distance).inverse();
        self.dirty = false;
    }

    #[must_use]
    pub fn position(&self) -> Vec3 {
        self.view.transpose().inverse().row(3).xyz()
    }
}
