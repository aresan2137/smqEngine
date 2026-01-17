
#pragma once

enum ShaderUniform : unsigned int {
bacic_vs_u_mvp = 0,
bacic_fs_u_tex = 1,
noPost_fs_u_tex = 0,
noPost_fs_u_depth = 1,

};

enum Shaders {
bacic,
noPost,

};

enum Meshes {
cube,
shelf,

};

enum Textures {
tile,

};

namespace smq {
	class Material;
    class Mesh;
    class Texture;
}

void initSam();
smq::Material* getMaterial(Shaders shader);
smq::Mesh* getMesh(Meshes mesh);
smq::Texture* getTexture(Textures texture);

