#version 300 es
precision mediump float;

out vec4 f_color;
in vec3 vColor;
in float t;
in vec2 res;

void main(void){
    vec2 cPos=-1.+2.*gl_FragCoord.xy/res.xy;
    float cLength=length(cPos);
    
    vec2 uv=gl_FragCoord.xy/res.xy+(cPos/cLength)*cos(cLength*12.-t*3.)*.2;
    vec3 col=vec3(uv.xy,vColor.z);
    
    f_color=vec4(col,1.);
    
}
