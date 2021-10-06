#ifndef LIB_PROJECTIONS
#define LIB_PROJECTIONS
    #include <constants.glsl>
    vec3 to_spherical(vec3 v) {
		float r = length(v);
		float t = acos(v.z / r);
		float p = atan(v.x/v.y);
		if (v.x < 0.0) {
			p += C_PI;
		}
		return vec3(r, t, p);
	}

	vec3 from_spherical(vec3 s) {
		float x = s.x * cos(s.z) * sin(s.y);
		float y = s.x * sin(s.z) * sin(s.y);
		float z = s.r * cos(s.y);
		return vec3(x, y, z);
	}
#endif