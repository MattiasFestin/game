struct Material{
    int type;
    float roughness, reflectance;
    vec3 color, emmision;
};

/*
 * fresnel approximation
 */
float fresnel (float na,  float nb, vec3 incidence, vec3 normal, float min, float max) {
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
