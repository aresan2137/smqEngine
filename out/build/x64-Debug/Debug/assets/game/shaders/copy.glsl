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

bool IntersectAABB(vec3 ro, vec3 rd, vec3 bmin, vec3 bmax, float maxDist)
{
    vec3 invDir = 1.0 / rd;

    vec3 t0 = (bmin - ro) * invDir;
    vec3 t1 = (bmax - ro) * invDir;

    vec3 tminV = min(t0, t1);
    vec3 tmaxV = max(t0, t1);

    float tmin = max(max(tminV.x, tminV.y), tminV.z);
    float tmax = min(min(tmaxV.x, tmaxV.y), tmaxV.z);

    return tmax >= max(0.0, tmin) && tmin < maxDist;
}

bool IntersectRayTriangle(vec3 ro, vec3 rd, vec3 v0, vec3 v1, vec3 v2, float maxDist)
{
    vec3 e1 = v1 - v0;
    vec3 e2 = v2 - v0;

    vec3 h = cross(rd, e2);
    float a = dot(e1, h);

    if (abs(a) < 0.00001)
        return false;

    float f = 1.0 / a;

    vec3 s = ro - v0;
    float u = f * dot(s, h);

    if (u < 0.0 || u > 1.0)
        return false;

    vec3 q = cross(s, e1);
    float v = f * dot(rd, q);

    if (v < 0.0 || u + v > 1.0)
        return false;

    float t = f * dot(e2, q);

    return (t > 0.001 && t < maxDist);
}

bool ShadowRay(vec3 ro, vec3 rd, float maxDist)
{
    uint meshCount = meshes.length();

    for (uint m = 0; m < meshCount; m++)
    {
        MeshInfo mesh = meshes[m];

        if (!IntersectAABB(ro, rd, mesh.aabbMin, mesh.aabbMax, maxDist))
            continue;

        for (uint i = mesh.indexStart; i < mesh.indexEnd; i += 3)
        {
            uint i0 = indices[i];
            uint i1 = indices[i + 1];
            uint i2 = indices[i + 2];

            vec3 v0 = vec3(vboData[i0 * STRIDE + 0], vboData[i0 * STRIDE + 1], vboData[i0 * STRIDE + 2]);
            vec3 v1 = vec3(vboData[i1 * STRIDE + 0], vboData[i1 * STRIDE + 1], vboData[i1 * STRIDE + 2]);
            vec3 v2 = vec3(vboData[i2 * STRIDE + 0], vboData[i2 * STRIDE + 1], vboData[i2 * STRIDE + 2]);

            if (IntersectRayTriangle(ro, rd, v0, v1, v2, maxDist))
                return true;
        }
    }

    return false;
}

vec3 ComputeLighting(vec3 worldPos, vec3 normal)
{
    vec3 result = vec3(0.0);

    uint lightCount = lights.length();

    for (uint i = 0; i < lightCount; i++)
    {
        Light L = lights[i];

        if (worldPos.x < L.aabbMin.x || worldPos.y < L.aabbMin.y || worldPos.z < L.aabbMin.z ||
            worldPos.x > L.aabbMax.x || worldPos.y > L.aabbMax.y || worldPos.z > L.aabbMax.z)
            continue;

        vec3 toL = L.pos - worldPos;
        float dist = length(toL);

        //if (dist > L.maxReach) continue;

        vec3 Ldir = toL / dist;

        float shadow = ShadowRay(worldPos, Ldir, dist) ? 0.0 : 1.0;

        if (shadow > 0.5) {
            return vec3(1,0,0);
        }

        float NdotL = max(dot(normal, Ldir), 0.0);

        float att = 1.0 - dist / L.maxReach;
        att *= sqrt(att);
        att *= 0.1;

        result += L.color * L.power * NdotL * att * shadow;
    }

    return result;
}

void main()
{
    ivec2 pixelCoords = ivec2(gl_GlobalInvocationID.xy);
    ivec2 size = imageSize(u_OutputTex);

    if (pixelCoords.x >= size.x || pixelCoords.y >= size.y) return;

    vec2 uv = (vec2(pixelCoords) + 0.5) / vec2(size);

    vec3 worldPos = texture(u_PositionTex, uv).xyz;
    vec3 normal = normalize(texture(u_Normal, uv).xyz);

    vec3 lighting = ComputeLighting(worldPos, normal);

    vec4 color = imageLoad(u_OutputTex, pixelCoords);

    color.rgb *= max(lighting, vec3(0.05));

    color.rgb = lighting;

    ivec2 lutuv = ivec2(int(color.b * 15.0) * 16 + int(color.r * 15.0), int(color.g * 15.0));

    //color = texelFetch(u_LUT, lutuv, 0);   

    imageStore(u_OutputTex, pixelCoords, color);
}