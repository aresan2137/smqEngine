#include "RotationScript.h"

#include "../smq/smq.h"


RotationScript::RotationScript(smq::comp::Position3D* posi) 
	: pos(posi)
{

}

void RotationScript::Start() {
	pos->rotation = { 45,0,0 };
}

void RotationScript::Update(float Delta) {
	pos->rotation.z += Delta * 20;
}
