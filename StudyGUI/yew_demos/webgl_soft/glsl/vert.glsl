#version 300 es

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;

uniform mat4 P;
uniform mat4 V;

out vec3 vColor;

include "./glsl/utils.glsl"

void main(void) {
  mat4 rx = Gen3DMatRotateX(radians(0.0));
  mat4 ry = Gen3DMatRotateY(radians(0.0));
  mat4 rz = Gen3DMatRotateZ(radians(0.0));
  mat4 r = rz * ry * rx;

  mat4 s = Gen3DMatScale(vec3(3.0, 3.0, 3.0));
  mat4 t = Gen3DMatTranslate(vec3(0.0, 0.0, 0.0));
  mat4 M = t * s * r;

  gl_Position = P * V * M * vec4(position, 1.0);
  vColor = color;
}