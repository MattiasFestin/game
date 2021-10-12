#version 450
#include <lib.glsl>
#include <uniforms.glsl>
// #define numOfLights 0
// #define topBVHIndex 0

// #include <pathtrace/lib.glsl>

// uniform float iTime = 0.0;
// layout(set = 3, binding = 0) uniform sampler u_texture;

layout(set = 2, binding = 0) uniform PathTracer_width {
    float width;
};
layout(set = 2, binding = 1) uniform PathTracer_height {
    float height;
};
layout(set = 2, binding = 2) uniform PathTracer_time {
    float time;
};
layout(set = 2, binding = 3) uniform PathTracer_samples {
    int samples;
};
layout(set = 2, binding = 4) uniform PathTracer_pathlenght {
    int pathlenght;
};
#define eps 0.0001
// #define EYEPATHLENGTH 10
// #define SAMPLES 20

// #define MOTIONBLUR
// #define MOTIONBLURFPS 100.

#define LIGHTCOLOR color_shifted_plank_law_rgb(6500.0)
#define WHITECOLOR vec3(.7295, .7355, .729)*0.7
#define GREENCOLOR vec3(.117, .4125, .115)*0.7
#define REDCOLOR vec3(.611, .0555, .062)*0.7


float hash1(inout float seed) {
    return fract(sin(seed += 0.1)*43758.5453123);
}

vec2 hash2(inout float seed) {
    return fract(sin(vec2(seed+=0.1,seed+=0.1))*vec2(43758.5453123,22578.1459123));
}

vec3 hash3(inout float seed) {
    return fract(sin(vec3(seed+=0.1,seed+=0.1,seed+=0.1))*vec3(43758.5453123,22578.1459123,19642.3490423));
}

//-----------------------------------------------------
// Intersection functions (by iq)
//-----------------------------------------------------

vec3 nSphere( in vec3 pos, in vec4 sph ) {
    return (pos-sph.xyz)/sph.w;
}

float iSphere( in vec3 ro, in vec3 rd, in vec4 sph ) {
    vec3 oc = ro - sph.xyz;
    float b = dot(oc, rd);
    float c = dot(oc, oc) - sph.w * sph.w;
    float h = b * b - c;
    if (h < 0.0) return -1.0;

	float s = sqrt(h);
	float t1 = -b - s;
	float t2 = -b + s;
	
	return t1 < 0.0 ? t2 : t1;
}

vec3 nPlane( in vec3 ro, in vec4 obj ) {
    return obj.xyz;
}

float iPlane( in vec3 ro, in vec3 rd, in vec4 pla ) {
    return (-pla.w - dot(pla.xyz,ro)) / dot( pla.xyz, rd );
}

//-----------------------------------------------------
// scene
//-----------------------------------------------------

vec3 cosWeightedRandomHemisphereDirection( const vec3 n, inout float seed ) {
  	vec2 r = hash2(seed);
    
	vec3  uu = normalize( cross( n, vec3(0.0,1.0,1.0) ) );
	vec3  vv = cross( uu, n );
	
	float ra = sqrt(r.y);
	float rx = ra*cos(6.2831*r.x); 
	float ry = ra*sin(6.2831*r.x);
	float rz = sqrt( 1.0-r.y );
	vec3  rr = vec3( rx*uu + ry*vv + rz*n );
    
    return normalize( rr );
}

vec3 randomSphereDirection(inout float seed) {
    vec2 h = hash2(seed) * vec2(2.,6.28318530718)-vec2(1,0);
    float phi = h.y;
	return vec3(sqrt(1.-h.x*h.x)*vec2(sin(phi),cos(phi)),h.x);
}

vec3 randomHemisphereDirection( const vec3 n, inout float seed ) {
	vec3 dr = randomSphereDirection(seed);
	return dot(dr,n) * dr;
}

//-----------------------------------------------------
// light
//-----------------------------------------------------

vec4 lightSphere;

void initLightSphere( float time ) {
	lightSphere = vec4( 3.0+2.*sin(time),2.8+2.*pow(sin(time*0.9), 2.0),3.0+4.*cos(time*0.7), .5 );
}

vec3 sampleLight( const in vec3 ro, inout float seed ) {
    vec3 n = randomSphereDirection( seed ) * lightSphere.w;
    return lightSphere.xyz + n;
}

//-----------------------------------------------------
// scene
//-----------------------------------------------------

