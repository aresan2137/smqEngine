#include "../Include.h"

namespace smq {
    Engine::Engine(Window window)
        : i_window(window)
    {
        if (!glfwInit()) Error("glfw init was not sucesfull");

        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 6);
        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
        glfwWindowHint(GLFW_DEPTH_BITS, 24);

        i_GlfwWindow = glfwCreateWindow(i_window.size.x, i_window.size.y, i_window.title.c_str(), nullptr, nullptr);

        if (!i_GlfwWindow) Error("window was not created correctly");

        glfwMakeContextCurrent(i_GlfwWindow);
        
        if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) Error("Glad initation was not sucsesfull");

        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);

        glEnable(GL_CULL_FACE);
        glCullFace(GL_BACK);
        glEnable(GL_DEPTH_TEST);
        glDepthFunc(GL_LESS);
        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

        plane = new smq::Mesh("resources/plane.smf");

        glfwSwapInterval(1);

        IMGUI_CHECKVERSION();
        i_ImGuiContext = ImGui::CreateContext();
        ImGui::StyleColorsDark();

        InputInit(i_GlfwWindow, i_ImGuiContext);

        ImGui_ImplGlfw_InitForOpenGL(i_GlfwWindow, false);

        ImGui_ImplOpenGL3_Init("#version 460 core");

        i_ImIo = &ImGui::GetIO();

        InitSound();        

        Log("init sucess");
    }

    Engine::~Engine() {
        glfwTerminate();

        ShutdownSound();

        delete currentScene->rootObject;

        Log("quit sucess");
    }

    ImGuiContext* Engine::GetImGuiContext() {
        return i_ImGuiContext; 
    }

}