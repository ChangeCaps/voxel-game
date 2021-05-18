#version 450

layout(location = 0) out vec3 pos;

const vec2 position[6] = vec2[6](
	vec2(-1.0, -1.0),
	vec2(1.0, -1.0),
	vec2(-1.0, 1.0),
	vec2(1.0, 1.0),
	vec2(-1.0, 1.0),
	vec2(1.0, -1.0)
);

layout(set = 0, binding = 0) uniform Layer {
	float layer;	
};

void main() {
	vec2 p = position[gl_VertexIndex];

    gl_Position = vec4(p, 0.0, 1.0);

	pos = vec3(p, layer);
}
