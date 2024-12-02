use raylib::prelude::*;

//================================================================

pub fn direction_from_euler(pitch: f32, yaw: f32) -> (Vector3, Vector3, Vector3) {
    let pitch = pitch * DEG2RAD as f32;
    let yaw = yaw * DEG2RAD as f32;

    let mut x = Vector3::default();
    let mut y = Vector3::default();
    let mut z = Vector3::default();

    // forward
    x.x = pitch.cos() * yaw.sin();
    x.y = -pitch.sin();
    x.z = pitch.cos() * yaw.cos();

    // up
    y.x = pitch.sin() * yaw.sin();
    y.y = pitch.cos();
    y.z = pitch.sin() * yaw.cos();

    // right
    z.x = yaw.cos();
    z.y = 0.0;
    z.z = -yaw.sin();

    (x, y, z)
}

/*
#[rustfmt::skip]
unsafe fn draw_cube(texture: &Texture2D, draw: &mut RaylibDrawHandle, position: Vector3, color: Color, camera: Camera3D)
{
    let mut draw_3d = draw.begin_mode3D(camera);

    draw_3d.draw_grid(32, 1.0);

    let x = position.x;
    let y = position.y;
    let z = position.z;

    let data = [
        // front
        Vector3::new(-1.0, -1.0,  1.0),
        Vector3::new( 1.0, -1.0,  1.0),
        Vector3::new( 1.0,  2.0,  1.0),
        Vector3::new(-1.0,  1.0,  1.0),
        // back
        Vector3::new(-1.0, -1.0, -1.0),
        Vector3::new( 1.0, -1.0, -1.0),
        Vector3::new( 1.0,  1.0, -1.0),
        Vector3::new(-1.0,  1.0, -1.0),
    ];

    for (i, vertex) in data.iter().enumerate() {
        ffi::DrawCubeV(vertex.into(), Vector3::new(0.1, 0.1, 0.1).into(), Color::WHITE.into());
    }

    ffi::rlSetTexture(texture.id);

        let mut idx = 0;
        let mut t_x = 0.0;
        let mut t_y = 0.0;

        ffi::rlBegin(ffi::RL_QUADS.try_into().unwrap());
            ffi::rlColor4ub(color.r, color.g, color.b, color.a);

            // Front Face
            ffi::rlNormal3f(0.0, 0.0, 1.0);
            idx = 0; t_x = 0.0; t_y = 1.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 1; t_x = 1.0; t_y = 1.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 2; t_x = 1.0; t_y = 0.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 3; t_x = 0.0; t_y = 0.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);

            // Back Face
            ffi::rlNormal3f(0.0, 0.0, -1.0);
            idx = 5; t_x = 0.0; t_y = 1.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 4; t_x = 1.0; t_y = 1.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 7; t_x = 1.0; t_y = 0.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 6; t_x = 0.0; t_y = 0.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);

            // Top Face
            ffi::rlNormal3f(0.0, 1.0, 0.0);       // Normal Pointing Up
            idx = 3; t_x = 0.0; t_y = 1.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 2; t_x = 1.0; t_y = 1.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 6; t_x = 1.0; t_y = 0.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 7; t_x = 0.0; t_y = 0.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);

            // Bottom Face
            ffi::rlNormal3f(0.0, - 1.0, 0.0);     // Normal Pointing Down
            idx = 1; t_x = 0.0; t_y = 1.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 0; t_x = 1.0; t_y = 1.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 4; t_x = 1.0; t_y = 0.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 5; t_x = 0.0; t_y = 0.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);

            // Right face
            ffi::rlNormal3f(1.0, 0.0, 0.0);       // Normal Pointing Right
            idx = 1; t_x = 0.0; t_y = 1.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 5; t_x = 1.0; t_y = 1.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 6; t_x = 1.0; t_y = 0.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 2; t_x = 0.0; t_y = 0.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);

            // Left Face
            ffi::rlNormal3f( - 1.0, 0.0, 0.0);    // Normal Pointing Left
            idx = 4; t_x = 0.0; t_y = 1.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 0; t_x = 1.0; t_y = 1.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 3; t_x = 1.0; t_y = 0.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);
            idx = 7; t_x = 0.0; t_y = 0.0; ffi::rlTexCoord2f(t_x, t_y); ffi::rlVertex3f(x + data[idx].x, y + data[idx].y, z + data[idx].z);

        ffi::rlEnd();

    ffi::rlSetTexture(0);

    drop(draw_3d);

    for (i, vertex) in data.iter().enumerate() {
        ffi::DrawCubeV(vertex.into(), Vector3::new(0.1, 0.1, 0.1).into(), Color::WHITE.into());
        let point = draw.get_world_to_screen(vertex, camera);
        draw.draw_text(&format!("{i}"), point.x as i32, point.y as i32, 16, Color::GREEN);
    }
}
*/

