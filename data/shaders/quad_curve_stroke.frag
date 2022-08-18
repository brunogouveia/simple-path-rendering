#version 410 core

// In
layout (location = 0) in vec2 p;
layout (location = 1) in float B_term;
layout (location = 2) in float C_term;
layout (location = 3) in float D_term;


// Out
layout (location = 0) out vec4 out_color;

const vec2 eta = vec2(-0.5, sqrt(0.75));
int solveCubic(in float a, in float b, in float c, in float d, out vec3 roots) {
    float h = 18.0 * a * b * c * d - 4.0 * b * b * b * d + b * b * c * c - 4.0 * a * c * c * c - 27.0 * a * a * d * d;

    b /= a, c /= a, d /= a;
    float d0 = b * b - 3.0 * c;
    float d1 = (2.0 * b * b - 9.0 * c) * b + 27.0 * d;
    float q = d1 * d1 - 4.0 * d0 * d0 * d0, j = sqrt(abs(q));

    vec2 C = q < 0.0 ? vec2(d1, j) : vec2(d1 + j, 0.0);
    if (abs(C.x) + abs(C.y) < 1e-3) C = vec2(d1 - j, 0.0);
    float t = atan(C.y, C.x) / 3.0, r = pow(0.25 * dot(C, C), 1.0 / 6.0);
    C = vec2(cos(t), sin(t));

    float w = -d0 / r - r;
    roots.x = (C.x * w - b) / 3.0;
    roots.y = (dot(vec2(C.x, -C.y), eta) * w - b) / 3.0;
    if (h > 0.0) roots.z = (dot(C, eta) * w - b) / 3.0;
    else if (abs(dot(C.yx, eta)) < abs(C.y)) roots.x = roots.y;

    return h < 0.0 ? 1 : 3;
}

vec2 q(float t) {
    vec2 c0 = vec2(100, 100);
    vec2 c1 = vec2(100, 600);
    vec2 c2 = vec2(600, 600);

    return t * t * (c0 - 2 * c1 + c2) + t * (-2 * c0 + 2 * c1) + c0;
}

void main() {
    vec3 roots;

    int numRoots = solveCubic(1, B_term, C_term, D_term, roots);

    if (numRoots >= 1) {
        float firstRoot = roots.x;

        vec2 point = q(firstRoot);

        if (firstRoot >= 0 && firstRoot <= 1 && distance(point, p) < 5) {
            out_color = vec4(1);        
            return;
        }
    }

    out_color = vec4(1, 0, 0, 1);
}