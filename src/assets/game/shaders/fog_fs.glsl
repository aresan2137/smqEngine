#version 330 core

layout(location = 0) out vec4 color;
in vec2 o_texCoord;

uniform sampler2D u_tex;
uniform sampler2D u_depth;

// fog
uniform float u_fogNear;
uniform float u_fogFar;
uniform vec3  u_fogColor;

uniform float u_crtStrength;

// color correction


void main() {

    float u_saturation = 1.2; // np. 1.6
    float u_contrast = 1.0;   // np. 1.25
    float u_gamma = 1.2;      // np. 1.9
    

    // ===== CRT barrel distortion =====
    vec2 uv = o_texCoord * 2.0 - 1.0;
    float r2 = dot(uv, uv);
    uv += uv * r2 * u_crtStrength;
    vec2 finalUV = uv * 0.4 + 0.5;

    // ===== sample =====
    vec3 col = texture(u_tex, finalUV).rgb;
    float depth = texture(u_depth, finalUV).r;

    // ===== linear depth =====
    float z = depth * 2.0 - 1.0;
    float linearDepth = (2.0 * 0.1 * 1000.0) /
                        (1000.0 + 0.1 - z * (1000.0 - 0.1));

    // ===== fog =====
    float fogFactor = clamp(
        (linearDepth - u_fogNear) / (u_fogFar - u_fogNear),
        0.0, 1.0
    );
    col = mix(col, u_fogColor, fogFactor);

    // ===== saturation =====
    float luma = dot(col, vec3(0.2126, 0.7152, 0.0722));
    col = mix(vec3(luma), col, u_saturation);

    // ===== contrast =====
    col = (col - 0.5) * u_contrast + 0.5;

    // ===== gamma =====
    col = pow(col, vec3(1.0 / u_gamma));

    color = vec4(col, 1.0);
}
