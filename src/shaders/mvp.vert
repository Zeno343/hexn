#version 460
layout (location = 0) in vec3 vPos; 
layout (location = 1) in vec3 vRgb;

layout (location = 0) uniform mat4 view;
layout (location = 1) uniform mat4 proj;

out vec3 fRgb;

void main() {
  gl_Position = proj * view * vec4(vPos, 1.0);
  fRgb = vRgb;
}
