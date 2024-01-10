use engine::{
    VersionedIndex,
    Registry,
    systems::apply_transforms
};

use crate::{
    components::{
        DrawComponent,
        MeshComponent
    },
    resources::Shader
};

pub fn draw_ebo(
    entity: &VersionedIndex,
    registry: &Registry
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(shader_cmp) = registry.get_component::<DrawComponent>(entity) else { return Ok(()) };
    let Some(shader) = registry.get_resource::<Shader>(&shader_cmp.shader_id) else { return Ok(()) };
    let Some(mesh) = registry.get_component::<MeshComponent>(entity) else { return Ok(()) };

    shader.program().use_program();
    mesh.vao().bind();
    
    if let Some(model) = apply_transforms(entity, registry) {
        shader.program().set_mat4("model", &model);
    }

    unsafe {
        gl::DrawElements(
            gl::TRIANGLES,
            mesh.vao().count(),
            gl::UNSIGNED_INT,
            std::ptr::null()
        );
    }

    mesh.vao().unbind();

    Ok(())
}