vec2 intersect( in vec3 ro, in vec3 rd, inout vec3 normal ) {
	vec2 res = vec2( 1e20, -1.0 );
    float t;
	
	t = iPlane( ro, rd, vec4( 0.0, 1.0, 0.0,0.0 ) ); if( t>eps && t<res.x ) { res = vec2( t, 1. ); normal = vec3( 0., 1., 0.); }
	t = iPlane( ro, rd, vec4( 0.0, 0.0,-1.0,8.0 ) ); if( t>eps && t<res.x ) { res = vec2( t, 1. ); normal = vec3( 0., 0.,-1.); }
    t = iPlane( ro, rd, vec4( 1.0, 0.0, 0.0,0.0 ) ); if( t>eps && t<res.x ) { res = vec2( t, 2. ); normal = vec3( 1., 0., 0.); }

    t = iPlane( ro, rd, vec4( 0.0,-1.0, 0.0,5.49) ); if( t>eps && t<res.x ) { res = vec2( t, 1. ); normal = vec3( 0., -1., 0.); }
    t = iPlane( ro, rd, vec4(-1.0, 0.0, 0.0,5.59) ); if( t>eps && t<res.x ) { res = vec2( t, 3. ); normal = vec3(-1., 0., 0.); }

	t = iSphere( ro, rd, vec4( 1.5,1.0, 2.7, 1.0) ); if( t>eps && t<res.x ) { res = vec2( t, 1. ); normal = nSphere( ro+t*rd, vec4( 1.5,1.0, 2.7,1.0) ); }
    t = iSphere( ro, rd, vec4( 4.0,1.0, 4.0, 1.0) ); if( t>eps && t<res.x ) { res = vec2( t, 6. ); normal = nSphere( ro+t*rd, vec4( 4.0,1.0, 4.0,1.0) ); }
    t = iSphere( ro, rd, lightSphere ); if( t>eps && t<res.x ) { res = vec2( t, 0.0 );  normal = nSphere( ro+t*rd, lightSphere ); }
					  
    return res;					  
}

bool intersectShadow( in vec3 ro, in vec3 rd, in float dist ) {
    float t;
	
	t = iSphere( ro, rd, vec4( 1.5,1.0, 2.7,1.0) );  if( t>eps && t<dist ) { return true; }
    t = iSphere( ro, rd, vec4( 4.0,1.0, 4.0,1.0) );  if( t>eps && t<dist ) { return true; }

    return false; // optimisation: planes don't cast shadows in this scene
}

//-----------------------------------------------------
// materials
//-----------------------------------------------------

vec3 matColor( const in float mat ) {
	vec3 nor = vec3(0., 0.95, 0.);
	
	if( mat<3.5 ) nor = REDCOLOR;
    if( mat<2.5 ) nor = GREENCOLOR;
	if( mat<1.5 ) nor = WHITECOLOR;
	if( mat<0.5 ) nor = LIGHTCOLOR;
					  
    return nor;					  
}

bool matIsSpecular( const in float mat ) {
    return mat > 4.5;
}

bool matIsLight( const in float mat ) {
    return mat < 0.5;
}

//-----------------------------------------------------
// brdf
//-----------------------------------------------------

vec3 getBRDFRay( in vec3 n, const in vec3 rd, const in float m, inout bool specularBounce, inout float seed ) {
    specularBounce = false;
    
    vec3 r = cosWeightedRandomHemisphereDirection( n, seed );
    if(  !matIsSpecular( m ) ) {
        return r;
    } else {
        specularBounce = true;
        
        float n1, n2, ndotr = dot(rd,n);
        
        if( ndotr > 0. ) {
            n1 = 1.0; 
            n2 = 1.5;
            n = -n;
        } else {
            n1 = 1.5;
            n2 = 1.0; 
        }
                
        float r0 = (n1-n2)/(n1+n2); r0 *= r0;
		float fresnel = r0 + (1.-r0) * pow(1.0-abs(ndotr),5.);
        
        vec3 ref =  refract( rd, n, n2/n1 );        
        if( ref == vec3(0) || hash1(seed) < fresnel ) {
            ref = reflect( rd, n );
        } 
        
        return ref;
	}
}

//-----------------------------------------------------
// eyepath
//-----------------------------------------------------

vec3 traceEyePath( in vec3 ro, in vec3 rd, const in bool directLightSampling, inout float seed ) {
    vec3 tcol = vec3(0.);
    vec3 fcol  = vec3(1.);
    
    bool specularBounce = true;
    
    for( int j=0; j<pathlenght; ++j ) {
        vec3 normal;
        
        vec2 res = intersect( ro, rd, normal );
        if( res.y < -0.5 ) {
            return tcol;
        }
        
        if( matIsLight( res.y ) ) {
            if( directLightSampling ) {
            	if( specularBounce ) tcol += fcol*LIGHTCOLOR;
            } else {
                tcol += fcol*LIGHTCOLOR;
            }
            return tcol;
        }
        
        ro = ro + res.x * rd;
        rd = getBRDFRay( normal, rd, res.y, specularBounce, seed );
            
        if(!specularBounce || dot(rd,normal) < 0.) {  
        	fcol *= matColor( res.y );
        }
        
        if( directLightSampling ) {
        	vec3 ld = sampleLight( ro, seed ) - ro;
			vec3 nld = normalize(ld);
            if( !specularBounce && j < pathlenght-1 && !intersectShadow( ro, nld, length(ld)) ) {

                float cos_a_max = sqrt(1. - clamp(lightSphere.w * lightSphere.w / dot(lightSphere.xyz-ro, lightSphere.xyz-ro), 0., 1.));
                float weight = 2. * (1. - cos_a_max);

                tcol += (fcol * LIGHTCOLOR) * (weight * clamp(dot( nld, normal ), 0., 1.));
            }
        }
    }    
    return tcol;
}

