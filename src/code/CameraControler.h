#pragma once

#include "../smq/smq.h"

#include "Mygui.h"

class CameraControler : public smq::Component {
public:
	CameraControler(smq::Camera* camera, MyGui* gui);

	void Start() override;

	void Update(float Delta) override;

private:
	smq::Camera* cam;
	MyGui* gu;

	smq::Vector2 last;

	bool z = false;
};
