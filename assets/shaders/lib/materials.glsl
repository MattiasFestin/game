#ifndef LIB_MATERIAL
#define LIB_MATERIAL
    #include <constants.glsl>
    #include <math.glsl>
    #include <structs.glsl>
    #include <ray.glsl>

    Ray snell(Raycast_result rcr, Ray light_ray) {
        vec3 dir = 2.0 * dot(rcr.normal, light_ray.dir) * (rcr.normal - light_ray.dir);
        vec3 pos = lerp(rcr.pos, dir, 0.01);
        return Ray(pos, dir);
    }

    float fresnel(float na,  float nb, vec3 incidence, vec3 normal, float min, float max) {
        float r = pow(
            (na - nb) / (na + nb), 
            2.0
        );
        float cx = -dot(normal, incidence);
                    
        if (na > nb) {
            float q = na / nb;
            float s2 = pow(q, 2.0) * 
                (1.0 - pow(cx, 2.0));
            
            if (s2 > 1.0) {
                return max;
            }

            cx = sqrt(1.0 - s2);
        }
                    
        float x = 1.0 - cx;
        float ret = r + (1.0 - r) * pow(x, 5.0);
        return mix(min, max, ret);
    }

    float oren_nayar_diffuse(Raycast_result rcr, Ray light_ray) {
        vec3 view_dir = rcr.r.dir;
        vec3 normal = rcr.normal;

        float l_v = dot(light_ray.dir, view_dir);
        float n_l = dot(light_ray.dir, normal);
        float n_v = dot(normal, view_dir);

        float sigma2 = rcr.m.roughness * rcr.m.roughness;
        float A = 1.0 + sigma2 * (rcr.m.albedo / (sigma2 + 0.13) + 0.5 / (sigma2 + 0.33));
        float B = 0.45 * sigma2 / (sigma2 + 0.09);

        float s = l_v - n_l * n_v;
        float t = mix(1.0, max(n_l, n_v), step(0.0, s));

        return rcr.m.albedo * max(0.0, n_l) * (A + B * s / t) / C_PI;
    }

    float plancks_law(float freq, float temp) {
        float a = (C_PLANCK_CONSTANT * freq);
        float f2 = freq * freq;
        float top = 2.0 * a * f2 / (C_LIGHTSPEED * C_LIGHTSPEED);
        float bottom = exp(a / (C_BOLTZMANN_CONSTANT * temp)) - 1.0;
            
        return top / bottom;
    }

    // vec3 plancks_law_rgb(float temp) {
    //     float r = plancks_law(C_R_FREQ, temp);
    //     float g = plancks_law(C_G_FREQ, temp);
    //     float b = plancks_law(C_B_FREQ, temp);

    //     //TODO: Matrix mul
    //     float r_s = (1.0 * r + 1.2 * g + 1.0 * b) * 0.70;
    //     float g_s = (0.4 * r + 1.0 * g + 1.2 * b) * 0.85;
    //     float b_s = 0.1 * r + 0.5 * g + 1.0 * b;

    //     return XYZtosRGB(vec3(r_s, g_s, b_s));
    // }

    // vec3 color_shifted_plank_law_rgb(float temp) {
    //     vec3 c = plancks_law_rgb(temp);
    //     float m = max3(c);
    //     m = 1 / pow(m + 1e-10, 0.97);
    //     return c * m;
    // }
    
#endif