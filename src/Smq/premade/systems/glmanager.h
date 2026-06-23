#pragma once

#include "../../units.h"
#include "../../runtime.h"

#include "../components/modelMaterial.h"
#include "../components/position3D.h"

class GlManager : public System {
public:

    unsigned int screenShader = 0;
    unsigned int dummyVAO = 0;
    int loc;

    unsigned int CreateShaderProgram(const char* vertexSource, const char* fragmentSource) {
        unsigned int vertexShader = glCreateShader(GL_VERTEX_SHADER);
        glShaderSource(vertexShader, 1, &vertexSource, NULL);
        glCompileShader(vertexShader);

        unsigned int fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(fragmentShader, 1, &fragmentSource, NULL);
        glCompileShader(fragmentShader);

        unsigned int shaderProgram = glCreateProgram();
        glAttachShader(shaderProgram, vertexShader);
        glAttachShader(shaderProgram, fragmentShader);
        glLinkProgram(shaderProgram);

        glDeleteShader(vertexShader);
        glDeleteShader(fragmentShader);
        return shaderProgram;
    }

    GlManager() {
        screenShader = CreateShaderProgram(R"(
#version 460 core
out vec2 TexCoords;

void main() {
    // Generuje automatycznie 4 wierzchołki prostokąta (fullscreen quad)
    float x = -1.0 + float((gl_VertexID & 1) << 2);
    float y = -1.0 + float((gl_VertexID & 2) << 1);
    TexCoords = vec2(x * 0.5 + 0.5, y * 0.5 + 0.5);
    gl_Position = vec4(x, y, 0.0, 1.0);
}
)", R"(
#version 460 core
out vec4 FragColor;
in vec2 TexCoords;

uniform sampler2D screenTexture;

void main() {
    FragColor = texture(screenTexture, TexCoords);
}
)");
        glGenVertexArrays(1, &dummyVAO);

        loc = glGetUniformLocation(screenShader, "screenTexture");
    }

    ~GlManager() {
        glDeleteProgram(screenShader);
        glDeleteVertexArrays(1, &dummyVAO);
    }

    void EarlyUpdate(Scene* scene) override {
        ImGui_ImplOpenGL3_NewFrame();
        ImGui_ImplGlfw_NewFrame();
        ImGui::NewFrame();

        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    }

    void LateUpdate(Scene* scene) override {
        Vector2Int winSize = res.get<Window>()->GetSize();
        glViewport(0, 0, winSize.x, winSize.y);

        glUseProgram(screenShader);

        glDisable(GL_DEPTH_TEST);

        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, scene->output->getTexture());
        glUniform1i(loc, 0);

        glBindVertexArray(dummyVAO);
        glDrawArrays(GL_TRIANGLES, 0, 3);
        glBindVertexArray(0);

        glBindTexture(GL_TEXTURE_2D, 0);

        glEnable(GL_DEPTH_TEST);

        ImGui::Render();
        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());

        glfwSwapBuffers(res.get<Window>()->window);
    }

};