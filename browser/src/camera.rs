use crate::types::*;
use nalgebra_glm as glm;

//fn screen2world(dspPoint: DspPoint) -> (f32, f32) {}

// based on #6 Intro to Modern OpenGL Tutorial: Camera and Perspective
// by thebennybox, https://www.youtube.com/watch?v=e3sc72ZDDpo

impl Camera {
    pub fn new(pos: V3, fov: f32, aspect: f32, z_near: f32, z_far: f32) -> Camera {
        Camera {
            pos: pos,
            perspective: glm::perspective(fov, aspect, z_near, z_far),
            forward: V3::new(0.0, 0.0, 1.0),
            up: V3::new(0.0, 1.0, 0.0),
        }
    }

    pub fn get_view_projection(&self) -> glm::Mat4 {
        let look = glm::look_at(&self.pos, &(self.pos + self.forward), &self.up);
        self.perspective * look
    }
}