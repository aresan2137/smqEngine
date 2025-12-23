#pragma once

#include "../smq/smq.h"

class RotationScript : public smq::Component {
public:
	RotationScript(smq::comp::Position3D* posi);

	void Start() override;

	void Update(float Delta) override;

private:
	smq::comp::Position3D* pos;
};
