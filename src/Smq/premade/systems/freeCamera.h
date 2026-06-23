#pragma once

#include "../units.h"
#include "../runtime.h"

class FreeCamera : public System {
public:
    float _speed = 2.5f;
    float _sensitivity = 0.3f;
protected:    
    float _pitch = 0.0f;
    float _yaw = 0.0f;  
    bool _controlling = false;
    Vector2 _prevMousePos = { 0, 0 };
    float _fps = 0.0f;
    float _deltaTime = 0.0f;

	Key _togleKey;
public:
    FreeCamera(Key toggleKey = Key::Z) 
        : _togleKey(toggleKey) 
    {}

    void Start(Scene* scene) override {
        _prevMousePos = getMousePos();
    }

    void Update(Scene* scene) override {
        Camera& cam = scene->cameras[0];

        Window* window = res.get<Window>();

        static double lastTime = glfwGetTime();
        double currentTime = glfwGetTime();
        _deltaTime = (float)(currentTime - lastTime);
        lastTime = currentTime;
        _fps = _fps * 0.95f + (1.0f / _deltaTime) * 0.05f;

        if (KeyPressed(_togleKey)) {
            _controlling = !_controlling;
            setMouseVisible(!_controlling);
            if (_controlling) {
                int w, h;
                glfwGetFramebufferSize(window->window, &w, &h);
                glfwSetCursorPos(window->window, w / 2.0, h / 2.0);
                _prevMousePos = { (float)(w / 2), (float)(h / 2) };
            }
        }

        if (KeyPressed(Key::Escape)) {
            _controlling = false;
            setMouseVisible(true);
        }

        if (_controlling) {
            Vector2 mousePos = getMousePos();
            Vector2 delta = mousePos - _prevMousePos;

            int w, h;
            glfwGetFramebufferSize(window->window, &w, &h);
            glfwSetCursorPos(window->window, w / 2.0, h / 2.0);
            _prevMousePos = { (float)(w / 2), (float)(h / 2) };

            _yaw -= delta.x * _sensitivity;
            _pitch -= delta.y * _sensitivity;
            _pitch = glm::clamp(_pitch, -89.0f, 89.0f);

            Quaternion qYaw = glm::angleAxis(glm::radians(_yaw), Vector3(0, 1, 0));
            Quaternion qPitch = glm::angleAxis(glm::radians(_pitch), Vector3(1, 0, 0));
            cam.rotation = glm::normalize(qYaw * qPitch);
        }

        Vector3 forward = cam.rotation * Vector3(0, 0, -1);
        Vector3 right = cam.rotation * Vector3(1, 0, 0);
        Vector3 up = cam.rotation * Vector3(0, 1, 0);
        float velocity = _speed * _deltaTime;

        if (KeyHeld(Key::W)) cam.position += forward * velocity;
        if (KeyHeld(Key::S)) cam.position -= forward * velocity;
        if (KeyHeld(Key::D)) cam.position += right * velocity;
        if (KeyHeld(Key::A)) cam.position -= right * velocity;
        if (KeyHeld(Key::E)) cam.position += up * velocity;
        if (KeyHeld(Key::Q)) cam.position -= up * velocity;

        ImGui::Begin("Debug");
        ImGui::Text("fps: %f", 1/deltaTime);
        ImGui::Text("delta: %f", deltaTime);
        ImGui::End();
    }

    ~FreeCamera() {}

    FreeCamera(const FreeCamera&) = delete;
    FreeCamera& operator=(const FreeCamera&) = delete;
};