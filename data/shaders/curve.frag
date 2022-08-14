#version 410 core

// In
layout (location = 0) in vec2 frag_uv;

// Out
layout (location = 0) out vec4 out_color;

void main() {
    float f = step(frag_uv.x * frag_uv.x - frag_uv.y, 0.0);

    if (f <= 0.0) {
        discard;
    } else {
        out_color = vec4(0.5) * f;
        // out_color = vec4(1);
    }
}