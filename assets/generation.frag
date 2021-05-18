#version 450

layout(location = 0) in vec3 pos;

layout(location = 0) out vec4 out_color;

layout(set = 1, binding = 0) uniform Radius {
	float radius;
};

void main() {
	if (length(pos) < 0.9) {	
		out_color = vec4(
			sin(pos.x * 1000.0) * 0.5 + 0.5, 
			sin(pos.y * 1000.0) * 0.5 + 0.5, 
			sin(pos.z * 1000.0) * 0.5 + 0.5, 
			1.0
		);
	} else {	
		out_color = vec4(0.0);
	}
}
