#pragma once

#include "Units.h"

// Vector2

inline smq::Vector2 operator+(const smq::Vector2& a, const smq::Vector2& b) {
    return { a.x + b.x, a.y + b.y };
}

inline smq::Vector2 operator-(const smq::Vector2& a, const smq::Vector2& b) {
    return { a.x - b.x, a.y - b.y };
}

inline smq::Vector2 operator*(const smq::Vector2& a, const smq::Vector2& b) {
    return { a.x * b.x, a.y * b.y };
}

inline smq::Vector2 operator/(const smq::Vector2& a, const smq::Vector2& b) {
    return { a.x / b.x, a.y / b.y };
}

inline smq::Vector2 operator+(const smq::Vector2& a, const float& b) {
    return { a.x + b, a.y + b };
}

inline smq::Vector2 operator-(const smq::Vector2& a, const float& b) {
    return { a.x - b, a.y - b };
}

inline smq::Vector2 operator-(const float& a, const smq::Vector2& b) {
    return { a - b.x, a - b.y };
}

inline smq::Vector2 operator*(const smq::Vector2& a, const float& b) {
    return { a.x * b, a.y * b };
}

inline smq::Vector2 operator/(const smq::Vector2& a, const float& b) {
    return { a.x / b, a.y / b };
}

inline smq::Vector2 operator/(const float& a, const smq::Vector2& b) {
    return { a / b.x, a / b.y };
}

// Vector3

inline smq::Vector3 operator+(const smq::Vector3& a, const smq::Vector3& b) {
    return { a.x + b.x, a.y + b.y, a.z + b.z };
}

inline smq::Vector3 operator-(const smq::Vector3& a, const smq::Vector3& b) {
    return { a.x - b.x, a.y - b.y, a.z - b.z };
}

inline smq::Vector3 operator*(const smq::Vector3& a, const smq::Vector3& b) {
    return { a.x * b.x, a.y * b.y, a.z * b.z };
}

inline smq::Vector3 operator/(const smq::Vector3& a, const smq::Vector3& b) {
    return { a.x / b.x, a.y / b.y, a.z / b.z };
}

inline smq::Vector3 operator+(const smq::Vector3& a, float b) {
    return { a.x + b, a.y + b, a.z + b };
}

inline smq::Vector3 operator-(const smq::Vector3& a, float b) {
    return { a.x - b, a.y - b, a.z - b };
}

inline smq::Vector3 operator*(const smq::Vector3& a, float b) {
    return { a.x * b, a.y * b, a.z * b };
}

inline smq::Vector3 operator/(const smq::Vector3& a, float b) {
    return { a.x / b, a.y / b, a.z / b };
}

inline smq::Vector3 operator+(float a, const smq::Vector3& b) {
    return { a + b.x, a + b.y, a + b.z };
}

inline smq::Vector3 operator-(float a, const smq::Vector3& b) {
    return { a - b.x, a - b.y, a - b.z };
}

inline smq::Vector3 operator*(float a, const smq::Vector3& b) {
    return { a * b.x, a * b.y, a * b.z };
}

inline smq::Vector3 operator/(float a, const smq::Vector3& b) {
    return { a / b.x, a / b.y, a / b.z };
}

// Vector4

inline smq::Vector4 operator+(const smq::Vector4& a, const smq::Vector4& b) {
    return { a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w };
}

inline smq::Vector4 operator-(const smq::Vector4& a, const smq::Vector4& b) {
    return { a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w };
}

inline smq::Vector4 operator*(const smq::Vector4& a, const smq::Vector4& b) {
    return { a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w };
}

inline smq::Vector4 operator/(const smq::Vector4& a, const smq::Vector4& b) {
    return { a.x / b.x, a.y / b.y, a.z / b.z, a.w / b.w };
}

inline smq::Vector4 operator+(const smq::Vector4& a, float b) {
    return { a.x + b, a.y + b, a.z + b, a.w + b };
}

inline smq::Vector4 operator-(const smq::Vector4& a, float b) {
    return { a.x - b, a.y - b, a.z - b, a.w - b };
}

inline smq::Vector4 operator*(const smq::Vector4& a, float b) {
    return { a.x * b, a.y * b, a.z * b, a.w * b };
}

inline smq::Vector4 operator/(const smq::Vector4& a, float b) {
    return { a.x / b, a.y / b, a.z / b, a.w / b };
}

inline smq::Vector4 operator+(float a, const smq::Vector4& b) {
    return { a + b.x, a + b.y, a + b.z, a + b.w };
}

inline smq::Vector4 operator-(float a, const smq::Vector4& b) {
    return { a - b.x, a - b.y, a - b.z, a - b.w };
}

inline smq::Vector4 operator*(float a, const smq::Vector4& b) {
    return { a * b.x, a * b.y, a * b.z, a * b.w };
}

inline smq::Vector4 operator/(float a, const smq::Vector4& b) {
    return { a / b.x, a / b.y, a / b.z, a / b.w };
}