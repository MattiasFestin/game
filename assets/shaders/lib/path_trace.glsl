#ifndef LIB_PATHTRACE
#define LIB_PATHTRACE
    vec3 cosineWeightedSample(vec3 normal, vec2 random){
        vec3 u = normalize(cross(normal,vec3(1.0,1.0,1.0)));
        vec3 v = cross(u,normal);
        float a = sqrt(random.y);
        float x = a*cos(tau*random.x); 
        float y = a*sin(tau*random.x);
        float z = sqrt(1.0-random.y);

        /*
            For more efficiency we can generate proportionally
            fewer rays where the cos term is small. That is,
                pdf(x) = cos (t) / constant
            where constant = pi, by normalization over hemisphere.
                Lo = (1 / N) sum [ ((c / pi) Li cos (t)) / (cos(t)/pi) ]
                = (c / N) sum [Li]
            Yay!
            */

        return normalize(vec3(x*u+y*v+z*normal));
    }
    //crude ray offset
    vec3 offset(vec3 direction, vec3 multiplier, float rand){
        vec3 random = rand * multiplier - multiplier / 2.0;
        return normalize(direction+random);
    }

    /*
    * raycast function to test intersection with each object
    */

    Raycastresult raycast(Ray ray){
        //will be used for depth testing
        float depth = far;
        Raycastresult hit;
        
        // //test spheres
        // for(int i=0;i<balls.length();i++){
        // 	Sphere ball = balls[i];
        // 	float bd = sphere(ray,ball);
        // 	if(bd>0.0&&bd<depth){
        // 		depth = bd;
        // 		vec3 position = mp(ray,bd);
        // 		vec3 normal = normalize(position-(ball.position/2.0));
        // 		hit = Raycastresult(true,normal,position,ball.material);
        // 	}
        // }
        
        // //test boxes
        // for(int i=0;i<boxes.length();i++){
        // 	Box block = boxes[i];
        // 	vec4 bd = box(ray,block);
        // 	if(bd.w>0.0&&bd.w<depth){
        // 		depth = bd.w;
        // 		vec3 position = mp(ray,bd.w);
        // 		vec3 normal = bd.xyz;
        // 		hit = Raycastresult(true,normal,position,block.material);
        // 	}
        // }
        
        //if ray missed then return a empty raycast result
        if (depth==far) {
            hit = Raycastresult(
                false,
                vec3(0.0),
                vec3(0.0),
                Material(MATERIALTYPE_DIFFUSE,0.0,0.0,vec3(0.0),clearColor)
            );
        }
        
        return hit;
    }

    /*
    * main pathtracing function
    */
    
    vec3 trace(Ray ray){
        vec3 accumulator;
        vec3 mask = vec3(1.0);
        
        //pathtracing loop
        for(int i=0;i<bounces;i++){
            //raycast
            Raycastresult result = raycast(ray);
            
            //accumulate color
            accumulator+=mask*result.material.emmision;
            mask*=result.material.color;
            
            //if ray hit a light or missed then stop
            if(result.hit==false||length(result.material.emmision)>0.0) break;
            
            //create ray direction based on material
            switch(result.material.type){
                case MATERIALTYPE_DIFFUSE:
                    //randomly reflect if material has any reflectance value
                    if (fresnel(1.0,GLASS_IOR,ray.direction,result.normal,0.0,1.0)*result.material.reflectance>h()) {
                        ray.origin = result.position+result.normal*1e-4;
                        ray.direction = reflect(ray.direction,result.normal);
                        
                        //offset ray direction based on roughness
                        ray.direction = offset(ray.direction,vec3(result.material.roughness));
                    } else {
                        ray.origin = result.position+result.normal*1e-4;
                        ray.direction = cosineWeightedSample(result.normal);
                    }
                break;
                case MATERIALTYPE_GLASS:
                    //randomly reflect for fresnel
                    if (fresnel(1.0,GLASS_IOR,ray.direction,result.normal,0.0,1.0)-0.07>h()) {
                        ray.origin = result.position+result.normal*1e-4;
                        ray.direction = reflect(ray.direction,result.normal);
                    } else {
                        ray.origin = result.position+ray.direction*1e-4;
                        ray.direction = refract(ray.direction,result.normal,1.0/GLASS_IOR);	
                    }
                break;
            }
        }
        
        return accumulator;
    }
#endif