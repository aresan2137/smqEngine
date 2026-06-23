#version 460 core

layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

layout(binding = 0) uniform sampler2D u_PositionTex;
layout(binding = 3) uniform sampler2D u_Normal;

layout(rgba8, binding = 1) uniform image2D u_OutputTex;
layout(binding = 2) uniform sampler2D u_LUT;

layout(std430, binding = 4) readonly buffer VertexBuffer {
    float vboData[];
};

layout(std430, binding = 5) readonly buffer IndexBuffer {
    uint indices[];
};

struct MeshInfo {
    uint indexStart;
    uint indexEnd;

    vec3 aabbMin;
    float _pad0;

    vec3 aabbMax;
    float _pad1;

    mat4 invModelMatrix;
};

layout(std430, binding = 6) readonly buffer MeshBuffer {
    MeshInfo meshes[];
};

struct Light {
    vec3 pos;
    float power;

    vec3 color;
    float maxReach;

    vec3 aabbMin;
    float _pad0;

    vec3 aabbMax;
    float _pad1;
};

layout(std430, binding = 7) readonly buffer LightBuffer {
    Light lights[];
};

const uint STRIDE = 8;

bool RayAABB(vec3 ro, vec3 rd, vec3 aabbMin, vec3 aabbMax, out float t) {
    vec3 invRd = 1.0 / rd;
    vec3 t0 = (aabbMin - ro) * invRd;
    vec3 t1 = (aabbMax - ro) * invRd;
    vec3 tmin = min(t0, t1);
    vec3 tmax = max(t0, t1);
    float tnear = max(max(tmin.x, tmin.y), tmin.z);
    float tfar = min(min(tmax.x, tmax.y), tmax.z);
    t = tnear;
    return tfar >= tnear && tfar > 0.0;
}

bool RayTriangle(vec3 ro, vec3 rd, vec3 v0, vec3 v1, vec3 v2, out float t) {
    vec3 edge1 = v1 - v0;
    vec3 edge2 = v2 - v0;
    vec3 h = cross(rd, edge2);
    float a = dot(edge1, h);
    if (a > -0.00001 && a < 0.00001) return false;
    float f = 1.0 / a;
    vec3 s = ro - v0;
    float u = f * dot(s, h);
    if (u < 0.0 || u > 1.0) return false;
    vec3 q = cross(s, edge1);
    float v = f * dot(rd, q);
    if (v < 0.0 || u + v > 1.0) return false;
    t = f * dot(edge2, q);
    return t > 0.00001;
}

vec4 PointsColor(vec3 worldPos, vec3 normal) {
    vec3 oute = vec3(0.0, 0.0, 0.0);
    int lightsCount = lights.length();
    
    for (int i = 0; i < lightsCount; i++) {
        vec3 deltapos = lights[i].pos - worldPos;
        float len = length(deltapos);
        vec3 dir = deltapos / len;
        
        float falloff = 1 / (len * 0.5 + 0.01);
        float ndotl = max(dot(normal, dir), 0.0);
        
        float shadow = 1.0;
        vec3 ro = worldPos + normal * 0.01;
        
        int meshCount = meshes.length();
        for (int m = 0; m < meshCount; m++) {
            
            // --- TELEPORTACJA PROMIENIA DO LOKALNEGO ŚWIATA OBIEKTU ---
            // Pozycja startowa (jako vec4 z 1.0 na końcu, żeby uwzględnić przesunięcie w macierzy)
            vec3 localRo = (meshes[m].invModelMatrix * vec4(ro, 1.0)).xyz;
            
            // Kierunek (używamy mat3, bo wektor kierunku ignoruje przesunięcie, tylko rotacja i skala)
            vec3 localDir = mat3(meshes[m].invModelMatrix) * dir;

            float tBox;
            // TERAZ WSZYSTKO DZIAŁA W LOCAL SPACE:
            // Promień jest lokalny, więc test AABB z lokalnymi wymiarami z pliku zadziała idealnie!
            if (RayAABB(localRo, localDir, meshes[m].aabbMin, meshes[m].aabbMax, tBox)) {
                if (tBox < len) {
                    for (uint idx = meshes[m].indexStart; idx < meshes[m].indexEnd; idx += 3) {
                        uint i0 = indices[idx] * STRIDE;
                        uint i1 = indices[idx+1] * STRIDE;
                        uint i2 = indices[idx+2] * STRIDE;
                        
                        // Wierzchołki z VBO też są na 0,0,0 (Local Space)
                        vec3 v0 = vec3(vboData[i0], vboData[i0+1], vboData[i0+2]);
                        vec3 v1 = vec3(vboData[i1], vboData[i1+1], vboData[i1+2]);
                        vec3 v2 = vec3(vboData[i2], vboData[i2+1], vboData[i2+2]);
                        
                        float tTri;
                        // Testujemy lokalny promień z lokalnym trójkątem. Zero mnożeń w pętli!
                        if (RayTriangle(localRo, localDir, v0, v1, v2, tTri)) {
                            if (tTri < len) {
                                shadow = 0.2;
                                break;
                            }
                        }
                    }
                }
            }
            if (shadow <= 0.5) break;
        }
        
        oute += falloff * lights[i].color * lights[i].power * ndotl * shadow;
    }
    
    return vec4(oute, 1.0);
}

void main() {
    ivec2 pixelCoords = ivec2(gl_GlobalInvocationID.xy);
    ivec2 size = imageSize(u_OutputTex);

    if (pixelCoords.x >= size.x || pixelCoords.y >= size.y) return;

    vec2 uv = (vec2(pixelCoords) + 0.5) / vec2(size);

    vec3 worldPos = texture(u_PositionTex, uv).xyz;
    vec3 normal = normalize(texture(u_Normal, uv).xyz);

    vec4 color = imageLoad(u_OutputTex, pixelCoords);

    color *= PointsColor(worldPos, normal);

    color.rgb = clamp(color.rgb, 0.0, 1.0);

    ivec2 lutuv = ivec2(int(color.b * 15.0) * 16 + int(color.r * 15.0), int(color.g * 15.0));

    color = texelFetch(u_LUT, lutuv, 0); 

    imageStore(u_OutputTex, pixelCoords, color);
}