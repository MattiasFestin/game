#ifndef LIB_ROTATE
#define LIB_ROTATE
	mat4 rotateY(float rotation) {
		rotation = radians(rotation);
		float ys = sin(rotation);
		float yc = cos(rotation);
		float yoc = 1.0-yc;
		return mat4(yc,0.0,ys,0.0,
					0.0,yoc+yc,0.0,0.0,
					-ys,0.0,yc,0.0,
					0.0,0.0,0.0,1.0);
	}

	mat4 rotateX(float rotation) {
		rotation = radians(rotation);
		float xs = sin(rotation);
		float xc = cos(rotation);
		float xoc = 1.0-xc;
		return mat4(xoc+xc,0.0,0.0,0.0,
					0.0,xc,-xs,0.0,
					0.0,xs,xc,0.0,
					0.0,0.0,0.0,1.0);
	}

	mat4 rotateZ(float rotation) {
		rotation = radians(rotation);
		float zs = sin(rotation);
		float zc = cos(rotation);
		float zoc = 1.0-zc;
		return mat4(zc,zs,0.0,0.0,
					-zs,zc,0.0,0.0,
					0.0,0.0,zoc+zc,0.0,
					0.0,0.0,0.0,1.0);
	}
#endif