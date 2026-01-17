#pragma once

#include "Units.h"

#define GLM_ENABLE_EXPERIMENTAL
#include "glm.hpp"
#include "gtc/quaternion.hpp"
#include "gtc/matrix_transform.hpp"
#include "gtc/type_ptr.hpp"
#include "gtx/quaternion.hpp"

#include <cstring>

namespace gtm {
    inline smq::Matrix4 Gmat4(const glm::mat4& g) {
        smq::Matrix4 r;
        std::memcpy(r.m, glm::value_ptr(g), sizeof(float) * 16);
        return r;
    }

    inline glm::vec3 Mvec3(const smq::Vector3& v) {
        return glm::vec3(v.x, v.y, v.z);
    }

    inline smq::Vector3 Gvec3(const glm::vec3& v) {
        return { v.x, v.y, v.z };
    }
}