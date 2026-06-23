#pragma once
#include "units.h"

class Resources {
private:
    std::unordered_map<std::type_index, void*> _resources;

public:
    template<typename T>
    void insert(T* resource) {
        static_assert(std::is_class_v<T>, "Resource must be a struct or class");
        auto key = std::type_index(typeid(T));
        if (_resources.count(key)) { error("Resource already exists: " + std::string(typeid(T).name())); return; }
        _resources[key] = resource;
    }

    template<typename T>
    T* get() {
        auto it = _resources.find(std::type_index(typeid(T)));
        if (it == _resources.end()) { warn("Resource not found: " + std::string(typeid(T).name())); return nullptr; }
        return static_cast<T*>(it->second);
    }

    template<typename T>
    bool has() {
        return _resources.count(std::type_index(typeid(T))) > 0;
    }

    template<typename T>
    void remove() {
        auto it = _resources.find(std::type_index(typeid(T)));
        if (it == _resources.end()) { warn("Resource not found: " + std::string(typeid(T).name())); return; }
        _resources.erase(it);
    }
};

inline Resources res;

enum class Key : int {
    Escape = GLFW_KEY_ESCAPE,
    Num1 = GLFW_KEY_1,
    Num2 = GLFW_KEY_2,
    Num3 = GLFW_KEY_3,
    Num4 = GLFW_KEY_4,
    Num5 = GLFW_KEY_5,
    Num6 = GLFW_KEY_6,
    Num7 = GLFW_KEY_7,
    Num8 = GLFW_KEY_8,
    Num9 = GLFW_KEY_9,
    Num0 = GLFW_KEY_0,
    Minus = GLFW_KEY_MINUS,
    Equal = GLFW_KEY_EQUAL,
    Backspace = GLFW_KEY_BACKSPACE,

    Tab = GLFW_KEY_TAB,
    Q = GLFW_KEY_Q,
    W = GLFW_KEY_W,
    E = GLFW_KEY_E,
    R = GLFW_KEY_R,
    T = GLFW_KEY_T,
    Y = GLFW_KEY_Y,
    U = GLFW_KEY_U,
    I = GLFW_KEY_I,
    O = GLFW_KEY_O,
    P = GLFW_KEY_P,
    LeftBracket = GLFW_KEY_LEFT_BRACKET,
    RightBracket = GLFW_KEY_RIGHT_BRACKET,
    Enter = GLFW_KEY_ENTER,

    CapsLock = GLFW_KEY_CAPS_LOCK,
    A = GLFW_KEY_A,
    S = GLFW_KEY_S,
    D = GLFW_KEY_D,
    F = GLFW_KEY_F,
    G = GLFW_KEY_G,
    H = GLFW_KEY_H,
    J = GLFW_KEY_J,
    K = GLFW_KEY_K,
    L = GLFW_KEY_L,
    Semicolon = GLFW_KEY_SEMICOLON,
    Apostrophe = GLFW_KEY_APOSTROPHE,
    Backslash = GLFW_KEY_BACKSLASH,

    LeftShift = GLFW_KEY_LEFT_SHIFT,
    Z = GLFW_KEY_Z,
    X = GLFW_KEY_X,
    C = GLFW_KEY_C,
    V = GLFW_KEY_V,
    B = GLFW_KEY_B,
    N = GLFW_KEY_N,
    M = GLFW_KEY_M,
    Comma = GLFW_KEY_COMMA,
    Period = GLFW_KEY_PERIOD,
    Slash = GLFW_KEY_SLASH,
    RightShift = GLFW_KEY_RIGHT_SHIFT,

    LeftCtrl = GLFW_KEY_LEFT_CONTROL,
    LeftAlt = GLFW_KEY_LEFT_ALT,
    Space = GLFW_KEY_SPACE,
    RightAlt = GLFW_KEY_RIGHT_ALT,
    RightCtrl = GLFW_KEY_RIGHT_CONTROL,

    Up = GLFW_KEY_UP,
    Down = GLFW_KEY_DOWN,
    Left = GLFW_KEY_LEFT,
    Right = GLFW_KEY_RIGHT,

    Insert = GLFW_KEY_INSERT,
    Delete = GLFW_KEY_DELETE,
    Home = GLFW_KEY_HOME,
    End = GLFW_KEY_END,
    PageUp = GLFW_KEY_PAGE_UP,
    PageDown = GLFW_KEY_PAGE_DOWN,
    PrintScreen = GLFW_KEY_PRINT_SCREEN,
    ScrollLock = GLFW_KEY_SCROLL_LOCK,
    Pause = GLFW_KEY_PAUSE,

    F1 = GLFW_KEY_F1,
    F2 = GLFW_KEY_F2,
    F3 = GLFW_KEY_F3,
    F4 = GLFW_KEY_F4,
    F5 = GLFW_KEY_F5,
    F6 = GLFW_KEY_F6,
    F7 = GLFW_KEY_F7,
    F8 = GLFW_KEY_F8,
    F9 = GLFW_KEY_F9,
    F10 = GLFW_KEY_F10,
    F11 = GLFW_KEY_F11,
    F12 = GLFW_KEY_F12,

