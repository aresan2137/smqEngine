#pragma once

#include <string>
#include <vector>
#include <cstdint>

#include "../ExternMakers/sam.h"

namespace smq {

	struct Vector2 {
		float x, y;
	};

	struct Vector2Int {
		int x, y;
	};

	struct Vector3 {
		float x, y, z;
	};

	struct Vector3Int {
		int x, y, z;
	};

	struct Vector4 {
		float x, y, z, w;
	};

	struct Vector4Int {
		int x, y, z, w;
	};

	struct Matrix4 {
		float m[4][4];

		float* operator[](size_t i) {
			return m[i];
		}
	};

	struct Window {
		Vector2Int size;
		std::string title;
	};

	class Object;
	class Camera;

	struct Scene {
		Object* rootObject;
		Camera* camera;
	};

	enum Key : unsigned int {
		Key_Null = 0,

		Key_Mouse_Left = 1,
		Key_Mouse_Right = 2,
		Key_Mouse_Middle = 3,

		Key_A = 65,
		Key_B = 66,
		Key_C = 67,
		Key_D = 68,
		Key_E = 69,
		Key_F = 70,
		Key_G = 71,
		Key_H = 72,
		Key_I = 73,
		Key_J = 74,
		Key_K = 75,
		Key_L = 76,
		Key_M = 77,
		Key_N = 78,
		Key_O = 79,
		Key_P = 80,
		Key_Q = 81,
		Key_R = 82,
		Key_S = 83,
		Key_T = 84,
		Key_U = 85,
		Key_V = 86,
		Key_W = 87,
		Key_X = 88,
		Key_Y = 89,
		Key_Z = 90,

		Key_0 = 48,
		Key_1 = 49,
		Key_2 = 50,
		Key_3 = 51,
		Key_4 = 52,
		Key_5 = 53,
		Key_6 = 54,
		Key_7 = 55,
		Key_8 = 56,
		Key_9 = 57,

		Key_Space = 32,
		Key_Enter = 257,
		Key_Escape = 256,
		Key_Tab = 258,
		Key_Backspace = 259,

		Key_Arrow_Right = 262,
		Key_Arrow_Left = 263,
		Key_Arrow_Down = 264,
		Key_Arrow_Up = 265,

		Key_F1 = 290,
		Key_F2 = 291,
		Key_F3 = 292,
		Key_F4 = 293,
		Key_F5 = 294,
		Key_F6 = 295,
		Key_F7 = 296,
		Key_F8 = 297,
		Key_F9 = 298,
		Key_F10 = 299,
		Key_F11 = 300,
		Key_F12 = 301,
	};	

	enum GlUniform {
		GlUniform_Float,
		GlUniform_Vec2,
		GlUniform_Vec3,
		GlUniform_Vec4,
		GlUniform_Int,
		GlUniform_IVec2,
		GlUniform_IVec3,
		GlUniform_IVec4,
		GlUniform_Mat4
	};

	struct GlAtribute {
		GlUniform type;
		union {
			float f;
			Vector2 f2;
			Vector3 f3;
			Vector4 f4;
			int i;
			Vector2Int i2;
			Vector3Int i3;
			Vector4Int i4;
			Matrix4 m4;
		};
	};

	using Shader = unsigned int;

	class Texture;

	class Material {
	public:

		Material(Shader shader, std::vector<unsigned int> maps, std::vector<GlAtribute> uniforms);

		void ActivateMaterial();

		void UpdateUniform(ShaderUniform uniform, float data);
		void UpdateUniform(ShaderUniform uniform, Vector2 data);
		void UpdateUniform(ShaderUniform uniform, Vector3 data);
		void UpdateUniform(ShaderUniform uniform, Vector4 data);

		void UpdateUniform(ShaderUniform uniform, int data);
		void UpdateUniform(ShaderUniform uniform, Vector2Int data);
		void UpdateUniform(ShaderUniform uniform, Vector3Int data);
		void UpdateUniform(ShaderUniform uniform, Vector4Int data);

		void UpdateUniform(ShaderUniform uniform, Matrix4 data);

		void UpdateMVP(Matrix4 data);
		void AddTexture(Texture* texture);

	private:
		Shader i_shader;
		std::vector<unsigned int> i_maps;
		std::vector<GlAtribute> i_uniforms;

		std::vector<Texture*> i_textures;

		unsigned int i_mvp;
		Matrix4 i_mvpV;

	};

	class Mesh {
	public:

		Mesh();
		Mesh(std::string Filename);
		Mesh(std::vector<float> data, std::vector<unsigned int> indices, std::vector<unsigned int> layout);
		void Delete();

		void ActivateMesh();

		unsigned int GetTriangleCount();

	private:

		unsigned int i_vao;
		unsigned int i_ibo;
		unsigned int i_meshID;

		unsigned int i_triangleCount;
	};

	class Texture {
	public:

		Texture();
		Texture(std::string Filename);
		void Delete();

		bool Valid();

		void ActivateTexture(unsigned int slot = 0);

		Vector2Int Size();

	private:

		unsigned int i_ID = 0;
		std::string i_Filename;
		Vector2Int i_size;
		int i_bpp;

	};

	class Camera {
	public:
		Camera();

		void ReCalculateProjection();

		Vector2Int renderResolution = { 1280,720 };

		Window* window = nullptr;

		Material* postProcesingMaterial = nullptr;

		Vector3 position = { 0.0f,0.0f,0.0f };
		Vector3 rotation = { 0.0f,0.0f,0.0f };

		float FOV = 60.0f;
		bool Perspective = true; // true = perspective, false = ortographic

	private:

	};

	class Object;
	class Component {
	public:
		Component();
		
		Object* GetParent();

		virtual void Start();
		virtual void Update(float Delta);

	private:
		Object* i_object;
	};


	namespace comp {
		class Position3D;
		class ModelMaterial;
	}

	class Object {
	public:
		Object();
		~Object();

		void AddComponent(Component* component);
		void RemoveComponent(Component* component, bool destroy = true);

		template<typename T>
		inline T* FindComponent() {
			for (int i = 0; i < i_components.size(); i++) {
				T* comp = dynamic_cast<T*>(i_components[i]);
				if (comp != nullptr) return comp;
			}
			return nullptr;
		}

		std::vector<Component*> GetAllComponents();

		void AddChild(Object* object);
		void RemoveChild(Object* object, bool destroy = true);
		Object* GetChild(unsigned int count);
		std::vector<Object*> GetAllChildren();

		void SetParent(Object* parent);
		Object* GetParent();

		comp::Position3D* GetPosition3D();
		comp::ModelMaterial* GetModelMaterial();

	private:
		std::vector<Component*> i_components;
		std::vector<Object*> i_children;
		Object* i_parent;

		comp::Position3D* i_Position3D = nullptr;
		comp::ModelMaterial* i_ModelMaterial = nullptr;
	};
}