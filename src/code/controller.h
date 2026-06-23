#pragma once

class Player2DControler : public FreeCamera {
    void Update(Scene* scene) override {
        if (scene->cameras.empty()) return;
        Camera& cam = scene->cameras[0];
        Window* window = res.get<Window>();

        static double lastTime = glfwGetTime();
        double currentTime = glfwGetTime();
        _deltaTime = (float)(currentTime - lastTime);
        lastTime = currentTime;

        if (KeyPressed(Key::Z)) {
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
        forward.y = 0.0f;
        forward = glm::normalize(forward);

        Vector3 right = cam.rotation * Vector3(1, 0, 0);
        right.y = 0.0f;
        right = glm::normalize(right);

        float velocity = _speed * _deltaTime;
        Vector3 movement = Vector3(0, 0, 0);

        if (KeyHeld(Key::W)) movement += forward;
        if (KeyHeld(Key::S)) movement -= forward;
        if (KeyHeld(Key::D)) movement += right;
        if (KeyHeld(Key::A)) movement -= right;

        if (glm::length(movement) > 0.0001f) {
            movement = glm::normalize(movement) * velocity;
        }

        cam.position += movement;

        if (KeyHeld(Key::LeftCtrl)) {
            cam.position.y = 1.8f;
			_speed = 1.5f;
        } else {
            cam.position.y = 2.6f;
            _speed = 2.5f;
        }

        
    }
};
