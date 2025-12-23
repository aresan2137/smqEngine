#pragma once

#include "../Smq/Smq.h"

class MyGui : public smq::Component {
public:
	MyGui::MyGui(smq::comp::Position3D* pos3d);

	void Start() override;

	void Update(float Delta) override;

	float camspeed = 20.0f;
	float sensivty = 0.5f;
	smq::Vector3 camrot = { 0.0f, 0.0f, 0.0f };

private:
	smq::comp::Position3D* pos3db;
};
