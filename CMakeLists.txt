cmake_minimum_required(VERSION 3.24)

set(CMAKE_GENERATOR_PLATFORM "x64" CACHE INTERNAL "" FORCE)
set(CMAKE_VS_PLATFORM_NAME "x64" CACHE INTERNAL "" FORCE)

project(SmqEngine LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(CMAKE_SIZEOF_VOID_P EQUAL 4)
    message(FATAL_ERROR "vs2026 is still broken")
endif()

file(GLOB_RECURSE SmqSOURCES "src/**.cpp")
add_executable(smq ${SmqSOURCES})

# settings
target_compile_definitions(smq PUBLIC SMQ_OBJECT_NAMES)

# headers
target_include_directories(smq PRIVATE libs/headers)

# GLAD
add_library(glad STATIC libs/glad/src/glad.c)
target_include_directories(glad PUBLIC libs/glad/include)

# GLFW
add_subdirectory(libs/glfw-3.4)

# GLM
add_library(glm INTERFACE)
target_include_directories(glm INTERFACE libs/glm-1.0.2)

# FAST NOISE 2
add_subdirectory(libs/FastNoise2-1.1.1)

# bullet3 (wyłączamy dema i testy, kompiluje się standardowo bez LTO)
#set(BUILD_UNIT_TESTS OFF CACHE INTERNAL "" FORCE)
#set(BUILD_CPU_DEMOS OFF CACHE INTERNAL "" FORCE)
#set(BUILD_BULLET2_DEMOS OFF CACHE INTERNAL "" FORCE)
#set(BUILD_EXTRAS OFF CACHE INTERNAL "" FORCE)
#set(BUILD_OPENGL3_DEMOS OFF CACHE INTERNAL "" FORCE)
#add_subdirectory(libs/bullet3)
#target_include_directories(smq PRIVATE libs/bullet3/src)

# IMGUI
add_library(imgui STATIC
    libs/imgui-1.92.5/imgui.cpp
    libs/imgui-1.92.5/imgui_draw.cpp
    libs/imgui-1.92.5/imgui_tables.cpp
    libs/imgui-1.92.5/imgui_widgets.cpp
    libs/imgui-1.92.5/imgui_demo.cpp
    libs/imgui-1.92.5/backends/imgui_impl_glfw.cpp
    libs/imgui-1.92.5/backends/imgui_impl_opengl3.cpp
)
target_include_directories(imgui PUBLIC 
    libs/imgui-1.92.5 
    libs/imgui-1.92.5/backends
    libs/glad/include
)
target_link_libraries(imgui PUBLIC glfw glad)

# MINIZ
add_subdirectory(libs/miniz-3.1.1)

target_link_libraries(smq PUBLIC
    glfw
    glad
    miniz
    imgui
    glm
    opengl32
    FastNoise2
    #BulletDynamics
    #BulletCollision
    #LinearMath
)

# --- SELEKTYWNE WŁĄCZENIE LTO / IPO ---
#Odpalamy optymalizację tylko dla wybranych celów, całkowicie ignorując Bulleta!



include(CheckIPOSupported)
check_ipo_supported(RESULT ipo_supported OUTPUT ipo_error)
if(ipo_supported)
    set_target_properties(smq glad imgui glfw FastNoise2 PROPERTIES INTERPROCEDURAL_OPTIMIZATION TRUE)
    message(STATUS "[LTO] Wlaczone dla: smq, glad, imgui, glfw, FastNoise2. Bullet3 pominiety.")
else()
    message(WARNING "[LTO] Nieobsługiwane przez kompilator: ${ipo_error}")
endif()


add_custom_command(TARGET smq POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
    "${CMAKE_SOURCE_DIR}/src/assets"
    "$<TARGET_FILE_DIR:smq>/assets"
)

find_package(Python3 REQUIRED COMPONENTS Interpreter)
add_custom_command(TARGET smq PRE_BUILD
    COMMAND ${Python3_EXECUTABLE} "${CMAKE_SOURCE_DIR}/src/assets/smf.py" "objs.obj" "game"
    COMMAND ${Python3_EXECUTABLE} "${CMAKE_SOURCE_DIR}/src/assets/ssf.py" "game"
    COMMAND ${CMAKE_COMMAND} -E copy_directory
        "${CMAKE_SOURCE_DIR}/src/assets"
        "$<TARGET_FILE_DIR:smq>/assets"
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}/src/assets"
    COMMENT "Running asset pipeline..."
)