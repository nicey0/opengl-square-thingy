#version 140

in vec2 pos;

uniform mat4 matrix; // transform

void main() {
    gl_Position = matrix * vec4(pos, 0.0, 1.0);
}