    NumLock = GLFW_KEY_NUM_LOCK,
    Numpad0 = GLFW_KEY_KP_0,
    Numpad1 = GLFW_KEY_KP_1,
    Numpad2 = GLFW_KEY_KP_2,
    Numpad3 = GLFW_KEY_KP_3,
    Numpad4 = GLFW_KEY_KP_4,
    Numpad5 = GLFW_KEY_KP_5,
    Numpad6 = GLFW_KEY_KP_6,
    Numpad7 = GLFW_KEY_KP_7,
    Numpad8 = GLFW_KEY_KP_8,
    Numpad9 = GLFW_KEY_KP_9,
    NumpadDot = GLFW_KEY_KP_DECIMAL,
    NumpadDiv = GLFW_KEY_KP_DIVIDE,
    NumpadMul = GLFW_KEY_KP_MULTIPLY,
    NumpadSub = GLFW_KEY_KP_SUBTRACT,
    NumpadAdd = GLFW_KEY_KP_ADD,
    NumpadEnter = GLFW_KEY_KP_ENTER,

    Mouse_Left = GLFW_MOUSE_BUTTON_LEFT + 10000,
    Mouse_Right = GLFW_MOUSE_BUTTON_RIGHT + 10000,
    Mouse_Middle = GLFW_MOUSE_BUTTON_MIDDLE + 10000,
    Mouse_4 = GLFW_MOUSE_BUTTON_4 + 10000,
    Mouse_5 = GLFW_MOUSE_BUTTON_5 + 10000,

    Pad_A = GLFW_GAMEPAD_BUTTON_A + 20000,
    Pad_B = GLFW_GAMEPAD_BUTTON_B + 20000,
    Pad_X = GLFW_GAMEPAD_BUTTON_X + 20000,
    Pad_Y = GLFW_GAMEPAD_BUTTON_Y + 20000,
    Pad_LB = GLFW_GAMEPAD_BUTTON_LEFT_BUMPER + 20000,
    Pad_RB = GLFW_GAMEPAD_BUTTON_RIGHT_BUMPER + 20000,
    Pad_Back = GLFW_GAMEPAD_BUTTON_BACK + 20000,
    Pad_Start = GLFW_GAMEPAD_BUTTON_START + 20000,
    Pad_Guide = GLFW_GAMEPAD_BUTTON_GUIDE + 20000,
    Pad_L3 = GLFW_GAMEPAD_BUTTON_LEFT_THUMB + 20000,
    Pad_R3 = GLFW_GAMEPAD_BUTTON_RIGHT_THUMB + 20000,
    Pad_Up = GLFW_GAMEPAD_BUTTON_DPAD_UP + 20000,
    Pad_Down = GLFW_GAMEPAD_BUTTON_DPAD_DOWN + 20000,
    Pad_Left = GLFW_GAMEPAD_BUTTON_DPAD_LEFT + 20000,
    Pad_Right = GLFW_GAMEPAD_BUTTON_DPAD_RIGHT + 20000,
};

inline GLFWwindow* _inputWindow = nullptr;
inline std::unordered_map<int, bool> _inputPrevious;

inline bool _getRaw(Key key) {
    if ((int)key >= 10000)
        return glfwGetMouseButton(_inputWindow, (int)key - 10000) == GLFW_PRESS;
    return glfwGetKey(_inputWindow, (int)key) == GLFW_PRESS;
}

inline bool KeyHeld(Key key) {
    return _getRaw(key);
}

inline bool KeyPressed(Key key) {
    bool current = _getRaw(key);
    bool previous = _inputPrevious.count((int)key) ? _inputPrevious[(int)key] : false;
    bool result = current && !previous;
    _inputPrevious[(int)key] = current;
    return result;
}

inline bool KeyUnPressed(Key key) {
    bool current = _getRaw(key);
    bool previous = _inputPrevious.count((int)key) ? _inputPrevious[(int)key] : false;
    bool result = !current && previous;
    _inputPrevious[(int)key] = current;
    return result;
}

inline Vector2 getMousePos() {
    double x, y;
    glfwGetCursorPos(_inputWindow, &x, &y);
    return { (float)x, (float)y };
}

inline void setMouseVisible(bool visible) {
    glfwSetInputMode(_inputWindow, GLFW_CURSOR,
        visible ? GLFW_CURSOR_NORMAL : GLFW_CURSOR_DISABLED);
}

class Runtime {
private:
    std::vector<Scene*> scenes;

    Window& window;

public:
    Runtime(Window& _window)
        : window(_window) 
    {};

    ~Runtime() {};

    void StartRuntime();

    void AddScene(Scene* scene) {
        scenes.push_back(scene);
    }

    Runtime(const Runtime&) = delete;
    Runtime& operator=(const Runtime&) = delete;
};

inline float deltaTime = 0.0f;