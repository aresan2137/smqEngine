#pragma once

#include <string>
#include <vector>
#include <cstdint>

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

	enum Key : unsigned int {
		Key_Null = 0,

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

		Key_Right = 262,
		Key_Left = 263,
		Key_Down = 264,
		Key_Up = 265,

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

	class Material {
	public:

		Material();
		Material::Material(const std::string vertexShaderFilename, const std::string fragmentShadeFilename);
		void Delete();

		void ActivateMaterial();

		void UpdateAtribute(std::string name, float value);
		void UpdateAtribute(std::string name, Vector2 value);
		void UpdateAtribute(std::string name, Vector3 value);
		void UpdateAtribute(std::string name, Vector4 value);

		void UpdateAtribute(std::string name, int value);
		void UpdateAtribute(std::string name, Vector2Int value);
		void UpdateAtribute(std::string name, Vector3Int value);
		void UpdateAtribute(std::string name, Vector4Int value);
		
		void UpdateAtribute(std::string name, Matrix4 value);

		void SetMVP(Matrix4 value);
		void SetTexture(int slot);

	private:
		unsigned int i_shader;

		int i_mvp = -1;
		int i_tex = -1;
	};

	class Mesh {
	public:

		Mesh();
		Mesh(std::string Filename); // Loads smf file
		Mesh(std::vector<float> data, std::vector<unsigned int> indices);
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

		Vector3 position = { 0.0f,0.0f,0.0f };
		Vector3 rotation = { 0.0f,0.0f,0.0f };

	private:

	};

	class Object;
	class Component {
	public:
		Component();
		
		Object* GetObject();

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