#include "Smq/Smq.h"

// This Code is a demo to show basic program

// to control camera use wasd
// to rotate camera pres z to togle rotation control

#include "code/RotationScript.h"

int main() {
    
    smq::init({ 1280,720 }, "Game"); // create window and init opengl 

    smq::Scene scene;

    smq::Camera* camera = new smq::Camera();
    scene.camera = camera;
    smq::Object* root = new smq::Object();
    scene.rootObject = root;

    smq::Material material = smq::Material("resources/Shaders/basic_vs.glsl", "resources/Shaders/basic_fs.glsl"); // creates material from shaders

    smq::Texture texture = smq::Texture("resources/textures/face.png"); 
    
    smq::Mesh mesh = smq::Mesh("resources/cube.smf");

    smq::Object* cube = new smq::Object();
    root->AddChild(cube); // adds cube as child of root
    smq::comp::ModelMaterial* cube_ModelMaterial = new smq::comp::ModelMaterial(mesh, material, texture); // component that gives renderer data to render - This is needed to render
    cube->AddComponent(cube_ModelMaterial);
    smq::comp::Position3D* cube_Position3D = new smq::comp::Position3D(); // this contains positional data - This is optional if not added it will render at 0,0,0 with no rotation
    cube->AddComponent(cube_Position3D);
    RotationScript* cube_RotationScript = new RotationScript(cube_Position3D); // this is demo that rotates the cube code is in code/RotationScript.h/cpp
    cube->AddComponent(cube_RotationScript);

    smq::comp::FreeCamera* root_FreeCamera = new smq::comp::FreeCamera(camera); // this allows to control the camera
    root->AddComponent(root_FreeCamera);
    
    

    smq::StartRuntime(scene);

    smq::quit();
}