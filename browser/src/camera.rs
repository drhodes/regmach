use crate::types::*;
use nalgebra_glm as glm;

// based on #6 Intro to Modern OpenGL Tutorial: Camera and Perspective
// by thebennybox, https://www.youtube.com/watch?v=e3sc72ZDDpo
impl Camera {
    pub fn new(pos: V3, fov: f32, aspect: f32, z_near: f32, z_far: f32) -> Camera {
        Camera { pos,
                 fov,
                 aspect,
                 z_near,
                 z_far,
                 perspective: glm::perspective(aspect, fov, z_near, z_far),
                 forward: V3::new(0.0, 0.0, 1.0),
                 up: V3::new(0.0, 1.0, 0.0) }
    }

    pub fn default() -> Camera {
        let fov = std::f32::consts::PI / 2.0;
        let z_near = 1.0; // if this is zero, then mouse picking will not work.
        Camera::new(V3::new(0f32, 0f32, -10f32), fov, 1.0, z_near, 1000.0)
    }

    pub fn update_aspect(&mut self, w: f64, h: f64) {
        if h <= EPSILON64 {
            return;
        }
        self.aspect = (w / h) as f32;
        self.perspective = glm::perspective(self.aspect, self.fov, self.z_near, self.z_far);
    }

    pub fn zoom_out(&mut self) {
        if self.pos.z < -160.0 {
            return;
        }
        self.pos.z *= 1.125;
    }

    pub fn zoom_in(&mut self) {
        if self.pos.z >= -self.z_near * 1.125 {
            return;
        }
        self.pos.z /= 1.125;
    }

    #[inline(always)]
    fn zoom_factor(&self) -> f32 {
        -self.pos.z * 0.1357 // four odd numbers in a row. (this is arbitrary)
    }

    pub fn pan_right(&mut self) {
        self.pos.x += self.zoom_factor();
    }

    pub fn pan_left(&mut self) {
        self.pos.x -= self.zoom_factor();
    }

    pub fn pan_up(&mut self) {
        self.pos.y += self.zoom_factor();
    }

    pub fn pan_down(&mut self) {
        self.pos.y -= self.zoom_factor();
    }

    pub fn center(&mut self) {
        self.pos.y = 0.0;
        self.pos.x = 0.0;
    }

    pub fn view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.pos, &(self.pos + self.forward), &self.up)
    }

    pub fn projection_matrix(&self) -> glm::Mat4 {
        self.perspective
    }

    pub fn get_view_projection(&self) -> glm::Mat4 {
        (self.perspective * self.view_matrix())
    }
}
