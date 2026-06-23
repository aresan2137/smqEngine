#version 460 core

layout(location = 0) in vec2 o_UV;
layout(location = 1) in vec3 o_Normal;
layout(location = 2) in vec3 v_WorldPos;

layout(binding = 2) uniform sampler2D u_Tex0;

layout(location = 0) out vec4 colorOut;
layout(location = 1) out vec4 positionOut;
layout(location = 2) out vec4 normal;

void main() {
    colorOut = texture(u_Tex0, o_UV);
    
    positionOut = vec4(v_WorldPos, 1.0);

    normal = vec4(o_Normal, 1.0);
}