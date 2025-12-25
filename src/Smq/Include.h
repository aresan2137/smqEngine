#pragma once

#include "Smq.h"

#include <glad/glad.h>
#include <GLFW/glfw3.h>

#include "../smq/glmToMy.h"

extern GLFWwindow* window;
extern ImGuiIO* io;

void InputInit();
void InputUpdate();

void InitDrawing(smq::Scene& scene);
void DrawScene(smq::Scene& scene);
