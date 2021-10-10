#ifndef LIB_COLOR
#define LIB_COLOR
    // Spectrum to xyz approx function from Sloan http://jcgt.org/published/0002/02/01/paper.pdf
    // Inputs:  Wavelength in nanometers
    float xFit_1931( float wave )
    {
        float t1 = (wave-442.0)*((wave<442.0)?0.0624:0.0374),
            t2 = (wave-599.8)*((wave<599.8)?0.0264:0.0323),
            t3 = (wave-501.1)*((wave<501.1)?0.0490:0.0382);
        return 0.362*exp(-0.5*t1*t1) + 1.056*exp(-0.5*t2*t2)- 0.065*exp(-0.5*t3*t3);
    }
    float yFit_1931( float wave )
    {
        float t1 = (wave-568.8)*((wave<568.8)?0.0213:0.0247),
            t2 = (wave-530.9)*((wave<530.9)?0.0613:0.0322);
        return 0.821*exp(-0.5*t1*t1) + 0.286*exp(-0.5*t2*t2);
    }
    float zFit_1931( float wave )
    {
        float t1 = (wave-437.0)*((wave<437.0)?0.0845:0.0278),
            t2 = (wave-459.0)*((wave<459.0)?0.0385:0.0725);
        return 1.217*exp(-0.5*t1*t1) + 0.681*exp(-0.5*t2*t2);
    }

    #define xyzFit_1931(w) vec3( xFit_1931(w), yFit_1931(w), zFit_1931(w) ) 

    // http://www.cie.co.at/technical-work/technical-resources
    vec3 standardObserver1931[] =
        vec3[] (
        vec3( 0.001368, 0.000039, 0.006450 ), // 380 nm
        vec3( 0.002236, 0.000064, 0.010550 ), // 385 nm
        vec3( 0.004243, 0.000120, 0.020050 ), // 390 nm
        vec3( 0.007650, 0.000217, 0.036210 ), // 395 nm
        vec3( 0.014310, 0.000396, 0.067850 ), // 400 nm
        vec3( 0.023190, 0.000640, 0.110200 ), // 405 nm
        vec3( 0.043510, 0.001210, 0.207400 ), // 410 nm
        vec3( 0.077630, 0.002180, 0.371300 ), // 415 nm
        vec3( 0.134380, 0.004000, 0.645600 ), // 420 nm
        vec3( 0.214770, 0.007300, 1.039050 ), // 425 nm
        vec3( 0.283900, 0.011600, 1.385600 ), // 430 nm
        vec3( 0.328500, 0.016840, 1.622960 ), // 435 nm
        vec3( 0.348280, 0.023000, 1.747060 ), // 440 nm
        vec3( 0.348060, 0.029800, 1.782600 ), // 445 nm
        vec3( 0.336200, 0.038000, 1.772110 ), // 450 nm
        vec3( 0.318700, 0.048000, 1.744100 ), // 455 nm
        vec3( 0.290800, 0.060000, 1.669200 ), // 460 nm
        vec3( 0.251100, 0.073900, 1.528100 ), // 465 nm
        vec3( 0.195360, 0.090980, 1.287640 ), // 470 nm
        vec3( 0.142100, 0.112600, 1.041900 ), // 475 nm
        vec3( 0.095640, 0.139020, 0.812950 ), // 480 nm
        vec3( 0.057950, 0.169300, 0.616200 ), // 485 nm
        vec3( 0.032010, 0.208020, 0.465180 ), // 490 nm
        vec3( 0.014700, 0.258600, 0.353300 ), // 495 nm
        vec3( 0.004900, 0.323000, 0.272000 ), // 500 nm
        vec3( 0.002400, 0.407300, 0.212300 ), // 505 nm
        vec3( 0.009300, 0.503000, 0.158200 ), // 510 nm
        vec3( 0.029100, 0.608200, 0.111700 ), // 515 nm
        vec3( 0.063270, 0.710000, 0.078250 ), // 520 nm
        vec3( 0.109600, 0.793200, 0.057250 ), // 525 nm
        vec3( 0.165500, 0.862000, 0.042160 ), // 530 nm
        vec3( 0.225750, 0.914850, 0.029840 ), // 535 nm
        vec3( 0.290400, 0.954000, 0.020300 ), // 540 nm
        vec3( 0.359700, 0.980300, 0.013400 ), // 545 nm
        vec3( 0.433450, 0.994950, 0.008750 ), // 550 nm
        vec3( 0.512050, 1.000000, 0.005750 ), // 555 nm
        vec3( 0.594500, 0.995000, 0.003900 ), // 560 nm
        vec3( 0.678400, 0.978600, 0.002750 ), // 565 nm
        vec3( 0.762100, 0.952000, 0.002100 ), // 570 nm
        vec3( 0.842500, 0.915400, 0.001800 ), // 575 nm
        vec3( 0.916300, 0.870000, 0.001650 ), // 580 nm
        vec3( 0.978600, 0.816300, 0.001400 ), // 585 nm
        vec3( 1.026300, 0.757000, 0.001100 ), // 590 nm
        vec3( 1.056700, 0.694900, 0.001000 ), // 595 nm
        vec3( 1.062200, 0.631000, 0.000800 ), // 600 nm
        vec3( 1.045600, 0.566800, 0.000600 ), // 605 nm
        vec3( 1.002600, 0.503000, 0.000340 ), // 610 nm
        vec3( 0.938400, 0.441200, 0.000240 ), // 615 nm
        vec3( 0.854450, 0.381000, 0.000190 ), // 620 nm
        vec3( 0.751400, 0.321000, 0.000100 ), // 625 nm
        vec3( 0.642400, 0.265000, 0.000050 ), // 630 nm
        vec3( 0.541900, 0.217000, 0.000030 ), // 635 nm
        vec3( 0.447900, 0.175000, 0.000020 ), // 640 nm
        vec3( 0.360800, 0.138200, 0.000010 ), // 645 nm
        vec3( 0.283500, 0.107000, 0.000000 ), // 650 nm
        vec3( 0.218700, 0.081600, 0.000000 ), // 655 nm
        vec3( 0.164900, 0.061000, 0.000000 ), // 660 nm
        vec3( 0.121200, 0.044580, 0.000000 ), // 665 nm
        vec3( 0.087400, 0.032000, 0.000000 ), // 670 nm
        vec3( 0.063600, 0.023200, 0.000000 ), // 675 nm
        vec3( 0.046770, 0.017000, 0.000000 ), // 680 nm
        vec3( 0.032900, 0.011920, 0.000000 ), // 685 nm
        vec3( 0.022700, 0.008210, 0.000000 ), // 690 nm
        vec3( 0.015840, 0.005723, 0.000000 ), // 695 nm
        vec3( 0.011359, 0.004102, 0.000000 ), // 700 nm
        vec3( 0.008111, 0.002929, 0.000000 ), // 705 nm
        vec3( 0.005790, 0.002091, 0.000000 ), // 710 nm
        vec3( 0.004109, 0.001484, 0.000000 ), // 715 nm
        vec3( 0.002899, 0.001047, 0.000000 ), // 720 nm
        vec3( 0.002049, 0.000740, 0.000000 ), // 725 nm
        vec3( 0.001440, 0.000520, 0.000000 ), // 730 nm
        vec3( 0.001000, 0.000361, 0.000000 ), // 735 nm
        vec3( 0.000690, 0.000249, 0.000000 ), // 740 nm
        vec3( 0.000476, 0.000172, 0.000000 ), // 745 nm
        vec3( 0.000332, 0.000120, 0.000000 ), // 750 nm
        vec3( 0.000235, 0.000085, 0.000000 ), // 755 nm
        vec3( 0.000166, 0.000060, 0.000000 ), // 760 nm
        vec3( 0.000117, 0.000042, 0.000000 ), // 765 nm
        vec3( 0.000083, 0.000030, 0.000000 ), // 770 nm
        vec3( 0.000059, 0.000021, 0.000000 ), // 775 nm
        vec3( 0.000042, 0.000015, 0.000000 )  // 780 nm
    );
    float standardObserver1931_w_min = 380.0f;
    float standardObserver1931_w_max = 780.0f;
    int standardObserver1931_length = 81;


    vec3 WavelengthToXYZLinear( float fWavelength )
    {
        float fPos = ( fWavelength - standardObserver1931_w_min ) / (standardObserver1931_w_max - standardObserver1931_w_min);
        float fIndex = fPos * float(standardObserver1931_length);
        float fFloorIndex = floor(fIndex);
        float fBlend = clamp( fIndex - fFloorIndex, 0.0, 1.0 );
        int iIndex0 = int(fFloorIndex);
        int iIndex1 = iIndex0 + 1;
        iIndex1 = min( iIndex1, standardObserver1931_length - 1);

        return mix( standardObserver1931[iIndex0], standardObserver1931[iIndex1], fBlend );
    }

    vec3 XYZtosRGB( vec3 XYZ )
    {
        // XYZ to sRGB
        // http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
    mat3 m = mat3 (
            3.2404542, -1.5371385, -0.4985314,
            -0.9692660,  1.8760108,  0.0415560,
            0.0556434, -0.2040259,  1.0572252 );
        
        return XYZ * m;
    }

    vec3 sRGBtoXYZ( vec3 RGB )
    {
    // sRGB to XYZ
    // http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html

    mat3 m = mat3(  	0.4124564,  0.3575761, 0.1804375,
                        0.2126729,  0.7151522, 0.0721750,
                        0.0193339,  0.1191920, 0.9503041 );
        
        
        return RGB * m;
    }

    vec3 WavelengthToXYZ( float f )
    {    
        //return xyzFit_1931( f ) * mXYZtoSRGB;
        
        return WavelengthToXYZLinear( f );
    }


    struct Chromaticities
    {
        vec2 R, G, B, W;
    };

    // chromatic adaptation

    // http://www.brucelindbloom.com/index.html?Eqn_ChromAdapt.html    

    // Test viewing condition CIE XYZ tristimulus values of whitepoint.
    vec3 XYZ_w = vec3( 1.09850,	1.00000,	0.35585); // Illuminant A
    // Reference viewing condition CIE XYZ tristimulus values of whitepoint.
    vec3 XYZ_wr = vec3(0.95047,	1.00000,	1.08883); // D65


    const mat3 CA_A_to_D65_VonKries = mat3(
        0.9394987, -0.2339150,  0.4281177,
        -0.0256939,  1.0263828,  0.0051761,
        0.0000000,  0.0000000,  3.0598005
        );


    const mat3 CA_A_to_D65_Bradford = mat3(
        0.8446965, -0.1179225,  0.3948108,
        -0.1366303,  1.1041226,  0.1291718,
        0.0798489, -0.1348999,  3.1924009
        );


    const mat3 mCAT_VonKries = mat3 ( 
        0.4002400,  0.7076000, -0.0808100,
        -0.2263000,  1.1653200,  0.0457000,
        0.0000000,  0.0000000,  0.9182200 );

    const mat3 mCAT_02 = mat3( 	0.7328, 0.4296, -0.1624,
                                -0.7036, 1.6975, 0.0061,
                                0.0030, 0.0136, 0.9834 );

    const mat3 mCAT_Bradford = mat3 (  0.8951000, 0.2664000, -0.1614000,
                                    -0.7502000,  1.7135000,  0.0367000,
                                    0.0389000, -0.0685000,  1.0296000 );


    mat3 GetChromaticAdaptionMatrix()
    {
        //return inverse(CA_A_to_D65_VonKries);    
        //return inverse(CA_A_to_D65_Bradford);
            
        //return mat3(1,0,0, 0,1,0, 0,0,1); // do nothing
        
        //mat3 M = mCAT_02;
        //mat3 M = mCAT_Bradford;
        mat3 M = mCAT_VonKries;
        //mat3 M = mat3(1,0,0,0,1,0,0,0,1);
        
        vec3 w = XYZ_w * M;
        vec3 wr = XYZ_wr * M;
        vec3 s = w / wr;
        
        mat3 d = mat3( 
            s.x,	0,		0,  
            0,		s.y,	0,
            0,		0,		s.z );
            
        mat3 cat = M * d * inverse(M);
        return cat;
    }
#endif