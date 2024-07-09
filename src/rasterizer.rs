use crate::math::*;
use crate::mesh::*;
use crate::scene::*;

use std::cmp::max;
use std::cmp::min;

pub fn draw_mesh(
    mesh: &Mesh,
    transform: Mat4,
    lights: &[Light],
    camera: Camera,
    pixel_buffer: &mut [Color],
    depth_buffer: &mut [f32],
) {
    let inverse_transform = match transform.inverse() {
        Some(inverse) => Mat3::from(inverse.transpose()),
        None => Mat3::default(),
    };

    for t in &mesh.face_indicies {
        let world_to_v0 = transform * mesh.verticies[t.a];
        let world_to_v1 = transform * mesh.verticies[t.b];
        let world_to_v2 = transform * mesh.verticies[t.c];

        let v0_normal = (inverse_transform * mesh.vertex_normals[t.a_normal]).normalized();
        let v1_normal = (inverse_transform * mesh.vertex_normals[t.b_normal]).normalized();
        let v2_normal = (inverse_transform * mesh.vertex_normals[t.c_normal]).normalized();

        let mut ndc_v0 = camera.projection_mat * camera.view_mat * world_to_v0;
        let mut ndc_v1 = camera.projection_mat * camera.view_mat * world_to_v1;
        let mut ndc_v2 = camera.projection_mat * camera.view_mat * world_to_v2;

        // let face_normal = Vector3::cross(world_to_v2 - world_to_v0, world_to_v1 - world_to_v0).normalized();

        // if any points are on screen
        // FIXME: I removed backface culling because it requires the view position, which is not
        // easily accesible yet
        if is_on_screen(ndc_v0, camera.near_plane, camera.far_plane)
            || is_on_screen(ndc_v1, camera.near_plane, camera.far_plane)
            || is_on_screen(ndc_v2, camera.near_plane, camera.far_plane)
        {
            // screen coords
            let pixel_v0 = ndc_v0.ndc_to_pixel(camera.canvas_width, camera.canvas_height);
            let pixel_v1 = ndc_v1.ndc_to_pixel(camera.canvas_width, camera.canvas_height);
            let pixel_v2 = ndc_v2.ndc_to_pixel(camera.canvas_width, camera.canvas_height);

            // (note: amoussa) perhaps this could be passed as a function pointer to the draw call
            let phong_lighting = |light: Light, vertex: Vector3, normal: Vector3| -> Vector3 {
                let v_to_light = (light.position - vertex).normalized();
                let color = light.color.to_vector3();
                (color * f32::max(Vector3::dot(normal, v_to_light), 0.0))
                    + (color * light.ambient_strength)
            };

            let c0 = lights
                .iter()
                .map(|&light| phong_lighting(light, world_to_v0, v0_normal))
                .fold(Vector3::default(), |acc, color| acc + color);
            let c1 = lights
                .iter()
                .map(|&light| phong_lighting(light, world_to_v1, v1_normal))
                .fold(Vector3::default(), |acc, color| acc + color);
            let c2 = lights
                .iter()
                .map(|&light| phong_lighting(light, world_to_v2, v2_normal))
                .fold(Vector3::default(), |acc, color| acc + color);

            // pre-compute inverse depth before loop
            ndc_v0.z = 1.0 / ndc_v0.z;
            ndc_v1.z = 1.0 / ndc_v1.z;
            ndc_v2.z = 1.0 / ndc_v2.z;

            let c0 = c0 * ndc_v0.z;
            let c1 = c1 * ndc_v1.z;
            let c2 = c2 * ndc_v2.z;

            let area = triangle_edge(pixel_v2, pixel_v0, pixel_v1);

            // axis aligned bounding box of triangle (clipped to match screen)
            let x_start = max(min(min(pixel_v0.x, pixel_v1.x), pixel_v2.x), 0);
            let x_end = min(
                max(max(pixel_v0.x, pixel_v1.x), pixel_v2.x),
                camera.canvas_width,
            );
            let y_start = max(min(min(pixel_v0.y, pixel_v1.y), pixel_v2.y), 0);
            let y_end = min(
                max(max(pixel_v0.y, pixel_v1.y), pixel_v2.y),
                camera.canvas_height,
            );

            for x in x_start..x_end {
                for y in y_start..y_end {
                    let current_pixel = ScreenCoordinate { x, y };
                    let mut w0 = triangle_edge(current_pixel, pixel_v1, pixel_v2);
                    let mut w1 = triangle_edge(current_pixel, pixel_v2, pixel_v0);
                    let mut w2 = triangle_edge(current_pixel, pixel_v0, pixel_v1);

                    let edge0 = ndc_v2 - ndc_v1;
                    let edge1 = ndc_v0 - ndc_v2;
                    let edge2 = ndc_v1 - ndc_v0;

                    // are we inside of a triangle? (also does a top left edge rule check)
                    if ((w0 == 0.0 && ((edge0.y == 0.0 && edge0.x > 0.0) || edge0.y > 0.0))
                        || w0 >= 0.0)
                        && ((w1 == 0.0 && ((edge1.y == 0.0 && edge1.x > 0.0) || edge1.y > 0.0))
                            || w1 >= 0.0)
                        && ((w2 == 0.0 && ((edge2.y == 0.0 && edge2.x > 0.0) || edge2.y > 0.0))
                            || w2 >= 0.0)
                    {
                        let buff_idx = ((y * camera.canvas_width) + x) as usize;
                        w0 /= area;
                        w1 /= area;
                        w2 /= area;

                        // (note: amoussa) this is a very unintuitive formula I recommend reading about
                        // it here: https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation/visibility-problem-depth-buffer-depth-interpolation.html
                        let depth = 1.0 / (ndc_v0.z * w0 + ndc_v1.z * w1 + ndc_v2.z * w2);

                        // depth test
                        if depth < depth_buffer[buff_idx] {
                            depth_buffer[buff_idx] = depth;
                            let lighting_color = (c0 * w0 + c1 * w1 + c2 * w2) * depth;
                            if mesh.texture.is_some() {
                                let v0_texture_coordinate =
                                    mesh.vertex_texture_coords[t.a_texture] * ndc_v0.z;
                                let v1_texture_coordinate =
                                    mesh.vertex_texture_coords[t.b_texture] * ndc_v1.z;
                                let v2_texture_coordinate =
                                    mesh.vertex_texture_coords[t.c_texture] * ndc_v2.z;

                                let object_uv = (v0_texture_coordinate * w0
                                    + v1_texture_coordinate * w1
                                    + v2_texture_coordinate * w2)
                                    * depth;
                                let object_color = mesh
                                    .texture
                                    .as_ref()
                                    .unwrap()
                                    .sample(object_uv.x, object_uv.y)
                                    .to_vector3();

                                pixel_buffer[buff_idx] = (object_color * lighting_color).to_color();
                            } else {
                                pixel_buffer[buff_idx] = lighting_color.to_color();
                            }
                        }
                    }
                }
            }
        }
    }
}

/*
 * This function determines which side of the line defined by v0 and v1 the the given point is on.
 * returns true if left of the line. v0 and v1 are intended to be provided in counter-clockwise order.
 */
fn triangle_edge(point: ScreenCoordinate, v0: ScreenCoordinate, v1: ScreenCoordinate) -> f32 {
    ((point.x - v0.x) * (v0.y - v1.y) - (point.y - v0.y) * (v0.x - v1.x)) as f32
}

/*
 * Expects an NDC vertex
 */
fn is_on_screen(point: Vector3, near: f32, far: f32) -> bool {
    point.z > near
        && point.z < far
        && point.x >= -1.0
        && point.x <= 1.0
        && point.y >= -1.0
        && point.y <= 1.0
}
