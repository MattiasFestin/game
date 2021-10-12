#ifndef LIB_STRUCTS
#define LIB_STRUCTS
    struct Chromaticities {
        vec2 R, G, B, W;
    };

    struct Material {
        int type;

        float roughness;
        float reflectance;
        float albedo;

        vec3 color;
        vec3 emission;
    };

    struct Ray {
        vec3 pos;
        vec3 dir;
    };


    struct Raycast_result {
        bool hit;
        Ray r;
        vec3 normal;
        vec3 pos;
        Material m;
    };

    #define NON_RAY Ray(vec3(0.0), vec3(0.0))
    #define NON_MATERAL Material(0, 0.0, 0.0, 0.0, vec3(0.0), vec3(0.0))

    #define LIGHT_MATERAL Material(-1, 0.0, 0.0, 0.0, vec3(1.0), vec3(1.0))
    
    #define CHAULK_MATERAL Material(1, 0.5, 0.0, 0.5, vec3(1.0, 0.5, 0.2), vec3(0.0))
    #define MIRROR_MATERIAL Material(2, 0.0, 1.0, 0.0, vec3(1.0), vec3(0.0))
#endif