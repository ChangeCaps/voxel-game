#version 450

layout(location = 0) in vec4 ray;

layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 0) uniform Camera {
    mat4 view_proj;
};

layout(set = 0, binding = 1) uniform CameraPosition {
    vec3 cam_position;
};

layout(set = 1, binding = 0) uniform texture2DArray voxel_texture;
layout(set = 1, binding = 1) uniform sampler voxel_sampler;

const int MAX_STEPS = 512;
const int MAX_DEPTH = 8;
const float ALPHA_THRESHOLD = 0.01;

struct Hit {
    vec4 data;
    vec3 pos;
    vec3 normal;
};

vec2 intersect_box(in vec3 ro, in vec3 rd, in vec3 b_min, in vec3 b_max) {
    vec3 t_min = (b_min - ro) / rd;
    vec3 t_max = (b_max - ro) / rd;
    vec3 t1 = min(t_min, t_max);
    vec3 t2 = max(t_min, t_max);
    float near = max(max(t1.x, t1.y), t1.z);
    float far = min(min(t2.x, t2.y), t2.z);
    return vec2(near, far);
}

bool in_bounds(in vec3 pos) {
    ivec3 dimensions = textureSize(sampler2DArray(voxel_texture, voxel_sampler), 0);

    return all(greaterThan(pos, vec3(0.0))) && all(lessThan(pos, vec3(dimensions)));
}

Hit trace(in vec3 ro, in vec3 rd) {
    Hit hit;
    hit.data = vec4(0.0);

    ivec3 texture_size = textureSize(sampler2DArray(voxel_texture, voxel_sampler), 0);

    vec3 lookup = ro;

    if (!in_bounds(lookup)) {
        vec2 h = intersect_box(ro, rd, vec3(0.0), vec3(texture_size));

        if (h.x > h.y || h.y < 0.0) {
            return hit;
        }

        lookup += rd * h.x * 1.001;
    }

    ivec3 cell = ivec3(floor(lookup));
    ivec3 dir = ivec3(sign(rd));

    for (int i = 0; i < MAX_STEPS; i++) {
        hit.pos = lookup;
        hit.data = texelFetch(sampler2DArray(voxel_texture, voxel_sampler), cell, 0);

        if (hit.data.a > 0.0 || !in_bounds(lookup)) {
            return hit;
        }

        ivec3 bounds = cell + max(ivec3(0), dir);

        vec3 delta = (vec3(bounds) - lookup) / rd;

        float t = min(min(delta.x, delta.y), delta.z);

        if (t == delta.x) {
            cell.x += dir.x;
            hit.normal = vec3(-dir.x, 0.0, 0.0);
        }

        if (t == delta.y) {
            cell.y += dir.y;
            hit.normal = vec3(0.0, -dir.y, 0.0);
        }

        if (t == delta.z) {
            cell.z += dir.z;
            hit.normal = vec3(0.0, 0.0, -dir.z);
        }

        lookup += rd * t;
    }

    return hit;
}

vec3 sky(in vec3 rd) {
    float sky = max(rd.y * 0.5 + 0.5, 0.0);

    vec3 color = vec3(0.5, 0.8, 0.9) - sky * 0.5;
    color = mix(color, vec3(0.5, 0.7, 0.9), exp(-10.0 * sky));

    return color;
}

void main() {
    mat4 inverse_view = inverse(view_proj);
    vec4 near = inverse_view * ray;
    vec4 far = near + inverse_view[2];
    near.xyz /= near.w;
    far.xyz /= far.w;

    vec3 sun_direction = normalize(vec3(1.0, 1.5, 1.0));

    vec3 rd = normalize(far.xyz - near.xyz);
    vec3 ro = cam_position;

    Hit hit = trace(ro, rd);

    Hit shadow_hit = trace(hit.pos + hit.normal * 0.0001, sun_direction);

    float shadow;

    if (shadow_hit.data.a > 0.0) {
        shadow = 0.5;
    } else {
        shadow = 1.0;
    }

    float diffuse = max(0.0, dot(hit.normal, sun_direction));

    float light = 0.0;
    light += 0.2 + 0.8 * shadow * diffuse;

    vec3 color = hit.data.rgb;

    if (hit.data.a > 0.0) {
        color *= light;
    } else {
        color = sky(rd);
    }

    out_color = vec4(color, 1.0);
}
