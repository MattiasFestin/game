#include "./lib.glsl"

struct Ray {
    vec3 origin, direction;
};

struct Raycastresult{
    bool hit;
    vec3 normal, position;
    Material material;
};

//for point moving
vec3 ray_move_point(Ray ray, float d){
	return lerp(ray.origin, ray.direction, d);
}