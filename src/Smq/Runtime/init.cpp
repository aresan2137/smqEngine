#include "../Include.h"

GLFWwindow* i_window = nullptr;
ImGuiIO* io = nullptr;

namespace smq {
    void init(Window* window) {
        if (!glfwInit()) Error("glfw init was not sucesfull");

        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
        glfwWindowHint(GLFW_DEPTH_BITS, 24);

        i_window = glfwCreateWindow(window->size.x, window->size.y, window->title.c_str(), nullptr, nullptr);

        if (!i_window) Error("window was not created correctly");

        glfwMakeContextCurrent(i_window);
        
        if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) Error("Glad initation was not sucsesfull");

        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);

        glEnable(GL_CULL_FACE);
        glCullFace(GL_BACK);
        glEnable(GL_DEPTH_TEST);
        glDepthFunc(GL_LESS);
        glEnable(GL_BLEND);
        //glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        //glDepthMask(GL_FALSE); 

        glfwSwapInterval(1);

        IMGUI_CHECKVERSION();
        ImGui::CreateContext();
        ImGui::StyleColorsDark();

        ImGui_ImplGlfw_InitForOpenGL(i_window, false);
        ImGui_ImplOpenGL3_Init("#version 330 core");

        InputInit();

        io = &ImGui::GetIO();

        InitSound();

        Log("init sucess");
    }

    void quit() {
        glfwTerminate();

        ShutdownSound();
    }
}