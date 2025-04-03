import os
import re


def transform_file_name(file_name):
    pattern = r"L(\d+)_a(\d+.\d+)_(\d+)\.(r|x)"
    match = re.match(pattern, file_name)

    if match:
        L, a, iter, extension = match.groups()
        new_name = f"isosurfaces_L{L}_ExpDist(a={float(a):.3f})_{iter}.{extension}"
        return new_name
    else:
        return None


directory = r"C:\Users\javgua\Desktop\TFM\outputs\rx_not_processed"

for file in os.listdir(directory):
    new_name = transform_file_name(file)
    # new_name = file.replace("isosurfaces_", "")

    if new_name:
        original_path = os.path.join(directory, file)
        new_path = os.path.join(directory, new_name)

        try:
            os.rename(original_path, new_path)
            print(f"File {file} renamed to {new_name}")
        except Exception as e:
            print(f"Error renaming {file}: {e}")
