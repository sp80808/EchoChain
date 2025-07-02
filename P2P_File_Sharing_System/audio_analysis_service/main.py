from fastapi import FastAPI, UploadFile, File
from fastapi.responses import JSONResponse
import aubio
import numpy as np
import tempfile
import os

app = FastAPI()

@app.post('/analyze')
async def analyze_audio(file: UploadFile = File(...)):
    try:
        # Create a temporary file to save the uploaded audio
        with tempfile.NamedTemporaryFile(delete=False, suffix=".wav") as tmp_file:
            tmp_file.write(await file.read())
            tmp_file_path = tmp_file.name

        # Initialize aubio objects
        samplerate = 44100  # You might want to infer this from the audio file
        win_s = 1024
        hop_s = 512

        s = aubio.source(tmp_file_path, samplerate, hop_s)
        samplerate = s.samplerate

        # BPM detection
        o = aubio.tempo("default", win_s, hop_s, samplerate)
        
        # Collect all BPMs found
        tempos = []
        total_frames = 0
        while True:
            samples, read = s()
            is_beat = o(samples)
            if is_beat[0]:
                tempos.append(o.get_bpm())
            total_frames += read
            if read < hop_s:
                break

        # Calculate average BPM if any tempos were detected
        bpm = np.mean(tempos) if tempos else None

        # Key detection (aubio doesn't have a direct key detection, usually done with pitch/chroma analysis)
        # For simplicity, we'll leave key as None or implement a basic placeholder.
        key = None # Placeholder for key detection

        return JSONResponse({
            'bpm': bpm,
            'key': key,
            'message': 'Audio analysis complete.'
        })
    except Exception as e:
        return JSONResponse({
            'bpm': None,
            'key': None,
            'message': f'Error during audio analysis: {e}'
        }, status_code=500)
    finally:
        # Clean up the temporary file
        if 'tmp_file_path' in locals() and os.path.exists(tmp_file_path):
            os.remove(tmp_file_path)