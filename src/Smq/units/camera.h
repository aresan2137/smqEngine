#pragma once

#include "units.h"
#include "renderTexture.h"

class Camera {
public:
    enum class ProjectionType : uint8_t {
        Perspective,
        Orthographic
    };

    struct Settings {
        float near = 0.01f;
        float far = 1000.0f;
        float FOV = 60.0f;
        float size = 5.0f;
    };

    Vector3 position = { 0, 0, 0 };
    Quaternion rotation = glm::identity<Quaternion>();

private:
    Matrix4 proj;
    ProjectionType projection;
    float aspectRatio;
    Settings settings;

    void recalculateProj() {
        if (projection == ProjectionType::Perspective) {
            proj = glm::perspective(
                glm::radians(settings.FOV),
                aspectRatio,
                settings.near,
                settings.far
            );
        } else {
            float h = settings.size;
            float w = h * aspectRatio;
            proj = glm::ortho(-w, w, -h, h, settings.near, settings.far);
        }
    }

public:
    RenderTexture* renderTexture;

    Camera(RenderTexture* _renderTexture, Settings _settings = {}, ProjectionType _projection = ProjectionType::Perspective, float _aspectRatio = 1280.0f / 720.0f)
        : settings(_settings)
        , projection(_projection)
        , aspectRatio(_aspectRatio) 
        , renderTexture(_renderTexture)
    {
        recalculateProj();
    }

    void setSettings(Settings s) {
        settings = s;
        recalculateProj();
    }

    Settings getSettings() const { return settings; }

    void setProjection(ProjectionType _projection) {
        projection = _projection;
        recalculateProj();
    }

    void setAspectRatio(float _aspectRatio) {
        if (aspectRatio != _aspectRatio) {
            aspectRatio = _aspectRatio;
            recalculateProj();
        }        
    }

    ProjectionType getProjection() const { return projection; }

    Matrix4 getView() const {
        Matrix4 view = glm::mat4_cast(glm::inverse(rotation));
        view = glm::translate(view, -position);
        return view;
    }

    Matrix4 getVP() const {
        return proj * getView();
    }
};