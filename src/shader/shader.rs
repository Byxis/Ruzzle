use raylib::prelude::*;

pub struct ShaderManager {
    pub cel_shade_shader: Shader,
    pub postprocess_shader: Shader,

    // cel_shade shader uniforms
    sunlight_dir_loc: i32,
    sunlight_color_loc: i32,
    ambient_color_loc: i32,

    // Postprocess shader uniforms
    pp_resolution_loc: i32,
    pp_bg_top_loc: i32,
    pp_bg_bottom_loc: i32,
}

impl ShaderManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // --- cel_shade shader ---
        let mut cel_shade_shader = rl.load_shader(
            thread,
            Some("rsc/shaders/cel_shade.vs"),
            Some("rsc/shaders/cel_shade.fs"),
        );

        let sunlight_dir_loc = cel_shade_shader.get_shader_location("sunLightDir");
        let sunlight_color_loc = cel_shade_shader.get_shader_location("sunLightColor");
        let ambient_color_loc = cel_shade_shader.get_shader_location("ambientColor");

        // Set default values
        cel_shade_shader.set_shader_value(sunlight_dir_loc, [-0.5f32, -0.8, -0.3]);
        cel_shade_shader.set_shader_value(sunlight_color_loc, [1.0f32, 0.9, 0.75]);
        cel_shade_shader.set_shader_value(ambient_color_loc, [0.25f32, 0.2, 0.15]);

        // --- Postprocess shader ---
        let postprocess_shader = rl.load_shader(thread, None, Some("rsc/shaders/postprocess.fs"));

        let pp_resolution_loc = postprocess_shader.get_shader_location("resolution");
        let pp_bg_top_loc = postprocess_shader.get_shader_location("bgTop");
        let pp_bg_bottom_loc = postprocess_shader.get_shader_location("bgBottom");

        ShaderManager {
            cel_shade_shader,
            postprocess_shader,
            sunlight_dir_loc,
            sunlight_color_loc,
            ambient_color_loc,
            pp_resolution_loc,
            pp_bg_top_loc,
            pp_bg_bottom_loc,
        }
    }

    pub fn update_postprocess_resolution(&mut self, width: f32, height: f32) {
        self.postprocess_shader
            .set_shader_value(self.pp_resolution_loc, [width, height]);
    }

    pub fn update_background_colors(&mut self, top: Color, bottom: Color) {
        let top_rgb = [
            top.r as f32 / 255.0,
            top.g as f32 / 255.0,
            top.b as f32 / 255.0,
        ];
        let bottom_rgb = [
            bottom.r as f32 / 255.0,
            bottom.g as f32 / 255.0,
            bottom.b as f32 / 255.0,
        ];
        self.postprocess_shader
            .set_shader_value(self.pp_bg_top_loc, top_rgb);
        self.postprocess_shader
            .set_shader_value(self.pp_bg_bottom_loc, bottom_rgb);
    }

    pub fn set_sunlight_dir(&mut self, dir: [f32; 3]) {
        self.cel_shade_shader
            .set_shader_value(self.sunlight_dir_loc, dir);
    }

    pub fn set_sunlight_color(&mut self, color: Color) {
        let rgb = [
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
        ];
        self.cel_shade_shader
            .set_shader_value(self.sunlight_color_loc, rgb);
    }

    pub fn set_ambient_color(&mut self, color: Color) {
        let rgb = [
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
        ];
        self.cel_shade_shader
            .set_shader_value(self.ambient_color_loc, rgb);
    }

    pub fn apply_cel_shade_to_model(&self, model: &mut Model) {
        for material in model.materials_mut() {
            material.shader = *self.cel_shade_shader.as_ref();
        }
    }
}
