#include "../Include.h"

namespace smq {
    void Engine::DrawScene() {

        // Render All

        for (int i = 0; i < currentScene->cameras.size(); i++) {
            currentScene->cameras[i].Render();
        }

        // Draw To Screen

        glBindFramebuffer(GL_FRAMEBUFFER, 0);
        glViewport(0, 0, i_window.size.x, i_window.size.y);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        plane->ActivateMesh();

        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, currentScene->renderTexture->GetTexture());
        i_post->UpdateUniform((MaterialUniforms)0, 0);

        glActiveTexture(GL_TEXTURE1);
        glBindTexture(GL_TEXTURE_2D, currentScene->renderTexture->GetTextureDepth());
        i_post->UpdateUniform((MaterialUniforms)1, 1);

        i_post->ActivateMaterial();

        glDrawElements(GL_TRIANGLES, plane->GetTriangleCount(), GL_UNSIGNED_INT, nullptr);
    }
}

