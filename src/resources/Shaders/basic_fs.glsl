#version 330 core

layout(location = 0) out vec4 color;
in vec2 o_texCoord;


uniform sampler2D u_tex;

void main() {

    color = texture(u_tex, o_texCoord);
}
