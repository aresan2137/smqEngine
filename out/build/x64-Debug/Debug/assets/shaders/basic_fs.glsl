#version 460 core

layout(location = 0) in vec2 o_UV;
layout(location = 1) in vec3 o_Normal;

layout(binding = 2) uniform sampler2D u_Tex0;

layout(location = 0) out vec4 color;

void main() {
    vec3 lightDir = normalize(vec3(0.5, 1.0, 0.3));
    vec3 normal = normalize(o_Normal);
    
    float diff = max(dot(normal, lightDir), 0.0);
    float ambient = 0.2;
    float light = ambient + diff * 0.8;
    
    vec4 texColor = texture(u_Tex0, o_UV);
    
    color = vec4(texColor.rgb * light, texColor.a);
}