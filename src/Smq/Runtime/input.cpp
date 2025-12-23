#include "../Include.h"

struct InputState {
    bool keys[348]{};
    bool keysPrevFrame[348]{};
    smq::Vector2 mousepos;
} Input;

void KeyCallback(GLFWwindow*, int key, int, int action, int) {
    if (key < 0 || key >= 348) return;

    if (action == GLFW_PRESS)   Input.keys[key] = true;
    if (action == GLFW_RELEASE) Input.keys[key] = false;
}

void MouseCallback(GLFWwindow*, double xpos, double ypos) {
    Input.mousepos.x = (float)xpos;
    Input.mousepos.y = (float)ypos;
}

void UpdateImGuiInput() {
    io->MousePos = ImVec2(Input.mousepos.x, Input.mousepos.y);
    io->MouseDown[0] = glfwGetMouseButton(window, GLFW_MOUSE_BUTTON_LEFT) == GLFW_PRESS;
}

void InputInit() {
    glfwSetKeyCallback(window, KeyCallback);
    glfwSetCursorPosCallback(window, MouseCallback);
}

void InputUpdate() {
    UpdateImGuiInput();
    for (int i = 0; i < 348; i++) {
        Input.keysPrevFrame[i] = Input.keys[i];
    }
}

namespace smq {
    
    bool IsKeyDown(Key key) {
        if (key == Key_Null) return false;
        return Input.keys[static_cast<unsigned int>(key)];
    }

    bool IsKeyPresed(Key key) {
        if (key == Key_Null) return false;
        unsigned int k = static_cast<unsigned int>(key);
        return Input.keys[k] && !Input.keysPrevFrame[k];
    }

    Vector2 GetMousePositon() {
        return Input.mousepos;
    }
}