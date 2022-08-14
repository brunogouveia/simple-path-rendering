#version 410 core

// In
layout (location = 0) in vec2 position;


void main() {
    vec2 normalized_position = position/vec2(1024,768) * 2 - 1;
    gl_Position = vec4(normalized_position, 0.5, 1);
}