use ffi::rlPopMatrix;
use ffi::rlPushMatrix;

pub fn draw_grid(slice: i32, space: f32, angle: Quaternion) {
    let half_slice = slice / 2;

    unsafe {
        rlPushMatrix();
        ffi::rlRotatef(angle.w, angle.x, angle.y, angle.z);

        ffi::rlBegin(ffi::RL_LINES.try_into().unwrap());
        for i in -half_slice..half_slice {
            if i == 0 {
                ffi::rlColor3f(0.5, 0.5, 0.5);
            } else {
                ffi::rlColor3f(0.75, 0.75, 0.75);
            }

            ffi::rlVertex3f(i as f32 * space, 0.0, -half_slice as f32 * space);
            ffi::rlVertex3f(i as f32 * space, 0.0, half_slice as f32 * space);

            ffi::rlVertex3f(-half_slice as f32 * space, 0.0, i as f32 * space);
            ffi::rlVertex3f(half_slice as f32 * space, 0.0, i as f32 * space);
        }
        ffi::rlEnd();

        rlPopMatrix();
    }
}

pub fn vector_3_unproject(source: Vector3, projection: Matrix, view: Matrix) -> Vector3 {
    let mut result = Vector3::zero();

    // Calculate unprojected matrix (multiply view matrix by projection matrix) and invert it
    let mat_view_proj = view * projection; // MatrixMultiply(view, projection);

    // Calculate inverted matrix -> MatrixInvert(matViewProj);
    // Cache the matrix values (speed optimization)
    let a00 = mat_view_proj.m0;
    let a01 = mat_view_proj.m1;
    let a02 = mat_view_proj.m2;
    let a03 = mat_view_proj.m3;
    let a10 = mat_view_proj.m4;
    let a11 = mat_view_proj.m5;
    let a12 = mat_view_proj.m6;
    let a13 = mat_view_proj.m7;
    let a20 = mat_view_proj.m8;
    let a21 = mat_view_proj.m9;
    let a22 = mat_view_proj.m10;
    let a23 = mat_view_proj.m11;
    let a30 = mat_view_proj.m12;
    let a31 = mat_view_proj.m13;
    let a32 = mat_view_proj.m14;
    let a33 = mat_view_proj.m15;

    let b00 = a00 * a11 - a01 * a10;
    let b01 = a00 * a12 - a02 * a10;
    let b02 = a00 * a13 - a03 * a10;
    let b03 = a01 * a12 - a02 * a11;
    let b04 = a01 * a13 - a03 * a11;
    let b05 = a02 * a13 - a03 * a12;
    let b06 = a20 * a31 - a21 * a30;
    let b07 = a20 * a32 - a22 * a30;
    let b08 = a20 * a33 - a23 * a30;
    let b09 = a21 * a32 - a22 * a31;
    let b10 = a21 * a33 - a23 * a31;
    let b11 = a22 * a33 - a23 * a32;

    // Calculate the invert determinant (inlined to avoid double-caching)
    let inv_det = 1.0 / (b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06);

    let mat_view_proj_inv = Matrix {
        m0: (a11 * b11 - a12 * b10 + a13 * b09) * inv_det,
        m1: (-a01 * b11 + a02 * b10 - a03 * b09) * inv_det,
        m2: (a31 * b05 - a32 * b04 + a33 * b03) * inv_det,
        m3: (-a21 * b05 + a22 * b04 - a23 * b03) * inv_det,
        m4: (-a10 * b11 + a12 * b08 - a13 * b07) * inv_det,
        m5: (a00 * b11 - a02 * b08 + a03 * b07) * inv_det,
        m6: (-a30 * b05 + a32 * b02 - a33 * b01) * inv_det,
        m7: (a20 * b05 - a22 * b02 + a23 * b01) * inv_det,
        m8: (a10 * b10 - a11 * b08 + a13 * b06) * inv_det,
        m9: (-a00 * b10 + a01 * b08 - a03 * b06) * inv_det,
        m10: (a30 * b04 - a31 * b02 + a33 * b00) * inv_det,
        m11: (-a20 * b04 + a21 * b02 - a23 * b00) * inv_det,
        m12: (-a10 * b09 + a11 * b07 - a12 * b06) * inv_det,
        m13: (a00 * b09 - a01 * b07 + a02 * b06) * inv_det,
        m14: (-a30 * b03 + a31 * b01 - a32 * b00) * inv_det,
        m15: (a20 * b03 - a21 * b01 + a22 * b00) * inv_det,
    };

    // Create quaternion from source point
    let quat = Quaternion::new(source.x, source.y, source.z, 1.0);

    // Multiply quat point by unprojecte matrix
    let q_transformed = Quaternion {
        // QuaternionTransform(quat, matViewProjInv)
        x: mat_view_proj_inv.m0 * quat.x
            + mat_view_proj_inv.m4 * quat.y
            + mat_view_proj_inv.m8 * quat.z
            + mat_view_proj_inv.m12 * quat.w,
        y: mat_view_proj_inv.m1 * quat.x
            + mat_view_proj_inv.m5 * quat.y
            + mat_view_proj_inv.m9 * quat.z
            + mat_view_proj_inv.m13 * quat.w,
        z: mat_view_proj_inv.m2 * quat.x
            + mat_view_proj_inv.m6 * quat.y
            + mat_view_proj_inv.m10 * quat.z
            + mat_view_proj_inv.m14 * quat.w,
        w: mat_view_proj_inv.m3 * quat.x
            + mat_view_proj_inv.m7 * quat.y
            + mat_view_proj_inv.m11 * quat.z
            + mat_view_proj_inv.m15 * quat.w,
    };

    // Normalized world points in vectors
    result.x = q_transformed.x / q_transformed.w;
    result.y = q_transformed.y / q_transformed.w;
    result.z = q_transformed.z / q_transformed.w;

    return result;
}

