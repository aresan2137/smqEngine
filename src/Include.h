#pragma once

#include "Smq.h"

#include <glad/glad.h>
#include <GLFW/glfw3.h>

void InputInit(GLFWwindow* window, ImGuiContext* ctx);
void InputUpdate(ImGuiIO* io);

void InitSound();
void ShutdownSound();