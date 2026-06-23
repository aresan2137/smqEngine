#pragma once

#include "units.h"

class Scene;

class System {
public:
	System() {};
	virtual ~System() {};

	virtual void Start(Scene* scene) {};
	virtual void Update(Scene* scene) {};
	virtual void LateUpdate(Scene* scene) {};
	virtual void EarlyUpdate(Scene* scene) {};

	System(const System&) = delete;
	System& operator=(const System&) = delete;
};
