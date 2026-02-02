#include "../Include.h"
#include "imgui.h" 
#include "imgui_impl_glfw.h"

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

GLFWwindow* i_window;
ImGuiContext* ctxe;

#define KEYS 348

struct InputState {
    bool keys[KEYS] = {};
    bool keysPrevFrame[KEYS] = {};
    smq::Vector2 mousepos;
} Input;


// --- SCROLL CALLBACK ---
void ScrollCallback(GLFWwindow* window, double xoffset, double yoffset) {
    if (ctxe != nullptr) ImGui::SetCurrentContext(ctxe);

    if (ImGui::GetCurrentContext() != nullptr) {
        ImGuiIO& io = ImGui::GetIO();
        io.AddMouseWheelEvent((float)xoffset, (float)yoffset);
    }

    // Opcjonalnie: ImGui_ImplGlfw_ScrollCallback(window, xoffset, yoffset);
}

// --- MOUSE MOVEMENT CALLBACK ---
void MouseMovmentCallback(GLFWwindow* window, double xpos, double ypos) {
    if (ctxe != nullptr) ImGui::SetCurrentContext(ctxe);

    if (ImGui::GetCurrentContext() != nullptr) {
        ImGuiIO& io = ImGui::GetIO();
        io.AddMousePosEvent((float)xpos, (float)ypos);
    }

    Input.mousepos.x = (float)xpos;
    Input.mousepos.y = (float)ypos;
}

// --- MOUSE BUTTON CALLBACK ---
void MouseCallback(GLFWwindow* window, int button, int action, int mods) {
    if (ctxe != nullptr) ImGui::SetCurrentContext(ctxe);

    if (ImGui::GetCurrentContext() != nullptr) {
        ImGuiIO& io = ImGui::GetIO();

        int imgui_button = -1;
        if (button == GLFW_MOUSE_BUTTON_LEFT) imgui_button = 0;
        else if (button == GLFW_MOUSE_BUTTON_RIGHT) imgui_button = 1;
        else if (button == GLFW_MOUSE_BUTTON_MIDDLE) imgui_button = 2;

        if (imgui_button != -1) {
            io.AddMouseButtonEvent(imgui_button, action == GLFW_PRESS);
        }
    }

    if (action == GLFW_PRESS)   Input.keys[button + 1] = true;
    if (action == GLFW_RELEASE) Input.keys[button + 1] = false;
}

// --- CHAR CALLBACK ---
void CharCallback(GLFWwindow* window, unsigned int codepoint) {
    if (ctxe != nullptr) ImGui::SetCurrentContext(ctxe);

    if (ImGui::GetCurrentContext() != nullptr) {
        ImGuiIO& io = ImGui::GetIO();
        io.AddInputCharacter(codepoint);
    }
}

