#include "../Include.h"

GLFWwindow* window = nullptr;
ImGuiIO* io = nullptr;

namespace smq {
    void init(Vector2 size, std::string WindowName) {
        if (!glfwInit()) Error("glfw init was not sucesfull");

        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
        glfwWindowHint(GLFW_DEPTH_BITS, 24);

        window = glfwCreateWindow(size.x, size.y, WindowName.c_str(), nullptr, nullptr);

        if (!window) Error("window was not created correctly");

        glfwMakeContextCurrent(window);

        if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) Error("Glad initation was not sucsesfull");

        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);

        glEnable(GL_CULL_FACE);
        glCullFace(GL_BACK);
        glEnable(GL_DEPTH_TEST);
        glDepthFunc(GL_LESS);

        IMGUI_CHECKVERSION();
        ImGui::CreateContext();
        ImGui::StyleColorsDark();

        ImGui_ImplGlfw_InitForOpenGL(window, false);
        ImGui_ImplOpenGL3_Init("#version 330 core");

        InputInit();

        io = &ImGui::GetIO();

        Log("init sucess");
    }

    void quit() {
        glfwTerminate();
    }
}