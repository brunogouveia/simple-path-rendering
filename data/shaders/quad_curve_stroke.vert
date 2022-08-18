#version 410 core

// In
layout (location = 0) in vec2 position;

// Out
layout (location = 0) out vec2 p;
layout (location = 1) out float B_term;
layout (location = 2) out float C_term;
layout (location = 3) out float D_term;

const vec2 uv[3] = vec2[3]( 
    vec2(0.0, 0.0),
    vec2(0.5, 0.0),
    vec2(1.0, 1.0)
);


float computeBTerm(vec2 A, vec2 B, vec2 C, vec2 p) {
    return 3 * (A.x * B.x + A.y * B.y);
}

float computeCTerm(vec2 A, vec2 B, vec2 C, vec2 p) {
    return 2 * (A.x * C.x + A.y * C.y) - 2 * (A.x * p.x + A.y * p.y) + B.x * B.x + B.y * B.y;
}

float computeDTerm(vec2 A, vec2 B, vec2 C, vec2 p) {
    return B.x * C.x + B.y * C.y - B.x * p.x - B.y * p.y;
}

void main() {
    vec2 c0 = vec2(100, 100);
    vec2 c1 = vec2(100, 600);
    vec2 c2 = vec2(600, 600);

    vec2 A = c0 - 2*c1 + c2;
    vec2 B = -2 * c0 + 2 * c1;
    vec2 C = c0;

    float denom = 2 * (A.x * A.x + A.y * A.y);

    B_term = computeBTerm(A, B, C, position);
    B_term /= denom;

    C_term = computeCTerm(A, B, C, position);
    C_term /= denom;

    D_term = computeDTerm(A, B, C, position);
    D_term /= denom;

	vec2 normalized_position = position/vec2(1024,768) * 2 - 1;

    gl_Position = vec4(normalized_position, 0, 1);
    p = position;
}