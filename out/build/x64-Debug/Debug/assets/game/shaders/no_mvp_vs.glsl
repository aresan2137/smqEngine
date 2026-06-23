#version 330 core

layout(location = 0) in vec4 position;
layout(location = 1) in vec2 texCoord;

out vec2 o_texCoord;

void main() {
    
    gl_Position = position;
    o_texCoord = texCoord;

}