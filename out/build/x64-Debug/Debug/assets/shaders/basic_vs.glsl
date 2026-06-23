#version 460 core

layout(location = 0) uniform mat4 u_MVP;  
layout(location = 1) uniform mat4 u_Model;

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 UV;
layout(location = 2) in vec3 normal;

layout(location = 0) out vec2 o_UV;
layout(location = 1) out vec3 o_Normal;

void main() {
    o_UV = UV;
    o_Normal = (u_Model * vec4(normal, 0.0)).xyz;
    
    gl_Position = u_MVP * vec4(position, 1.0);
}