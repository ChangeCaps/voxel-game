#version 450

layout(location = 0) out vec4 ray;

const vec2 position[6] = vec2[6](
	vec2(-1.0, -1.0),
	vec2(1.0, -1.0),
	vec2(-1.0, 1.0),
	vec2(1.0, 1.0),
	vec2(-1.0, 1.0),
	vec2(1.0, -1.0)
);

void main() {
    gl_Position = vec4(position[gl_VertexIndex], 0.0, 1.0);
    ray = vec4(position[gl_VertexIndex], 0.0, 1.0);
}