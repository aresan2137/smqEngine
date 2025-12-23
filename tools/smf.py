import struct

def load_obj(filename):
    vertices = []
    uvs = []
    faces = []

    # Wczytujemy OBJ
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
                    # Format: v/vt (ignorujemy vn)
                    vals = p.split("/")
                    vi = int(vals[0]) - 1
                    ti = int(vals[1]) - 1 if len(vals) > 1 and vals[1] != '' else -1
                    face.append((vi, ti))
                # Triangulacja quads jeśli trzeba
                if len(face) == 3:
                    faces.append(face)
                elif len(face) == 4:
                    # split quada na dwa trójkąty
                    faces.append([face[0], face[1], face[2]])
                    faces.append([face[0], face[2], face[3]])
    return vertices, uvs, faces

def save_megamesh(vertices, uvs, faces, output_file):
    # Tworzymy unikalne vertex + UV
    unique_vertices = []
    indices = []
    vertex_map = {}
    
    for face in faces:
        for vi, ti in face:
            if ti >= 0:
                uv = uvs[ti]
            else:
                uv = (0.0, 0.0)

            key = (vi, ti)  # ← KLUCZOWA POPRAWKA

            if key not in vertex_map:
                x, y, z = vertices[vi]
                unique_vertices.append((x, y, z, uv[0], uv[1]))
                vertex_map[key] = len(unique_vertices) - 1

            indices.append(vertex_map[key])



    # Zapis binarny
    with open(output_file, "wb") as f:
        f.write(struct.pack("<I", len(unique_vertices)))
        f.write(struct.pack("<I", len(faces)))
        for v in unique_vertices:
            f.write(struct.pack("<5f", *v))
        for i in range(0, len(indices), 3):
            f.write(struct.pack("<3I", indices[i], indices[i+1], indices[i+2]))

if __name__ == "__main__":
    obj_file = "tools/out/Untitled.obj"          # Twój plik OBJ z Blendera
    output_file = "tools/out/cube.smf"  # Twój binarny format
    vertices, uvs, faces = load_obj(obj_file)
    save_megamesh(vertices, uvs, faces, output_file)
    print(f"Plik {output_file} wygenerowany z {len(vertices)} wierzchołków i {len(faces)} trójkątów")
