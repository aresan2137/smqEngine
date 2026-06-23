#include "../smq.h"

void Runtime::StartRuntime() {
	Scene* currentScene = scenes[0];

	_inputWindow = window.window;

	for (size_t i = 0; i < currentScene->systems.size(); i++) {
		currentScene->systems[i]->Start(currentScene);
	}

    double lastTime = glfwGetTime();

	while (!glfwWindowShouldClose(window.window)) {

        double now = glfwGetTime();
        deltaTime = (float)(now - lastTime);
        lastTime = now;

        glfwPollEvents();

        for (auto sys : currentScene->systems) {
            sys->EarlyUpdate(currentScene);
        }

        for (auto sys : currentScene->systems) {
            sys->Update(currentScene);
        }

        for (auto sys : currentScene->systems) {
            sys->LateUpdate(currentScene);
        }

        for (auto& [key, val] : _inputPrevious)
            val = _getRaw((Key)key);
    }
}