import struct
import os
import sys

EXTENSION_MAP = {
    '.bin': 0x01,
    '.png': 0x02,
    '.smf': 0x03,
    '.txt': 0x04,
    '.glsl': 0x04,
    '.ssd': 0x05,
    '.wav': 0x05
}

def pack_directory(directory, output_ssf, output_layout):
    files_to_pack = []
    
    for root, _, files in os.walk(directory):
        for file in files:
            ext = os.path.splitext(file)[1]
            if ext in EXTENSION_MAP:
                full_path = os.path.join(root, file)
                
                rel_path = os.path.relpath(full_path, directory)
                files_to_pack.append((rel_path, full_path, ext))
    
    with open(output_ssf, 'wb') as f_ssf, open(output_layout, 'w') as f_txt:
        f_ssf.write(b'SSF')
        f_ssf.write(struct.pack('B', 1))

        for index, (rel_path, full_path, ext) in enumerate(files_to_pack):
            type_id = EXTENSION_MAP.get(ext, 0x01)
            
            with open(full_path, 'rb') as f_in:
                data = f_in.read()
            
            f_ssf.write(struct.pack('B', type_id))
            f_ssf.write(struct.pack('I', len(data)))
            f_ssf.write(data)
            
            name_without_ext = os.path.splitext(rel_path)[0]
            enum_name = name_without_ext.replace('\\', '_').replace('/', '_')
            
            f_txt.write(f"{enum_name},\n")
            
            print(f"Spakowano: {rel_path} (Index: {index}) -> Enum: {enum_name}")

        f_ssf.write(struct.pack('B', 0xFF))
        
    print(f"\nGotowe! Pliki spakowane do: {output_ssf}")
    print(f"Enum zapisany w: {output_layout}")

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Błąd: Podaj folder do spakowania!")
        print("Użycie: python ssf.py <folder>")
        sys.exit(1)
        
    pack_directory(sys.argv[1], sys.argv[1] + '.ssf', sys.argv[1] + '_layout.txt')