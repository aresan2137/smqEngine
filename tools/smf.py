import struct

# Instructons For Blender Export
# Export As obj
# Check Off normals and material 
# also check on triangulate and check selected only becose this code will not work prolely with more than 1 object
# modify input sourse
in_file = "here .obj"
# modify output folder
out_file = "here .smf"
# run script
# shud be working

# abaut materials
# you need to make shaders in glsl and create material in code
# very simple materials are alredy made basic_vs and basic_fs they are very simple tho no shading no nothing they do show texture

# for now there isn't a way to convert non obj files but if you want you can do this and commit/pull request this

def load_obj(filename):
    vertices = []
    uvs = []
    faces = []

    with open(filename, "r") as f:
        for line in f:
            if line.startswith("v "):
                parts = line.strip().split()
                x, y, z = map(float, parts[1:4])
                vertices.append((x, y, z))
            elif line.startswith("vt "):
                parts = line.strip().split()
                u, v = map(float, parts[1:3])
                uvs.append((u, v))
            elif line.startswith("f "):
                parts = line.strip().split()[1:]
                face = []
                for p in parts:
                    vals = p.split("/")
                    vi = int(vals[0]) - 1
                    ti = int(vals[1]) - 1 if len(vals) > 1 and vals[1] != '' else -1
                    face.append((vi, ti))
                if len(face) == 3:
                    faces.append(face)
                elif len(face) == 4:
                    faces.append([face[0], face[1], face[2]])
                    faces.append([face[0], face[2], face[3]])
    return vertices, uvs, faces

def save_smf(vertices, uvs, faces, output_file):
    unique_vertices = []
    indices = []
    vertex_map = {}

    for face in faces:
        for vi, ti in face:
            if ti >= 0:
                uv = uvs[ti]
            else:
                uv = (0.0, 0.0)

            key = (vi, ti)

            if key not in vertex_map:
                x, y, z = vertices[vi]
                unique_vertices.append((x, y, z, uv[0], uv[1]))
                vertex_map[key] = len(unique_vertices) - 1

            indices.append(vertex_map[key])

    with open(output_file, "wb") as f:
        f.write(struct.pack("<I", len(unique_vertices)))
        f.write(struct.pack("<I", len(faces)))
        for v in unique_vertices:
            f.write(struct.pack("<5f", *v))
        for i in range(0, len(indices), 3):
            f.write(struct.pack("<3I", indices[i], indices[i+1], indices[i+2]))

if __name__ == "__main__":
    obj_file = in_file
    output_file = out_file
    vertices, uvs, faces = load_obj(obj_file)
    save_smf(vertices, uvs, faces, output_file)
    print(f"file {output_file} generated with {len(vertices)} verticies and {len(faces)} thriangles")
