#ifndef LIB_MATH
#define LIB_MATH
    vec3 lerp(vec3 origin , vec3 dir , float x) {
        return origin + dir * x;
    }

    float max3 (vec3 v) {
        return max(max(v.x, v.y), v.z);
    }

    float normal_pdf(float x, float m, float d) {
        float y = (x-m)/d;
        return exp(-0.5*y*y) / (d * sqrt(2.0*C_PI));
    }
#endif