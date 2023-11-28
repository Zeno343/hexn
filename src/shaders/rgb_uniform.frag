#version 460
layout (location = 2) uniform vec3 rgb;
out vec4 FragColor;

void main() {
  FragColor = vec4(rgb, 1.0);
}

