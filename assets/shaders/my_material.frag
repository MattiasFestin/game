#version 450
#include <lib.glsl>

layout(location = 0) out vec4 o_Target;
layout(set = 2, binding = 0) uniform MyMaterial_color {
    vec4 color;
};
void main() {
    o_Target = color;
}