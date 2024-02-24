use crate::{
    prelude::{
        qp_ecs::{
            components::{
                CSprite,
                CTransform2D
            },
            resources::{
                RCamera2D,
                RShader
            },
        },
        qp_data::{
            FrameState,
            IRenderer,
        },
        Registry
    },
    platform::opengl::capabilities::{
        gl_blending_func,
        gl_enable,
        GLBlendingFactor,
        GLCapability
    },
};

use super::super::batch_renderer::BatchRenderer;

pub struct Renderer2D {
    camera: u64,
    shader: u64,

    renderer: BatchRenderer<10000, CSprite>
}

impl Renderer2D {
    pub fn new(
        registry: &mut Registry,
        camera: &str,
        shader: &str
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let Some(camera) = registry.get_resource_id(camera) else {
            return Err("[2d renderer] camera is not loaded".into());
        };

        let Some(shader) = registry.get_resource_id(shader) else {
            return Err("[2d renderer] shader is not loaded".into());
        };

        Ok(Self {
            camera,
            shader,
            renderer: BatchRenderer::new(),
        })
    }
}

impl IRenderer for Renderer2D {
    fn draw(
        &mut self,
        _frame_state: &mut FrameState,
        registry: &mut Registry
    ) -> Option<u32> {
        let entities = registry.entities.query_all::<CSprite>();

        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);

        if registry.get_resource::<RShader>(self.shader).is_none() {
            #[cfg(debug_assertions)]
            println!("[sprite controller] tried to use a shader that is not loaded");

            return None;
        };

        let Some(camera) = registry.get_resource::<RCamera2D>(self.camera) else {
            #[cfg(debug_assertions)]
            println!("[sprite controller] tried to use a camera that is not loaded");

            return None;
        };

        let view = RCamera2D::calc_view_matrix(&camera.transform);
        let projection = RCamera2D::calc_projection_matrix(&camera.params);

        self.renderer.reset_info();
        self.renderer.begin_batch();
        for entity in entities.iter() {
            let Some(transform) = registry.entities.get::<CTransform2D>(&entity) else {
                #[cfg(debug_assertions)]
                println!("[sprite controller] tried to render a sprite without a tranform component");

                continue;
            };
            let model = transform.to_matrix();

            let Some(sprite) = registry.entities.get_mut::<CSprite>(&entity) else {
                #[cfg(debug_assertions)]
                println!("[sprite controller] tried to render a sprite without a sprite component");

                continue;
            };
            sprite.apply_matrices(model, view, projection);

            let sprite = registry.entities.get::<CSprite>(&entity).unwrap();
            let texture = match &sprite.texture_atlas {
                Some(atlas) => registry.get_resource(atlas.texture),
                _ => None
            };

            self.renderer.draw_mesh(sprite, registry.get_resource(self.shader)?, texture);
        }
        self.renderer.end_batch();
        self.renderer.flush_batch(registry.get_resource(self.shader)?);

        Some(self.renderer.draw_calls)
    }
}