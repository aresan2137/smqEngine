#pragma once

#include "Units.h"

#include <cmath>
#include <algorithm>

namespace smq {
    inline unsigned int RandomHash(Vector2Int vec, int seed) {
        unsigned int mangled = seed;
        mangled *= 0xB5297A4D;
        mangled += vec.x;
        mangled *= 0x68E31DA4;
        mangled += vec.y;
        mangled *= 0x1B56C4E9;
        mangled ^= (mangled >> 8);
        return mangled;
    }

    inline float Random01(Vector2Int vec, int seed) {
        return (float)(RandomHash(vec, seed) & 0x7FFFFFFF) / 2147483647.0f;
    }

    inline void GetGradient(smq::Vector2Int vec, int seed, float& outX, float& outY) {
        unsigned int h = RandomHash(vec, seed);
        float angle = ((h & 0xFFFF) / 65536.0f) * 6.2831853f;
        outX = std::cos(angle);
        outY = std::sin(angle);
    }

    inline float Smoothstep(float t) {
        return t * t * (3.0f - 2.0f * t);
    }

    inline float Lerp(float a, float b, float t) {
        return a + t * (b - a);
    }

    inline float ValueNoise(Vector2 vec, int seed) {
        int ix = (int)std::floor(vec.x);
        int iy = (int)std::floor(vec.y);
        float fx = vec.x - ix;
        float fy = vec.y - iy;

        float n00 = Random01({ ix, iy }, seed);
        float n10 = Random01({ix + 1, iy}, seed);
        float n01 = Random01({ix, iy + 1}, seed);
        float n11 = Random01({ix + 1, iy + 1}, seed);

        float u = Smoothstep(fx);
        float v = Smoothstep(fy);

        float nx0 = Lerp(n00, n10, u);
        float nx1 = Lerp(n01, n11, u);

        return Lerp(nx0, nx1, v) * 2.0f - 1.0f;
    }

    inline float GradientNoise(Vector2 vec, int seed) {
        int ix = (int)std::floor(vec.x);
        int iy = (int)std::floor(vec.y);
        float fx = vec.x - ix;
        float fy = vec.y - iy;

        float gx, gy;

        GetGradient({ ix, iy }, seed, gx, gy);
        float n00 = gx * fx + gy * fy;

        GetGradient({ ix + 1, iy }, seed, gx, gy);
        float n10 = gx * (fx - 1.0f) + gy * fy;

        GetGradient({ ix, iy + 1 }, seed, gx, gy);
        float n01 = gx * fx + gy * (fy - 1.0f);

        GetGradient({ ix + 1, iy + 1 }, seed, gx, gy);
        float n11 = gx * (fx - 1.0f) + gy * (fy - 1.0f);

        float u = Smoothstep(fx);
        float v = Smoothstep(fy);

        float res = Lerp(Lerp(n00, n10, u), Lerp(n01, n11, u), v);

        return res * 1.414f;
    }

    inline float WorleyNoise(Vector2 vec, int seed) {
        int ix = (int)std::floor(vec.x);
        int iy = (int)std::floor(vec.y);
        float fx = vec.x - ix;
        float fy = vec.y - iy;

        float minDist = 1.0f;

        for (int yOff = -1; yOff <= 1; yOff++) {
            for (int xOff = -1; xOff <= 1; xOff++) {
                float rX = Random01({ix + xOff, iy + yOff}, seed);
                float rY = Random01({ix + xOff, iy + yOff}, seed + 2137);
                float vecX = xOff + rX - fx;
                float vecY = yOff + rY - fy;

                float dist = std::sqrt(vecX * vecX + vecY * vecY);
                if (dist < minDist) minDist = dist;
            }
        }
        return minDist;
    }

    inline float Fractal(Vector2 vec, int seed, int octaves, float persistence = 0.5f, float lacunarity = 2.0f, int type = 0) {
        float total = 0.0f;
        float freq = 1.0f;
        float amp = 1.0f;
        float maxVal = 0.0f;

        for (int i = 0; i < octaves; i++) {
            int octaveSeed = seed + i * 123;

            float val = 0.0f;
            if (type == 0) val = GradientNoise({ vec.x * freq, vec.y * freq }, octaveSeed);
            else           val = ValueNoise({ vec.x * freq, vec.y * freq }, octaveSeed);

            total += val * amp;
            maxVal += amp;

            amp *= persistence;
            freq *= lacunarity;
        }

        return total / maxVal;
    }
}