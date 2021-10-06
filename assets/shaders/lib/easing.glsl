#ifndef LIB_EASING
#define LIB_EASING
    float smooth_start2(float x) {
        return pow(x, 2.0);
    }
    float smooth_start3(float x) {
        return pow(x, 3.0);
    }

    float smooth_stop2(float x) {
        return 1.0 - pow(1.0 - x, 2.0);
    }
    float smooth_stop3(float x) {
        return 1.0 - pow(1.0 - x, 3.0);
    }

    //NOTE: mix is in OpenGL
    float smooth_step2(float x) {
        return mix(smooth_start2(x), smooth_stop2(x), x);
    }

    float smooth_step3(float x) {
        return mix(smooth_start3(x), smooth_stop3(x), x);
    }

    float asymptotic_averaging(float current, float target, float speed) {
        return current + (target - current) * speed;
    }

    vec3 asymptotic_averaging_3d(vec3 current, vec3 target, float speed) {
        return current + (target - current) * speed;
    }

    vec4 asymptotic_averaging_4d(vec4 current, vec4 target, float speed) {
        return current + (target - current) * speed;
    }
#endif