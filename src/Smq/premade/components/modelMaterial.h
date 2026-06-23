#pragma once

#include "../units.h"
#include "../runtime.h"

class ModelMaterial : public Component {
public:
	Mesh* mesh;
	Material* material;

	ModelMaterial(Mesh* _mesh, Material* _material)
		: mesh(_mesh)
		, material(_material)
	{};

	~ModelMaterial() {};
};
