#include <Include.h>

#include "ExternMakers/sam.h"

#include "Code/test.h"

smq::Engine eng = smq::Engine(smq::Window({ {1280,720}, "Game" }));

void SetupCameras(smq::Scene* scene) {
	// main camera
	smq::Renderer* rendCamRenderer = new smq::DefultRenderer();
	smq::RenderTexture* mainCam = new smq::RenderTexture({ 1280,720 }, rendCamRenderer);
	scene->renderTexture = mainCam;
	scene->cameras.emplace_back(mainCam, smq::RenderType_Perspective);

	// shadow camera
	smq::Renderer* ShadowCamRenderer = new smq::DefultRenderer();
	smq::RenderTexture* shadowCam = new smq::RenderTexture({ 4096,4096 }, ShadowCamRenderer, true);
	scene->cameras.emplace_back(shadowCam, smq::RenderType_Orthographic);
	scene->cameras[1].orthographic.size = 100.0f;
	scene->cameras[1].orthographic.nearP = 0.1f;
	scene->cameras[1].orthographic.farP = 500.0f;
	scene->cameras[1].position = { 0.0f, 100.0f, 0.0f };
	scene->cameras[1].rotation = { -90.0f, 0.0f, 0.0f };
	scene->cameras[1].ReCalculateProjection();

	GetMaterial(s_shadow)->AddTexture(new smq::Texture(shadowCam->GetTextureDepth(), { 4096,4096 }));
	GetMaterial(s_shadow)->UpdateUniform(s_shadow_fs_u_shadowMap, (int)1);
}

void SetupObjects(smq::Scene* scene) {
	scene->rootObject = new smq::Object();
	scene->rootObject->AddComponent(new smq::comp::FreeCamera(&scene->cameras[0]));
	scene->rootObject->AddComponent(new Test());

	smq::Object* floor = new smq::Object();
	floor->AddComponent(new smq::comp::Position3D({ 0,0,0 }, { -90,0,0 }, {1,1,1}));
	floor->AddComponent(new smq::comp::ModelMaterial(GetMesh(m_plane), GetMaterial(s_shadow)));
	scene->rootObject->AddChild(floor);

	smq::Object* ball = new smq::Object();
	ball->AddComponent(new smq::comp::Position3D({ 0,10,0 }, { -90,0,0 }, { 1,1,1 }));
	ball->AddComponent(new smq::comp::ModelMaterial(GetMesh(m_ball), GetMaterial(s_shadow)));
	scene->rootObject->AddChild(ball);
}

int main() {
	smq::Scene* scene = new smq::Scene();

	initSam();
	 
	//smq::PlayMusicLoop("resources/textures/music.mp3");
	//smq::SetMusicVolume(0.15f);

	SetupCameras(scene);

	SetupObjects(scene);

	smq::Light l;
	l.position = { 0.0f, 100.0f, 0.0f, 1000.0f };
	l.color = { 1.0f, 1.0f, 1.0f, 1.0f };
	scene->lights.push_back(l);

	GetMaterial(s_shadow)->UpdateUniform(s_shadow_fs_u_lightCount, (int)scene->lights.size());

	eng.SetPostProcesingMaterial(GetMaterial(s_post));
	eng.SetScene(scene);

    eng.StartRuntime();

	quitSam();
}

// PostProcesing note uniform1 = image, uniform2 = depth buffer, rest is free