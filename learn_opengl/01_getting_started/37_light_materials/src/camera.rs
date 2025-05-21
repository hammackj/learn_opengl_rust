extern crate nalgebra_glm as glm;

pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

pub const YAW: f32 = -90.0;
pub const PITCH: f32 = 0.0;
pub const SPEED: f32 = 2.5;
pub const SENSITIVITY: f32 = 0.1;
pub const ZOOM: f32 = 45.0;

pub struct Camera {
    pub position: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,
    pub world_up: glm::Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(position: glm::Vec3, up: glm::Vec3, yaw: f32, pitch: f32) -> Camera {
        Camera {
            position: position,
            front: glm::vec3(0.0, 0.0, -0.1),
            up: up,
            right: glm::vec3(0.0, 0.0, 0.0),
            world_up: up,
            yaw: yaw,
            pitch: pitch,
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVITY,
            zoom: ZOOM,
        }
    }

    pub fn get_view_matrix(&mut self) -> glm::Mat4 {
        let view = glm::look_at(&self.position, &(&self.position + &self.front), &self.up);

        return view;
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;

        match direction {
            CameraMovement::FORWARD => {
                self.position += self.front * velocity;
            }
            CameraMovement::BACKWARD => {
                self.position -= self.front * velocity;
            }
            CameraMovement::LEFT => {
                self.position -= self.right * velocity;
            }
            CameraMovement::RIGHT => {
                self.position += self.right * velocity;
            }
        }

        self.update_camera_vectors();
    }

    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32, constrain_pitch: bool) {
        let local_x_offset = x_offset * self.mouse_sensitivity;
        let local_y_offset = y_offset * self.mouse_sensitivity;

        self.yaw += local_x_offset;
        self.pitch += local_y_offset;

        if constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0
            }

            if self.pitch < -89.0 {
                self.pitch = -89.0
            }
        }

        self.update_camera_vectors();
    }

    pub fn process_mouse_scroll(&mut self, y_offset: f64) {
        self.zoom -= y_offset as f32;

        if self.zoom < 1.0 {
            self.zoom = 1.0;
        }

        if self.zoom > 45.0 {
            self.zoom = 45.0;
        }
    }

    pub fn update_camera_vectors(&mut self) {
        let mut front = glm::vec3(0.0, 0.0, 0.0);

        front.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        front.y = self.pitch.to_radians().sin();
        front.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.front = glm::normalize(&front);

        self.right = glm::normalize(&glm::cross(&self.front, &self.world_up));
        self.up = glm::normalize(&glm::cross(&self.right, &self.front));
    }
}
