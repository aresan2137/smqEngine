import struct
import sys
import os
import zlib

def convert_data_to_smf(lines, smf_path, global_v, global_vt, global_vn):
    vertices = []
    indices = []
    vertex_map = {}

    for line in lines:
        parts = line.strip().split()
        if not parts or parts[0] != 'f': continue
        
        for vert in parts[1:]:
            if vert not in vertex_map:
                vertex_map[vert] = len(vertices) // 8
                p_uv_n = vert.split('/')
                
                p = int(p_uv_n[0])
                t = int(p_uv_n[1]) if len(p_uv_n) > 1 and p_uv_n[1] else 0
                n = int(p_uv_n[2]) if len(p_uv_n) > 2 and p_uv_n[2] else 0
                
                pos = global_v[p-1]
                uv = global_vt[t-1] if t > 0 else (0, 0)
                nor = global_vn[n-1] if n > 0 else (0, 0, 1)
                
                vertices.extend([*pos, *uv, *nor])
            indices.append(vertex_map[vert])

    vertex_count = len(vertices) // 8
    index_count = len(indices)
    
    if vertex_count == 0: return

    # Standardowy layout: Float3 (Pozycja), Float2 (UV), Float3 (Normalne)
    layout = [0, 1, 0] 

    # Budujemy surowy payload danych do skompresowania
    payload = bytearray()
    payload.extend(struct.pack('I', vertex_count))
    payload.extend(struct.pack('I', index_count))
    payload.extend(struct.pack('B', len(layout)))
    for fmt in layout:
        payload.extend(struct.pack('B', fmt))
    for v in vertices:
        payload.extend(struct.pack('f', v))
    for i in indices:
        payload.extend(struct.pack('I', i))

    # Kompresujemy całą geometrię (poziom 9 daje maksymalny skurcz pliku)
    compressed_payload = zlib.compress(payload, level=9)
    uncompressed_size = len(payload)

    # Zapisujemy plik .smf na dysk
    with open(smf_path, 'wb') as f:
        f.write(struct.pack('B', 0b10110000))       # 1 Bajt nagłówka formatu
        f.write(struct.pack('I', uncompressed_size)) # 4 Bajty informujące o rozmiarze po dekompresji
        f.write(compressed_payload)                  # Skompresowana zupka bajtów

    print(f"Saved (COMPRESSED): {smf_path} | Real size: {os.path.getsize(smf_path)} bytes (Raw: {uncompressed_size})")

def process_file(obj_path, output_dir):
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    global_v, global_vt, global_vn = [], [], []
    blocks = []
    current_name = "default"
    current_lines = []

    with open(obj_path, 'r') as f:
        for line in f:
            parts = line.strip().split()
            if not parts: continue
            
            if parts[0] == 'v':
                global_v.append((float(parts[1]), float(parts[2]), float(parts[3])))
            elif parts[0] == 'vt':
                global_vt.append((float(parts[1]), float(parts[2])))
            elif parts[0] == 'vn':
                global_vn.append((float(parts[1]), float(parts[2]), float(parts[3])))
            elif parts[0] == 'o':
                if current_lines:
                    blocks.append((current_name, current_lines))
                current_name = parts[1]
                current_lines = []
            elif parts[0] == 'f':
                current_lines.append(line)
        
        if current_lines:
            blocks.append((current_name, current_lines))

    for name, lines in blocks:
        convert_data_to_smf(lines, os.path.join(output_dir, f"{name}.smf"), global_v, global_vt, global_vn)

if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Wrong usage! Expected: python smf.py input.obj output_dir")
    else:
        process_file(sys.argv[1], sys.argv[2])