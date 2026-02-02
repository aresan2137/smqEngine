#include "test.h"

#include "../ExternMakers/sam.h"

#include <Include.h>

Test::Test() {

}

void Test::Start() {

}

void Test::Update(float Delta) {

	GetMaterial(s_shadow)->UpdateUniform(s_shadow_vs_u_lightSpaceMatrix, eng.GetScene()->cameras[1].GetVP());
}

