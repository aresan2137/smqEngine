#include "Smq/Smq.h"

#include "ExternMakers/sam.h"

int main() {

	smq::Window* window = new smq::Window({{1280,720}, "Game" });
    
    smq::init(window);

	initSam();

	getMaterial(bacic)->AddTexture(getTexture(tile));

	smq::PlayMusicLoop("resources/textures/music.wav");
	smq::SetMusicVolume(0.8f);

	smq::Scene* scene = new smq::Scene();
	scene->camera = new smq::Camera();
	scene->camera->window = window;
	scene->camera->postProcesingMaterial = getMaterial(noPost);

	scene->rootObject = new smq::Object();
	scene->rootObject->AddComponent(new smq::comp::FreeCamera(scene->camera));

	smq::Object* cube_o = new smq::Object();
	cube_o->AddComponent(new smq::comp::Position3D());
	cube_o->GetPosition3D()->position = { 0.0f,0.0f,-5.0f };
	cube_o->AddComponent(new smq::comp::ModelMaterial(getMesh(cube), getMaterial(bacic)));
	scene->rootObject->AddChild(cube_o);

    smq::StartRuntime(scene);

    smq::quit();
}

// PostProcesing note uniform1 = image, uniform2 = depth buffer, rest is free

// TODO:
// add removing comments from shaders - Not inportant
// add sound engine
// make ssc - Not inportant
// add Bullet Physics
