#pragma once

#include <vector>
#include <map>
#include <unordered_map>
#include <string>
#include <fstream>
#include <sstream>
#include <variant>
#include <typeindex>
#include <utility>
#include <thread>
#include <mutex>
#include <future>
#include <algorithm>
#include <queue>
#include <cmath>

#include "glad/glad.h"
#include "glfw/glfw3.h"

#include "glm/glm.hpp"
#include "glm/gtc/matrix_transform.hpp"
#include "glm/gtc/quaternion.hpp"

#include "imgui.h"
#include <imgui_impl_glfw.h>
#include <imgui_impl_opengl3.h>

#include "FastNoise/FastNoise.h"

#include "../log.h"

using Vector2 = glm::vec2;
using Vector3 = glm::vec3;
using Vector4 = glm::vec4;

using Vector2Int = glm::ivec2;
using Vector3Int = glm::ivec3;
using Vector4Int = glm::ivec4;

using Vector2Duble = glm::dvec2;
using Vector3Duble = glm::dvec3;
using Vector4Duble = glm::dvec4;

using Matrix3 = glm::mat3;
using Matrix4 = glm::mat4;

using Quaternion = glm::quat;

struct Color {
	std::uint8_t r, g, b, a;
	Color(std::uint8_t r, std::uint8_t g, std::uint8_t b, std::uint8_t a) : r(r), g(g), b(b), a(a) {}
};

using Shader = uint32_t;