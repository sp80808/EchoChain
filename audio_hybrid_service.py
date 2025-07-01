from fastapi import FastAPI, File, UploadFile, HTTPException
from dejavu import Dejavu
from dejavu.recognize import FileRecognizer
import pyacoustid
import chromaprint
import os
import shutil
import logging
from pydantic import BaseModel
from typing import Optional
from dotenv import load_dotenv

# Load environment variables
load_dotenv()

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Load Dejavu config from environment
config = {
    "database": {
        "type": os.getenv("DJV_DB_TYPE", "sqlite"),
        "database": os.getenv("DJV_DB_PATH", "dejavu.db")
    }
}
try:
    djv = Dejavu(config)
except Exception as e:
    logger.error(f"Failed to initialize Dejavu: {str(e)}")
    raise

app = FastAPI()

# Get AcoustID API key from environment
ACOUSTID_API_KEY = os.getenv("ACOUSTID_API_KEY")
if not ACOUSTID_API_KEY:
    logger.warning("AcoustID API key not configured")

class AudioSampleResponse(BaseModel):
    dejavu_match: dict
    acoustid_matches: list
    error: Optional[str] = None

@app.post("/api/check-sample", response_model=AudioSampleResponse)
async def check_sample(file: UploadFile = File(...)):
    if not file.filename.lower().endswith(('.wav', '.mp3', '.ogg', '.flac')):
        raise HTTPException(status_code=400, detail="Unsupported file format")

    temp_path = f"temp_{file.filename}"
    try:
        # Save uploaded file
        with open(temp_path, "wb") as buffer:
            shutil.copyfileobj(file.file, buffer)

        # 1. Check against private library (Dejavu)
        try:
            dejavu_result = djv.recognize(FileRecognizer, temp_path)
        except Exception as e:
            logger.error(f"Dejavu recognition failed: {str(e)}")
            dejavu_result = {"error": str(e)}

        # 2. Check against global database (AcoustID)
        acoustid_matches = []
        if ACOUSTID_API_KEY:
            try:
                duration, fp = chromaprint.decode_fingerprint(chromaprint.fingerprint_file(temp_path))
                acoustid_result = pyacoustid.lookup(ACOUSTID_API_KEY, fp, duration)
                acoustid_matches = [r for r in acoustid_result['results']] if 'results' in acoustid_result else []
            except Exception as e:
                logger.error(f"AcoustID lookup failed: {str(e)}")
                acoustid_matches = []
        else:
            logger.warning("AcoustID lookup skipped - no API key configured")

        return AudioSampleResponse(
            dejavu_match=dejavu_result,
            acoustid_matches=acoustid_matches
        )
    except Exception as e:
        logger.error(f"Sample check failed: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))
    finally:
        # Clean up temp file
        if os.path.exists(temp_path):
            try:
                os.remove(temp_path)
            except Exception as e:
                logger.error(f"Failed to remove temp file: {str(e)}")
