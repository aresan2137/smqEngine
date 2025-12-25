#include "Smq/Smq.h"

#include "code/RotationScript.h"

int main() {
    
    smq::init({ 1280,720 }, "Game");    

    smq::Scene scene;
    smq::Camera* camera = new smq::Camera();
    scene.camera = camera;
    smq::Object* root = new smq::Object();
    scene.rootObject = root;

    smq::Material material = smq::Material("resources/Shaders/basic_vs.glsl", "resources/Shaders/basic_fs.glsl");

    smq::Texture texture1 = smq::Texture("resources/textures/cord.png");
    smq::Texture texture2 = smq::Texture("resources/textures/tbj.png");

    smq::Mesh mesh1 = smq::Mesh("resources/coridor.smf");
    smq::Mesh mesh2 = smq::Mesh("resources/cube.smf");

    smq::Object* cube = new smq::Object();
    root->AddChild(cube);
    smq::comp::ModelMaterial* cube_ModelMaterial = new smq::comp::ModelMaterial(mesh2, material, texture2);
    cube->AddComponent(cube_ModelMaterial);
    smq::comp::Position3D* cube_Position3D = new smq::comp::Position3D();
    cube->AddComponent(cube_Position3D);
    RotationScript* cube_RotationScript = new RotationScript(cube_Position3D);
    cube->AddComponent(cube_RotationScript);


    smq::comp::ModelMaterial* root_ModelMaterial = new smq::comp::ModelMaterial(mesh1, material, texture1);
    root->AddComponent(root_ModelMaterial);
    smq::comp::Position3D* root_Position3D = new smq::comp::Position3D();
    root->AddComponent(root_Position3D);
    smq::comp::FreeCamera* root_FreeCamera = new smq::comp::FreeCamera(camera);
    root->AddComponent(root_FreeCamera);
    
    

    smq::StartRuntime(scene);

    smq::quit();
}