#include "Smq/Smq.h"

#include "code/Mygui.h"
#include "code/CameraControler.h"
#include "code/RotationScript.h"

int main() {
    
    smq::init({ 1280,720 }, "Game");    

    smq::Scene scene;
    smq::Camera* camera = new smq::Camera();
    scene.camera = camera;
    smq::Object* root = new smq::Object();
    scene.rootObject = root;

    smq::Material material = smq::Material("resources/basic_vs.glsl", "resources/basic_fs.glsl");

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
    MyGui* root_MyGui = new MyGui(cube_Position3D);
    root->AddComponent(root_MyGui);
    CameraControler* root_CameraControler = new CameraControler(camera, root_MyGui);
    root->AddComponent(root_CameraControler);
    
    

    smq::StartRuntime(scene);

    smq::quit();
}