from fastapi import FastAPI, UploadFile, File
from fastapi.responses import JSONResponse
import aubio
import numpy as np
import tempfile
import os

app = FastAPI()


@app.post("/analyze")
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

        # Key detection using basic pitch/chroma analysis with aubio
        # This is a simplified approach to detect the dominant key by analyzing pitch over time.
        # A more sophisticated method would involve full chroma feature extraction and correlation with key profiles.
        p = aubio.pitch("yin", win_s, hop_s, samplerate)
        pitches = []
        s = aubio.source(
            tmp_file_path, samplerate, hop_s
        )  # Re-open the source for pitch analysis
        while True:
            samples, read = s()
            pitch = p(samples)[0]
            if pitch > 0:  # Only consider valid pitch values
                pitches.append(pitch)
            if read < hop_s:
                break

        # Convert pitches to MIDI notes and find the most common note as a rough key estimate
        if pitches:
            midi_notes = [
                int(round(12.0 * np.log2(p / 440.0) + 69)) % 12
                for p in pitches
                if p > 0
            ]
            if midi_notes:
                most_common_note = max(set(midi_notes), key=midi_notes.count)
                key_names = [
                    "C",
                    "C#",
                    "D",
                    "D#",
                    "E",
                    "F",
                    "F#",
                    "G",
                    "G#",
                    "A",
                    "A#",
                    "B",
                ]
                key = key_names[most_common_note]
            else:
                key = "Unknown (no valid pitches detected)"
        else:
            key = "Unknown (no pitches detected)"

        return JSONResponse(
            {"bpm": bpm, "key": key, "message": "Audio analysis complete."}
        )
    except Exception as e:
        return JSONResponse(
            {"bpm": None, "key": None, "message": f"Error during audio analysis: {e}"},
            status_code=500,
        )
    finally:
        # Clean up the temporary file
        if "tmp_file_path" in locals() and os.path.exists(tmp_file_path):
            os.remove(tmp_file_path)
