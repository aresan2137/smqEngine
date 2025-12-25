#version 330 core

layout(location = 0) out vec4 color;
in vec2 o_texCoord;

uniform sampler2D u_tex;
uniform sampler2D u_depth;


uniform float u_fogNear;
uniform float u_fogFar;
uniform vec3 u_fogColor;

void main() {
    vec3 colori = texture(u_tex, o_texCoord).rgb;
    float depth = texture(u_depth, o_texCoord).r;

    float z = depth * 2.0 - 1.0; 
    float linearDepth = (2.0 * 0.1 * 1000.0) / (1000.0 + 0.1 - z * (1000.0 - 0.1));

    float fogFactor = clamp((linearDepth - u_fogNear) / (u_fogFar - u_fogNear), 0.0, 1.0);

    vec3 finalColor = mix(colori, u_fogColor, fogFactor);
    color = vec4(finalColor, 1.0);
    //color = vec4(depth,depth,depth, 1.0);
}