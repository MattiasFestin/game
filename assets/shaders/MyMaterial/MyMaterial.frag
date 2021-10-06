#version 450
#include <lib.glsl>
layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;
layout(location = 1) in vec3 fNormal;
layout(set = 2, binding = 0) uniform MyMaterial_time {
    float time;
};
void main() {
    // float speed = 0.7;
    // float translation = sin(time * speed);
    // float percentage = 0.6;
    // float threshold = v_Uv.x + translation * percentage;
    // vec3 red = vec3(1., 0., 0.);
    // vec3 blue = vec3(0., 0., 1.);
    // vec3 mixed = mix(red, blue, threshold);
    // vec3 c = plancks_law_rgb(mod(time * 1000.0, 50000.0));
    vec3 c = color_shifted_plank_law_rgb(2000.0+7000.0*sin(time/(0.5 * C_PI)));
    o_Target = vec4(c, 1.0);
}