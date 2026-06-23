#pragma once

#include "units.h"
#include "component.h"

class Object {
private:
    static inline uint32_t _nextId = 0;
    std::unordered_map<std::type_index, Component*> _components;

public:
    const uint32_t id;
    Tag tag = static_cast<Tag>(0);
    Tag2 tag2 = static_cast<Tag2>(0);
#ifdef SMQ_OBJECT_NAMES
    std::string name = "";
#endif // SMQ_OBJECT_NAMES

    Object* parent = nullptr;
    std::vector<Object*> kids;

    Object() : id(_nextId++) {}

    template<typename T>
    T* addComponent(T* component) {
        static_assert(std::is_base_of_v<Component, T>, "T must inherit from Component");
        auto key = std::type_index(typeid(T));
        if (_components.count(key)) { warn("Component already exists"); return nullptr; }
        _components[key] = component;
        return component;
    }

    template<typename T>
    T* getComponent() {
        auto it = _components.find(std::type_index(typeid(T)));
        if (it == _components.end()) return nullptr;
        return static_cast<T*>(it->second);
    }

    template<typename T>
    bool hasComponent() {
        return _components.count(std::type_index(typeid(T))) > 0;
    }

    template<typename T>
    void removeComponent() {
        auto it = _components.find(std::type_index(typeid(T)));
        if (it == _components.end()) return;
        delete it->second;
        _components.erase(it);
    }

    void addObject(Object* kid) {
        kid->parent = this;
        kids.push_back(kid);
    }

    void removeKid(Object* kid) {
        kids.erase(
            std::remove(kids.begin(), kids.end(), kid),
            kids.end()
        );
        kid->parent = nullptr;
    }

    Object* findByTag(Tag _tag) {
        for (auto* kid : kids) {
            if (kid->tag == _tag) return kid;
            auto* found = kid->findByTag(_tag);
            if (found) return found;
        }
        return nullptr;
    }

    Object* findByTag2(Tag2 _tag) {
        for (auto* kid : kids) {
            if (kid->tag2 == _tag) return kid;
            auto* found = kid->findByTag2(_tag);
            if (found) return found;
        }
        return nullptr;
    }

    Object* findById(uint32_t _id) {
        for (auto* kid : kids) {
            if (kid->id == _id) return kid;
            auto* found = kid->findById(_id);
            if (found) return found;
        }
        return nullptr;
    }

    ~Object() {
        for (auto& [type, comp] : _components) delete comp;
        for (auto* kid : kids) delete kid;
        parent->removeKid(this);
    }

    Object(const Object&) = delete;
    Object& operator=(const Object&) = delete;
};