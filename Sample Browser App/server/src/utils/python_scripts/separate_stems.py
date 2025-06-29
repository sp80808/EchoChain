import spleeter
import sys
import json
from spleeter.separator import Separator

def separate_stems(file_path, output_path):
    try:
        separator = Separator('spleeter:2stems') # You can choose 2stems, 4stems, or 5stems
        separator.separate_to_file(file_path, output_path)
        return {"success": True, "output_path": output_path}
    except Exception as e:
        return {"error": str(e)}

if __name__ == "__main__":
    if len(sys.argv) > 2:
        audio_file_path = sys.argv[1]
        output_dir = sys.argv[2]
        result = separate_stems(audio_file_path, output_dir)
        print(json.dumps(result))
    else:
        print(json.dumps({"error": "Missing audio file path or output directory"}))