void main() {
    vec4 pos = gl_FragCoord;
    // pos.x = 1.0 - pos.x;
    // pos.y = 1.0 - pos.y;
    pos.z = 1.0 - pos.z;

    vec2 u_resolution = vec2(width, height);

    vec2 q = pos.xy / u_resolution.xy;
        
    float splitCoord = u_resolution.x;//(iMouse.x == 0.0) ? iResolution.x/2. + iResolution.x*cos(iTime*.5) : iMouse.x;
    bool directLightSampling = false;
    
    //-----------------------------------------------------
    // camera
    //-----------------------------------------------------

    vec2 p = -1.0 + 2.0 * (pos.xy) / u_resolution.xy;
    p.x *= u_resolution.x/u_resolution.y;

    float seed = p.x + p.y * 3.43121412313 + fract(1.12345314312*time);

    vec3 ro = vec3(2.78, 2.73, -8.00);
    vec3 ta = vec3(2.78, 2.73,  0.00);
    vec3 ww = normalize( ta - ro );
    vec3 uu = normalize( cross(ww,vec3(0.0,1.0,0.0) ) );
    vec3 vv = normalize( cross(uu,ww));

    //-----------------------------------------------------
    // render
    //-----------------------------------------------------

    vec3 col = vec3(0.0);
    vec3 tot = vec3(0.0);
    vec3 uvw = vec3(0.0);
    
    for( int a=0; a<samples; a++ ) {

        vec2 rpof = 2.*(hash2(seed)-vec2(0.5)) / u_resolution.y;
        vec3 rd = normalize( (p.x+rpof.x)*uu + (p.y+rpof.y)*vv + 3.0*ww );
        
        vec3 fp = ro + rd * 12.0;
        vec3 rof = ro + (uu*(hash1(seed)-0.5) + vv*(hash1(seed)-0.5))*0.125;
        rd = normalize( fp - rof );   
        
        initLightSphere( time );        

        
        col = traceEyePath( rof, rd, directLightSampling, seed );

        tot += col;
        
        seed = mod( seed*1.1234567893490423, 13. );
    }
    
    tot /= float(samples);
    tot = pow( clamp(tot,0.0,1.0), vec3(0.45) );

    // float pixel[4];
    // glReadPixels(pos.x, pos.y, 1, 1, 0, 0, &pixel);
    // vec4 old = vec4(pixel[0], pixel[1], pixel[2], pixel[3]);

    o_Target = vec4( tot, 1.0 );

    // float ar = width / height;
    // vec2 offset = vec2(width/4.0, 0.0);
    // vec2 st = (gl_FragCoord.xy - offset)/(u_resolution.xy); //* vec2(1/ar, 1.0));

    //Z-ordering
    // Sphere[] spheres = {
    //     Sphere(0.5,vec3(0.35, 0.8, 0.4), CHAULK_MATERAL),
    //     Sphere(0.5,vec3(0.15, 0.2, 0.4), MIRROR_MATERIAL),
    //     Sphere(0.5, vec3(0.25, 0.0, 5.0), LIGHT_MATERAL),
    // };

    // // Sphere s = Sphere(0.5,vec3(0.25, 0.5, 0.2), CHAULK_MATERAL);
    // Ray r = Ray(vec3(st, 0.0), vec3(0.0));

    // vec3 color = ray_cast(spheres, r);
    // // for (int i = 0; i < 2; i++) {
        
    // //     if (length(color) > 0) {
    // //         break;
    // //     }
    // // }

    // InitRNG(gl_FragCoord.xy, 1);

    // float r1 = 2.0 * rand();
    // float r2 = 2.0 * rand();

    // vec2 jitter;
    // jitter.x = r1 < 1.0 ? sqrt(r1) - 1.0 : 1.0 - sqrt(2.0 - r1);
    // jitter.y = r2 < 1.0 ? sqrt(r2) - 1.0 : 1.0 - sqrt(2.0 - r2);

    // jitter /= (screenResolution * 0.5);
    // vec2 d = (2.0 * TexCoords - 1.0) + jitter;

    // float scale = tan(camera.fov * 0.5);
    // d.y *= screenResolution.y / screenResolution.x * scale;
    // d.x *= scale;
    // vec3 rayDir = normalize(d.x * camera.right + d.y * camera.up + camera.forward);

    // vec3 focalPoint = camera.focalDist * rayDir;
    // float cam_r1 = rand() * TWO_PI;
    // float cam_r2 = rand() * camera.aperture;
    // vec3 randomAperturePos = (cos(cam_r1) * camera.right + sin(cam_r1) * camera.up) * sqrt(cam_r2);
    // vec3 finalRayDir = normalize(focalPoint - randomAperturePos);

    // Ray ray = Ray(camera.position + randomAperturePos, finalRayDir);

    // vec3 pixelColor = PathTrace(ray);

    // o_Target = vec4(1.0);
}