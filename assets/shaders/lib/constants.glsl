#ifndef LIB_CONSTANTS
#define LIB_CONSTANTS
    //Math constants
    #define C_PI    3.141592653589793
    #define C_TAU   6.283185307179586

    //physics constants
    #define C_LIGHTSPEED            299792458.0
    #define C_PLANCK_CONSTANT       6.62607015e-34
    #define C_BOLTZMANN_CONSTANT    1.380649e-23
    #define C_WIEN                  2.897771955e-3

    //Light constants
    #define C_IR_WAVELEN 750.0 //nm
    #define  C_R_WAVELEN 620.0 //nm
    #define  C_G_WAVELEN 580.0 //nm
    #define  C_B_WAVELEN 485.0 //nm
    #define C_UV_WAVELEN 370.0 //nm

    #define C_IR_FREQ 3.997232773333e14
    #define  C_R_FREQ 4.835362225806e14
    #define  C_G_FREQ 5.168835482759e14
    #define  C_B_FREQ 6.181287793814e14
    #define  C_V_FREQ 8.102498864865e14
    #define  C_RGB_FREQ vec3(C_R_FREQ, C_G_FREQ, C_B_FREQ)

    #define INFINITY 1.0e30

    
#endif