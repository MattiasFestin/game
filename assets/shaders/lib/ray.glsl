#ifndef LIB_RAY
#define LIB_RAY
    #include <math.glsl>

    //for point moving
    vec3 ray_move_point(Ray ray, float d){
        return lerp(ray.pos, ray.dir, d);
    }
#endif