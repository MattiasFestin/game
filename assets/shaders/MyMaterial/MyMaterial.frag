#version 450
layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;
layout(set = 2, binding = 0) uniform MyMaterial_value {
    float time;
};
void main() {
    float speed = 0.7;
    float translation = sin(time * speed);
    float percentage = 0.6;
    float threshold = v_Uv.x + translation * percentage;
    vec3 red = vec3(1., 0., 0.);
    vec3 blue = vec3(0., 0., 1.);
    vec3 mixed = mix(red, blue, threshold);
    o_Target = vec4(mixed, 1.0);
}