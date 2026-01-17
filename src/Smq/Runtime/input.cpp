#include "../Include.h"

#ifdef _WIN32
#include <windows.h>

void HideCursor() {
    while (ShowCursor(FALSE) >= 0);
}

void ShowCursor() {
    while (ShowCursor(TRUE) < 0);
}
#else 
#warning "Cursor visibility not implemented for this platform"
void HideCursor() {
}
void ShowCursor() {
}
#endif

#define KEYS 348

struct InputState {
    bool keys[KEYS] = {};
    bool keysPrevFrame[KEYS] = {};
    smq::Vector2 mousepos;
} Input;

void KeyCallback(GLFWwindow*, int key, int, int action, int) {
    if (key < 0 || key >= 348) return;

    if (action == GLFW_PRESS)   Input.keys[key] = true;
    if (action == GLFW_RELEASE) Input.keys[key] = false;
}

void MouseCallback(GLFWwindow*, int key, int action, int) {
    if (action == GLFW_PRESS)   Input.keys[key+1] = true;
    if (action == GLFW_RELEASE) Input.keys[key+1] = false;
}

void MouseMovmentCallback(GLFWwindow*, double xpos, double ypos) {
    Input.mousepos.x = (float)xpos;
    Input.mousepos.y = (float)ypos;
}

void UpdateImGuiInput() {
    io->MousePos = ImVec2(Input.mousepos.x, Input.mousepos.y);
    io->MouseDown[0] = glfwGetMouseButton(i_window, GLFW_MOUSE_BUTTON_LEFT) == GLFW_PRESS;
}

void InputInit() {
    glfwSetKeyCallback(i_window, KeyCallback);
    glfwSetMouseButtonCallback(i_window, MouseCallback);
    glfwSetCursorPosCallback(i_window, MouseMovmentCallback);
}

void InputUpdate() {
    UpdateImGuiInput();
    for (int i = 0; i < KEYS; i++) {
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

    void SetCursor(bool visible) {
        if (visible) ShowCursor();
		else         HideCursor();
    }    
}