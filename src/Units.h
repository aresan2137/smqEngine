#pragma once

#include <string>
#include <vector>
#include <cstdint>
#include <thread>
#include <atomic>
#include <mutex>

#include <btBulletDynamicsCommon.h>

#define GLM_ENABLE_EXPERIMENTAL
#include "glm.hpp"
#include "gtc/quaternion.hpp"
#include "gtc/matrix_transform.hpp"
#include "gtc/type_ptr.hpp"
#include "gtx/quaternion.hpp"

enum MaterialUniforms : unsigned int;
struct GLFWwindow;
struct ImGuiIO;
struct ImGuiContext;

namespace smq {

	using Vector2 = glm::vec2;
	using Vector3 = glm::vec3;
	using Vector4 = glm::vec4;

	using Vector2Int = glm::ivec2; 
	using Vector3Int = glm::ivec3;
	using Vector4Int = glm::ivec4;

	using Matrix4 = glm::mat4;
	using Matrix3 = glm::mat3;

	struct Window {
		Vector2Int size;
		std::string title;
	};

	struct Light {
		Vector4 position;
		Vector4 color;
	};

	class Object;
	class Camera;
	class RenderTexture;

	struct Scene {
		Object* rootObject;
		RenderTexture* renderTexture;
		std::vector<Camera> cameras;
		std::vector<Light> lights;
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
		GlUniform_Mat3,
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
			Matrix3 m3;
			Matrix4 m4;
		};
	};

	using Shader = unsigned int;

	class Texture;	

	class Material {
	public:

		Material(Shader shader, std::vector<unsigned int> maps, std::vector<GlAtribute> uniforms);

		void ActivateMaterial();

		void UpdateUniform(MaterialUniforms uniform, float data);
		void UpdateUniform(MaterialUniforms uniform, Vector2 data);
		void UpdateUniform(MaterialUniforms uniform, Vector3 data);
		void UpdateUniform(MaterialUniforms uniform, Vector4 data);

		void UpdateUniform(MaterialUniforms uniform, int data);
		void UpdateUniform(MaterialUniforms uniform, Vector2Int data);
		void UpdateUniform(MaterialUniforms uniform, Vector3Int data);
		void UpdateUniform(MaterialUniforms uniform, Vector4Int data);

		void UpdateUniform(MaterialUniforms uniform, Matrix3 data);
		void UpdateUniform(MaterialUniforms uniform, Matrix4 data);

		void UpdateMVP(Matrix4 data);
		void UpdateNormalMVP(Matrix3 data);
		void UpdateM(Matrix4 data);

		void AddTexture(Texture* texture);

		Material(const Material&) = delete;
		Material& operator=(const Material&) = delete;

	private:
		Shader i_shader;
		std::vector<unsigned int> i_maps;
		std::vector<GlAtribute> i_uniforms;

		std::vector<Texture*> i_textures;

		unsigned int i_mvp;
		unsigned int i_normal_mvp;
		unsigned int i_M;
		
		Matrix4 i_mvpV;
		Matrix3 i_normal_mvpV;
		Matrix4 i_MV;

	};

	class Mesh {
	public:

		Mesh(std::string Filename);
		Mesh(std::vector<float> data, std::vector<unsigned int> indices, std::vector<unsigned int> layout);

		~Mesh();

		void ActivateMesh();

		unsigned int GetTriangleCount();

		Mesh(const Mesh&) = delete;
		Mesh& operator=(const Mesh&) = delete;

	private:

		unsigned int i_vao;
		unsigned int i_ibo;
		unsigned int i_meshID;

		unsigned int i_triangleCount;
	};

	class Texture {
	public:

		Texture(std::string Filename);
		Texture(unsigned int ID, Vector2Int size);

		~Texture();

		bool Valid();

		void ActivateTexture(unsigned int slot = 0);

		Vector2Int Size();

		Texture(const Texture&) = delete;
		Texture& operator=(const Texture&) = delete;

	private:

		unsigned int i_ID = 0;
		std::string i_Filename;
		Vector2Int i_size;
		int i_bpp;

	};

	enum RenderType {
		RenderType_Orthographic,
		RenderType_Perspective
	};

	class Camera {
	public:

		Camera(RenderTexture* renderTexture, RenderType type);

		void ReCalculateProjection();

		void Render();

		RenderTexture* GetRenderTexture();
		Matrix4 GetVP();

		Vector3 position = { 0.0f,0.0f,0.0f };
		Vector3 rotation = { 0.0f,0.0f,0.0f };		


		union {
			struct {
				float size;
				float nearP;
				float farP;
			} orthographic;

			struct {
				float fov;
				float nearP;
				float farP;
			} perspective;
		};		

	private:
		RenderTexture* i_renderTexture;

		RenderType i_type;

		Matrix4 proj;
	};

	class Object;
	class Component {
	public:
		Component();
		virtual ~Component() = default;
		
		Object* GetParent();
		void SetParent(Object* object);

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
		Object* GetChildByTag(uint64_t tag);
		std::vector<Object*>* GetAllChildren();

		void SetParent(Object* parent);
		Object* GetParent();

		void SetTag(uint64_t tag);
		uint64_t GetTag();

		comp::Position3D* GetPosition3D();
		comp::ModelMaterial* GetModelMaterial();

	private:
		uint64_t i_tag;

		std::vector<Component*> i_components;
		std::vector<Object*> i_children;
		Object* i_parent = nullptr;

		comp::Position3D* i_Position3D = nullptr;
		comp::ModelMaterial* i_ModelMaterial = nullptr;
	};

	class Renderer;

	class RenderTexture {
	public:
		RenderTexture(Vector2Int size, Renderer* renderer, bool depthOnly = false);
		~RenderTexture();

		bool Valid();

	    void ActivateRenderTexture();

    	void Clear(bool color = true, bool depth = true);

		void Render(Matrix4 vp);

		Vector2Int GetSize();

		unsigned int GetTexture();
		unsigned int GetTextureDepth();

		RenderTexture(const RenderTexture&) = delete;
		RenderTexture& operator=(const RenderTexture&) = delete;

	private:

		unsigned int i_fbo;
		unsigned int i_texture;
		unsigned int i_textureDepth;

		Vector2Int i_size;

		Renderer* i_renderer;

	};

	class Renderer {
	public:
		Renderer();
		~Renderer();

		virtual void Render(Scene* scene, RenderTexture* renderTexture, Matrix4 vp);

		Renderer(const Renderer&) = delete;
		Renderer& operator=(const Renderer&) = delete;

	private:

	};

	class Engine {
	public:
		Engine(Window window);
		~Engine();

		void SetScene(Scene* scene);
		Scene* GetScene();

		void SetPostProcesingMaterial(Material* material);

		void StartRuntime();

		ImGuiContext* GetImGuiContext();

		Engine(const Engine&) = delete;
		Engine& operator=(const Engine&) = delete;

	private:

		// Drawing

		void DrawScene();
		Mesh* plane;
		Material* i_post;

		// Other

		Scene* currentScene;

		Window i_window;
		GLFWwindow* i_GlfwWindow;
		ImGuiIO* i_ImIo;
		ImGuiContext* i_ImGuiContext;
	};

	class SSBO {
	public:
		SSBO(int bindingPoint);
		~SSBO();

		void SetData(void* data, int size);

		void ActivateSSBO();

		void SetBindingPoint(int Binding);

		SSBO(const SSBO&) = delete;
		SSBO& operator=(const SSBO&) = delete;

	private:
		unsigned int m_ID;
		int m_BindingPoint;

	};
	
	class Physics {
	public:
		Physics();

		void Start();
		void Stop();

		btDiscreteDynamicsWorld* GetWorld();

		std::mutex& GetMutex();

		void addRigidBody(Object* object, btRigidBody* body);

		Physics(const Physics&) = delete;
		Physics& operator=(const Physics&) = delete;

	private:
		void PhysicsLoop();

		btDiscreteDynamicsWorld* i_world = nullptr;
		btDefaultCollisionConfiguration* i_collisionConfig = nullptr;
		btCollisionDispatcher* i_dispatcher = nullptr;
		btDbvtBroadphase* i_broadphase = nullptr;
		btSequentialImpulseConstraintSolver* i_solver = nullptr;

		std::thread i_physicsThread;
		std::atomic<bool> i_running{ false };
		std::mutex i_physicsMtx;

		std::vector<Object*> i_objects;
		std::vector<btRigidBody*> i_bodys;

	};
}