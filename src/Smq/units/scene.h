#pragma once

#include "units.h"
#include "camera.h"
#include "system.h"
#include "object.h"

#include "../runtime.h"

class Scene {
public:
	
	Object* rootObject;
	std::vector<Camera> cameras;
	std::vector<System*> systems;
	RenderTexture* output;

	template<typename T>
	void addSystem(T* system) {
		systems.push_back(system);
		res.insert(system);
	}

	Scene() {};

	Scene(const Scene&) = delete;
	Scene& operator=(const Scene&) = delete;
};