// --- KEY CALLBACK (PEŁNY) ---
void KeyCallback(GLFWwindow* window, int key, int scancode, int action, int mods) {
    if (ctxe != nullptr) ImGui::SetCurrentContext(ctxe);

    if (ImGui::GetCurrentContext() != nullptr) {
        ImGuiIO& io = ImGui::GetIO();
        ImGuiKey imgui_key = ImGuiKey_None;

        switch (key) {
            // -- Modyfikatory (To naprawia Twój błąd!) --
        case GLFW_KEY_LEFT_CONTROL:  imgui_key = ImGuiKey_LeftCtrl; break;
        case GLFW_KEY_RIGHT_CONTROL: imgui_key = ImGuiKey_RightCtrl; break;
        case GLFW_KEY_LEFT_SHIFT:    imgui_key = ImGuiKey_LeftShift; break;
        case GLFW_KEY_RIGHT_SHIFT:   imgui_key = ImGuiKey_RightShift; break;
        case GLFW_KEY_LEFT_ALT:      imgui_key = ImGuiKey_LeftAlt; break;
        case GLFW_KEY_RIGHT_ALT:     imgui_key = ImGuiKey_RightAlt; break;
        case GLFW_KEY_LEFT_SUPER:    imgui_key = ImGuiKey_LeftSuper; break;
        case GLFW_KEY_RIGHT_SUPER:   imgui_key = ImGuiKey_RightSuper; break;

            // -- Nawigacja i Edycja --
        case GLFW_KEY_TAB:       imgui_key = ImGuiKey_Tab; break;
        case GLFW_KEY_LEFT:      imgui_key = ImGuiKey_LeftArrow; break;
        case GLFW_KEY_RIGHT:     imgui_key = ImGuiKey_RightArrow; break;
        case GLFW_KEY_UP:        imgui_key = ImGuiKey_UpArrow; break;
        case GLFW_KEY_DOWN:      imgui_key = ImGuiKey_DownArrow; break;
        case GLFW_KEY_PAGE_UP:   imgui_key = ImGuiKey_PageUp; break;
        case GLFW_KEY_PAGE_DOWN: imgui_key = ImGuiKey_PageDown; break;
        case GLFW_KEY_HOME:      imgui_key = ImGuiKey_Home; break;
        case GLFW_KEY_END:       imgui_key = ImGuiKey_End; break;
        case GLFW_KEY_INSERT:    imgui_key = ImGuiKey_Insert; break;
        case GLFW_KEY_DELETE:    imgui_key = ImGuiKey_Delete; break;
        case GLFW_KEY_BACKSPACE: imgui_key = ImGuiKey_Backspace; break;
        case GLFW_KEY_SPACE:     imgui_key = ImGuiKey_Space; break;
        case GLFW_KEY_ENTER:     imgui_key = ImGuiKey_Enter; break;
        case GLFW_KEY_ESCAPE:    imgui_key = ImGuiKey_Escape; break;
        case GLFW_KEY_KP_ENTER:  imgui_key = ImGuiKey_KeypadEnter; break;

            // -- Litery A-Z --
        case GLFW_KEY_A: imgui_key = ImGuiKey_A; break;
        case GLFW_KEY_B: imgui_key = ImGuiKey_B; break;
        case GLFW_KEY_C: imgui_key = ImGuiKey_C; break;
        case GLFW_KEY_D: imgui_key = ImGuiKey_D; break;
        case GLFW_KEY_E: imgui_key = ImGuiKey_E; break;
        case GLFW_KEY_F: imgui_key = ImGuiKey_F; break;
        case GLFW_KEY_G: imgui_key = ImGuiKey_G; break;
        case GLFW_KEY_H: imgui_key = ImGuiKey_H; break;
        case GLFW_KEY_I: imgui_key = ImGuiKey_I; break;
        case GLFW_KEY_J: imgui_key = ImGuiKey_J; break;
        case GLFW_KEY_K: imgui_key = ImGuiKey_K; break;
        case GLFW_KEY_L: imgui_key = ImGuiKey_L; break;
        case GLFW_KEY_M: imgui_key = ImGuiKey_M; break;
        case GLFW_KEY_N: imgui_key = ImGuiKey_N; break;
        case GLFW_KEY_O: imgui_key = ImGuiKey_O; break;
        case GLFW_KEY_P: imgui_key = ImGuiKey_P; break;
        case GLFW_KEY_Q: imgui_key = ImGuiKey_Q; break;
        case GLFW_KEY_R: imgui_key = ImGuiKey_R; break;
        case GLFW_KEY_S: imgui_key = ImGuiKey_S; break;
        case GLFW_KEY_T: imgui_key = ImGuiKey_T; break;
        case GLFW_KEY_U: imgui_key = ImGuiKey_U; break;
        case GLFW_KEY_V: imgui_key = ImGuiKey_V; break;
        case GLFW_KEY_W: imgui_key = ImGuiKey_W; break;
        case GLFW_KEY_X: imgui_key = ImGuiKey_X; break;
        case GLFW_KEY_Y: imgui_key = ImGuiKey_Y; break;
        case GLFW_KEY_Z: imgui_key = ImGuiKey_Z; break;

            // -- Cyfry --
        case GLFW_KEY_0: imgui_key = ImGuiKey_0; break;
        case GLFW_KEY_1: imgui_key = ImGuiKey_1; break;
        case GLFW_KEY_2: imgui_key = ImGuiKey_2; break;
        case GLFW_KEY_3: imgui_key = ImGuiKey_3; break;
        case GLFW_KEY_4: imgui_key = ImGuiKey_4; break;
        case GLFW_KEY_5: imgui_key = ImGuiKey_5; break;
        case GLFW_KEY_6: imgui_key = ImGuiKey_6; break;
        case GLFW_KEY_7: imgui_key = ImGuiKey_7; break;
        case GLFW_KEY_8: imgui_key = ImGuiKey_8; break;
        case GLFW_KEY_9: imgui_key = ImGuiKey_9; break;

            // -- Klawisze F --
        case GLFW_KEY_F1:  imgui_key = ImGuiKey_F1; break;
        case GLFW_KEY_F2:  imgui_key = ImGuiKey_F2; break;
        case GLFW_KEY_F3:  imgui_key = ImGuiKey_F3; break;
        case GLFW_KEY_F4:  imgui_key = ImGuiKey_F4; break;
        case GLFW_KEY_F5:  imgui_key = ImGuiKey_F5; break;
        case GLFW_KEY_F6:  imgui_key = ImGuiKey_F6; break;
        case GLFW_KEY_F7:  imgui_key = ImGuiKey_F7; break;
        case GLFW_KEY_F8:  imgui_key = ImGuiKey_F8; break;
        case GLFW_KEY_F9:  imgui_key = ImGuiKey_F9; break;
        case GLFW_KEY_F10: imgui_key = ImGuiKey_F10; break;
        case GLFW_KEY_F11: imgui_key = ImGuiKey_F11; break;
        case GLFW_KEY_F12: imgui_key = ImGuiKey_F12; break;
        }

        if (imgui_key != ImGuiKey_None) {
            // To mówi ImGui, że klawisz został wciśnięty/puszczony
            io.AddKeyEvent(imgui_key, (action == GLFW_PRESS || action == GLFW_REPEAT));
        }
    }

    // Twoja logika gry
    if (key < 0 || key >= 348) return;
    if (action == GLFW_PRESS)   Input.keys[key] = true;
    if (action == GLFW_RELEASE) Input.keys[key] = false;
}

void UpdateImGuiInput(GLFWwindow* window, ImGuiIO* io) {

    io->MousePos = ImVec2(Input.mousepos.x, Input.mousepos.y);
    io->MouseDown[0] = glfwGetMouseButton(window, GLFW_MOUSE_BUTTON_LEFT) == GLFW_PRESS;
}

void InputInit(GLFWwindow* window, ImGuiContext* ctx) {
    i_window = window;
    ctxe = ctx;

    ImGui::SetCurrentContext(ctxe);
    

    glfwSetKeyCallback(window, KeyCallback);
    glfwSetCharCallback(window, CharCallback);
    glfwSetMouseButtonCallback(window, MouseCallback);
    glfwSetCursorPosCallback(window, MouseMovmentCallback);
    glfwSetScrollCallback(window, ScrollCallback);
}

void InputUpdate(ImGuiIO* io) {
    UpdateImGuiInput(i_window, io);
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

    void SetCursor(bool visible, bool LockMouse) {
        if (visible) ShowCursor();
        else         HideCursor();

        if (LockMouse) glfwSetCursorPos(i_window, 1280 / 2, 720 / 2);
    }
}