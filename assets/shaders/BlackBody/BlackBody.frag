#version 450
#include <lib.glsl>

layout(set = 2, binding = 0) uniform BlackBody_temperature {
    float temperature;
};
void main() {
    // vec3 c = plancks_law_rgb(temperature);
    // vec3 c = color_shifted_plank_law_rgb(temperature);
    o_Target = vec4(1.0);
}