pub fn get_screen_to_world_ray_ex(
    position: Vector2,
    camera: Camera3D,
    width: i32,
    height: i32,
) -> Ray {
    let mut ray = Ray::default();

    // Calculate normalized device coordinates
    // NOTE: y value is negative
    let x = (2.0 * position.x) / width as f32 - 1.0;
    let y = 1.0 - (2.0 * position.y) / height as f32;
    let z = 1.0;

    // Store values in a vector
    let device_coords = Vector3::new(x, y, z);

    // Calculate view matrix from camera look at
    let mat_view = Matrix::look_at(camera.position, camera.target, camera.up);

    let mut mat_proj = Matrix::identity();

    if camera.camera_type() == CameraProjection::CAMERA_PERSPECTIVE {
        // Calculate projection matrix from perspective
        mat_proj = Matrix::perspective(
            camera.fovy * DEG2RAD as f32,
            width as f32 / height as f32,
            ffi::RL_CULL_DISTANCE_NEAR as f32,
            ffi::RL_CULL_DISTANCE_FAR as f32,
        );
    } else if camera.camera_type() == CameraProjection::CAMERA_ORTHOGRAPHIC {
        let aspect = width as f32 / height as f32;
        let top = camera.fovy / 2.0;
        let right = top * aspect;

        // Calculate projection matrix from orthographic
        mat_proj = Matrix::ortho(-right, right, -top, top, 0.01, 1000.0);
    }

    // Unproject far/near points
    let near_point = vector_3_unproject(
        Vector3::new(device_coords.x, device_coords.y, 0.0),
        mat_proj,
        mat_view,
    );
    let far_point = vector_3_unproject(
        Vector3::new(device_coords.x, device_coords.y, 1.0),
        mat_proj,
        mat_view,
    );

    // Unproject the mouse cursor in the near plane
    // We need this as the source position because orthographic projects,
    // compared to perspective doesn't have a convergence point,
    // meaning that the "eye" of the camera is more like a plane than a point
    let camera_plane_pointer_pos = vector_3_unproject(
        Vector3::new(device_coords.x, device_coords.y, -1.0),
        mat_proj,
        mat_view,
    );

    // Calculate normalized direction vector
    let direction = Vector3::normalized(&(far_point - near_point));

    if camera.camera_type() == CameraProjection::CAMERA_PERSPECTIVE {
        ray.position = camera.position
    } else if camera.camera_type() == CameraProjection::CAMERA_ORTHOGRAPHIC {
        ray.position = camera_plane_pointer_pos
    };

    // Apply calculated vectors to ray
    ray.direction = direction;

    return ray;
}
