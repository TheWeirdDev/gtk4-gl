#version 300 es
precision mediump float;

vec3 vColor = vec3(0.9412, 0.4667, 0.4667);
out vec4 f_color;
void main() {
    f_color = vec4(vColor, 1.0);
}