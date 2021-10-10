#ifndef LIB_RAY
#define LIB_RAY
    #include <materials.glsl>
    #include <math.glsl>

    struct Ray {
        vec3 origin;
        vec3 direction;
    };

    struct Raycastresult {
        bool hit;
        vec3 normal, position;
        Material material;
    };

    //for point moving
    vec3 ray_move_point(Ray ray, float d){
        return lerp(ray.origin, ray.direction, d);
    }
#endif