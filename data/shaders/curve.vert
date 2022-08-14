#version 410 core

// In
layout (location = 0) in vec2 position;

// Out
layout (location = 0) out vec2 frag_uv;

const vec2 uv[3] = vec2[3]( 
    vec2(0.0, 0.0),
    vec2(0.5, 0.0),
    vec2(1.0, 1.0)
);

void main() {
	vec2 normalized_position = position/vec2(1024,768) * 2 - 1;

    gl_Position = vec4(normalized_position, 0, 1);
    frag_uv = uv[gl_VertexID % 3];
}