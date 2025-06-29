import librosa
import sys
import json

def analyze_audio(file_path):
    try:
        y, sr = librosa.load(file_path)

        # Extract BPM
        tempo, _ = librosa.beat.beat_track(y=y, sr=sr)

        # Extract Key
        # Using harmonic-percussive source separation for better key detection
        y_harmonic, y_percussive = librosa.effects.hpss(y)
        chroma = librosa.feature.chroma_cqt(y=y_harmonic, sr=sr)
        key = librosa.key.key_to_notes(librosa.feature.tonnetz(y=y_harmonic, sr=sr).mean(axis=1))

        return {
            "bpm": float(tempo),
            "key": key[0] # Taking the first suggested key
        }
    except Exception as e:
        return {"error": str(e)}

if __name__ == "__main__":
    if len(sys.argv) > 1:
        audio_file_path = sys.argv[1]
        result = analyze_audio(audio_file_path)
        print(json.dumps(result))
    else:
        print(json.dumps({"error": "No audio file path provided"}))
