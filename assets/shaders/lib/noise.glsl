#ifndef LIB_EASING
#define LIB_EASING
    #define BIT_NOISE1 0xB5297A4D
    #define BIT_NOISE2 0x68E31DA4
    #define BIT_NOISE3 0x1B56C4E9

    #define PRIME_Y 675847181
    #define PRIME_Z 259263317

    #define PRIME_Yf float(PRIME_Y)
    #define PRIME_Zf float(PRIME_Z)

    int squirle3i(int x, int seed) {
        int mangled = x;
        mangled *= BIT_NOISE1;
        mangled += seed;
        mangled ^= mangled >> 8;
        mangled += BIT_NOISE2;
        mangled ^= mangled << 8;
        mangled *= BIT_NOISE3;
        mangled ^= mangled >> 8;
        return mangled;
    }

    float squirle3f(float x, int seed) {
        return float(squirle3i(int(x), seed)) / 2147483647.0;
    }

    int noise(int x, int seed) {
        return squirle3i(x, seed);
    }

    int noise(ivec2 p, int seed) {
        return squirle3i(p.x + p.y * PRIME_Y, seed);
    }

    int noise(ivec3 p, int seed) {
        return squirle3i(p.x + p.y * PRIME_Y + p.z * PRIME_Z, seed);
    }

    float noise(float x, int seed) {
        return squirle3f(x, seed);
    }

    float noise(vec2 p, int seed) {
        return squirle3f(p.x + p.y * PRIME_Yf, seed);
    }

    float noise(vec3 p, int seed) {
        return squirle3f(p.x + p.y * PRIME_Yf + p.z * PRIME_Zf, seed);
    }
#endif