#pragma once

#include "units.h"

class Window {
public:
    GLFWwindow* window = nullptr;

    Window(Vector2Int size, std::string title = "game") {
        if (!glfwInit()) error("GLFW init failed");

        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 6);
        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
        glfwWindowHint(GLFW_DEPTH_BITS, 24);

        window = glfwCreateWindow(size.x, size.y, title.c_str(), nullptr, nullptr);

        if (!window) error("GLFW window creation failed");

        glfwMakeContextCurrent(window);

        if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) error("GLAD init error");

        glClearColor(0.01f, 0.01f, 0.01f, 1.0f);

        glEnable(GL_CULL_FACE);
        glCullFace(GL_BACK);
        glEnable(GL_DEPTH_TEST);
        glDepthFunc(GL_LESS);
        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

        glfwSwapInterval(1);

        IMGUI_CHECKVERSION();
        ImGuiContext* contex = ImGui::CreateContext();
        ImGui::StyleColorsDark();

        ImGui_ImplGlfw_InitForOpenGL(window, true);

        ImGui_ImplOpenGL3_Init("#version 460 core");

        log("Window creation sucess");
    }

    ~Window() {
        ImGui_ImplOpenGL3_Shutdown();
        ImGui_ImplGlfw_Shutdown();
        ImGui::DestroyContext();

        if (window) glfwDestroyWindow(window);
        glfwTerminate();

        log("Window deletion sucess");
    }

    void SetSize(Vector2Int size) {
        glfwSetWindowSize(window, size.x, size.y);
    }

    Vector2Int GetSize() {
        int width, height;
        glfwGetWindowSize(window, &width, &height);
        return { width, height };
	}

    Window(const Window&) = delete;
    Window& operator=(const Window&) = delete;
};
