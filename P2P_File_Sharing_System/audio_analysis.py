#!/usr/bin/env python3
"""
Comprehensive audio analysis module for EchoChain P2P file sharing system.
Provides musical key detection, tempo analysis, audio fingerprinting, and compatibility checking.
"""

import os
import hashlib
import numpy as np
import logging
from typing import Dict, List, Optional, Tuple, Union

# Test imports more carefully
LIBROSA_AVAILABLE = False
AUBIO_AVAILABLE = False

try:
    import librosa
    import librosa.display
    from scipy import stats
    LIBROSA_AVAILABLE = True
    print("✓ librosa successfully imported in audio_analysis module")
except ImportError as e:
    print(f"❌ librosa import failed: {e}")

try:
    import aubio
    AUBIO_AVAILABLE = True
    print("✓ aubio successfully imported")
except ImportError as e:
    print(f"ℹ️ aubio not available (optional): {e}")

logger = logging.getLogger(__name__)

class AudioAnalyzer:
    """
    Comprehensive audio analysis system for musical content.
    
    Features:
    - Musical key detection using chromagram analysis
    - Tempo (BPM) detection with beat tracking
    - Audio fingerprinting for content identification
    - Spectral feature extraction (MFCCs, spectral centroid, etc.)
    - Musical compatibility checking
    """
    
    # Krumhansl-Schmuckler key profiles for major and minor keys
    MAJOR_PROFILE = np.array([6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88])
    MINOR_PROFILE = np.array([6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17])
    
    # Note names for key detection
    NOTE_NAMES = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B']
    
    def __init__(self, sample_rate: int = 22050):
        """
        Initialize the AudioAnalyzer.
        
        Args:
            sample_rate: Target sample rate for audio processing
        """
        self.sample_rate = sample_rate
        self.logger = logging.getLogger(f"{__name__}.{self.__class__.__name__}")
        
        if not LIBROSA_AVAILABLE:
            print(f"Debug: LIBROSA_AVAILABLE = {LIBROSA_AVAILABLE}")
            try:
                import librosa
                print("✓ librosa can be imported directly in __init__")
                global LIBROSA_AVAILABLE
                LIBROSA_AVAILABLE = True
            except ImportError as e:
                raise ImportError(f"librosa is required for audio analysis. Install with: pip install librosa. Error: {e}")
    
    def analyze_audio(self, file_path: str) -> Dict:
        """
        Perform comprehensive audio analysis on an audio file.
        
        Args:
            file_path: Path to the audio file
            
        Returns:
            Dictionary containing analysis results
        """
        if not os.path.exists(file_path):
            raise FileNotFoundError(f"Audio file not found: {file_path}")
        
        try:
            # Load audio file
            y, sr = librosa.load(file_path, sr=self.sample_rate)
            
            # Perform various analyses
            analysis_result = {
                'file_path': file_path,
                'duration': len(y) / sr,
                'sample_rate': sr,
                'tempo': self._detect_tempo(y, sr),
                'key': self._detect_key(y, sr),
                'fingerprint': self._generate_fingerprint(y),
                'features': self._extract_features(y, sr),
                'energy': float(np.mean(y**2)),
                'spectral_features': self._extract_spectral_features(y, sr)
            }
            
            self.logger.info(f"Audio analysis completed for {file_path}")
            return analysis_result
            
        except Exception as e:
            self.logger.error(f"Error analyzing audio file {file_path}: {str(e)}")
            raise
    
    def _detect_tempo(self, y: np.ndarray, sr: int) -> float:
        """
        Detect the tempo (BPM) of the audio.
        
        Args:
            y: Audio time series
            sr: Sample rate
            
        Returns:
            Estimated tempo in BPM
        """
        try:
            if AUBIO_AVAILABLE:
                # Use aubio for tempo detection if available
                return self._detect_tempo_aubio(y, sr)
            else:
                # Use librosa for tempo detection
                tempo, _ = librosa.beat.beat_track(y=y, sr=sr)
                return float(tempo)
        except Exception as e:
            self.logger.warning(f"Tempo detection failed: {str(e)}")
            return 120.0  # Default tempo
    
    def _detect_tempo_aubio(self, y: np.ndarray, sr: int) -> float:
        """
        Detect tempo using aubio library.
        
        Args:
            y: Audio time series
            sr: Sample rate
            
        Returns:
            Estimated tempo in BPM
        """
        # Convert numpy array to aubio format
        hop_s = 512
        o = aubio.tempo("default", 1024, hop_s, sr)
        
        # Process audio in chunks
        tempos = []
        for i in range(0, len(y) - hop_s, hop_s):
            chunk = y[i:i + hop_s].astype(np.float32)
            is_beat = o(chunk)
            if is_beat[0]:
                tempos.append(o.get_bpm())
        
        return float(np.mean(tempos)) if tempos else 120.0
    
    def _detect_key(self, y: np.ndarray, sr: int) -> str:
        """
        Detect the musical key of the audio using chromagram analysis.
        
        Args:
            y: Audio time series
            sr: Sample rate
            
        Returns:
            Detected key as string (e.g., "C major", "A minor")
        """
        try:
            # Extract chromagram
            chroma = librosa.feature.chroma_stft(y=y, sr=sr)
            
            # Average chromagram over time
            chroma_mean = np.mean(chroma, axis=1)
            
            # Normalize to sum to 1
            chroma_mean = chroma_mean / np.sum(chroma_mean)
            
            # Calculate correlation with key profiles
            major_correlations = []
            minor_correlations = []
            
            for i in range(12):
                # Rotate profiles to test all keys
                major_profile = np.roll(self.MAJOR_PROFILE, i)
                minor_profile = np.roll(self.MINOR_PROFILE, i)
                
                # Normalize profiles
                major_profile = major_profile / np.sum(major_profile)
                minor_profile = minor_profile / np.sum(minor_profile)
                
                # Calculate correlations
                major_corr = np.corrcoef(chroma_mean, major_profile)[0, 1]
                minor_corr = np.corrcoef(chroma_mean, minor_profile)[0, 1]
                
                major_correlations.append(major_corr)
                minor_correlations.append(minor_corr)
            
            # Find best matches
            best_major_idx = np.argmax(major_correlations)
            best_minor_idx = np.argmax(minor_correlations)
            
            best_major_corr = major_correlations[best_major_idx]
            best_minor_corr = minor_correlations[best_minor_idx]
            
            # Determine if major or minor key is better
            if best_major_corr > best_minor_corr:
                return f"{self.NOTE_NAMES[best_major_idx]} major"
            else:
                return f"{self.NOTE_NAMES[best_minor_idx]} minor"
                
        except Exception as e:
            self.logger.warning(f"Key detection failed: {str(e)}")
            return "C major"  # Default key
    
    def _generate_fingerprint(self, y: np.ndarray) -> str:
        """
        Generate a unique fingerprint for the audio content.
        
        Args:
            y: Audio time series
            
        Returns:
            Hexadecimal fingerprint string
        """
        try:
            # Extract MFCCs for fingerprinting
            mfccs = librosa.feature.mfcc(y=y, sr=self.sample_rate, n_mfcc=13)
            
            # Take mean of MFCCs over time
            mfcc_mean = np.mean(mfccs, axis=1)
            
            # Quantize to create binary fingerprint
            mfcc_binary = (mfcc_mean > np.median(mfcc_mean)).astype(int)
            
            # Convert to string and hash
            fingerprint_str = ''.join(map(str, mfcc_binary))
            fingerprint_hash = hashlib.md5(fingerprint_str.encode()).hexdigest()
            
            return fingerprint_hash
            
        except Exception as e:
            self.logger.warning(f"Fingerprint generation failed: {str(e)}")
            return hashlib.md5(str(np.random.random()).encode()).hexdigest()
    
    def _extract_features(self, y: np.ndarray, sr: int) -> Dict:
        """
        Extract various audio features.
        
        Args:
            y: Audio time series
            sr: Sample rate
            
        Returns:
            Dictionary of extracted features
        """
        try:
            features = {}
            
            # MFCCs
            mfccs = librosa.feature.mfcc(y=y, sr=sr, n_mfcc=13)
            features['mfcc_mean'] = np.mean(mfccs, axis=1).tolist()
            features['mfcc_std'] = np.std(mfccs, axis=1).tolist()
            
            # Zero crossing rate
            zcr = librosa.feature.zero_crossing_rate(y)
            features['zcr_mean'] = float(np.mean(zcr))
            features['zcr_std'] = float(np.std(zcr))
            
            # RMS energy
            rms = librosa.feature.rms(y=y)
            features['rms_mean'] = float(np.mean(rms))
            features['rms_std'] = float(np.std(rms))
            
            return features
            
        except Exception as e:
            self.logger.warning(f"Feature extraction failed: {str(e)}")
            return {}
    
    def _extract_spectral_features(self, y: np.ndarray, sr: int) -> Dict:
        """
        Extract spectral features from the audio.
        
        Args:
            y: Audio time series
            sr: Sample rate
            
        Returns:
            Dictionary of spectral features
        """
        try:
            spectral_features = {}
            
            # Spectral centroid
            spectral_centroids = librosa.feature.spectral_centroid(y=y, sr=sr)
            spectral_features['centroid_mean'] = float(np.mean(spectral_centroids))
            spectral_features['centroid_std'] = float(np.std(spectral_centroids))
            
            # Spectral bandwidth
            spectral_bandwidth = librosa.feature.spectral_bandwidth(y=y, sr=sr)
            spectral_features['bandwidth_mean'] = float(np.mean(spectral_bandwidth))
            spectral_features['bandwidth_std'] = float(np.std(spectral_bandwidth))
            
            # Spectral rolloff
            spectral_rolloff = librosa.feature.spectral_rolloff(y=y, sr=sr)
            spectral_features['rolloff_mean'] = float(np.mean(spectral_rolloff))
            spectral_features['rolloff_std'] = float(np.std(spectral_rolloff))
            
            return spectral_features
            
        except Exception as e:
            self.logger.warning(f"Spectral feature extraction failed: {str(e)}")
            return {}
    
    def check_compatibility(self, analysis1: Dict, analysis2: Dict, 
                          key_tolerance: int = 2, tempo_tolerance: float = 10.0) -> Dict:
        """
        Check musical compatibility between two analyzed audio files.
        
        Args:
            analysis1: Analysis results from first file
            analysis2: Analysis results from second file
            key_tolerance: Maximum semitone difference for key compatibility
            tempo_tolerance: Maximum BPM difference for tempo compatibility
            
        Returns:
            Dictionary with compatibility information
        """
        try:
            compatibility = {
                'compatible': False,
                'key_match': False,
                'tempo_match': False,
                'key_distance': None,
                'tempo_distance': None,
                'overall_score': 0.0
            }
            
            # Check key compatibility
            key1 = analysis1.get('key', 'C major')
            key2 = analysis2.get('key', 'C major')
            
            key_distance = self._calculate_key_distance(key1, key2)
            compatibility['key_distance'] = key_distance
            compatibility['key_match'] = key_distance <= key_tolerance
            
            # Check tempo compatibility
            tempo1 = analysis1.get('tempo', 120.0)
            tempo2 = analysis2.get('tempo', 120.0)
            
            tempo_distance = abs(tempo1 - tempo2)
            compatibility['tempo_distance'] = tempo_distance
            compatibility['tempo_match'] = tempo_distance <= tempo_tolerance
            
            # Calculate overall compatibility score
            key_score = max(0, 1 - (key_distance / 6))  # Normalize to 0-1
            tempo_score = max(0, 1 - (tempo_distance / 60))  # Normalize to 0-1
            
            compatibility['overall_score'] = (key_score + tempo_score) / 2
            compatibility['compatible'] = compatibility['key_match'] and compatibility['tempo_match']
            
            return compatibility
            
        except Exception as e:
            self.logger.error(f"Compatibility check failed: {str(e)}")
            return {'compatible': False, 'error': str(e)}
    
    def _calculate_key_distance(self, key1: str, key2: str) -> int:
        """
        Calculate the distance between two musical keys in semitones.
        
        Args:
            key1: First key (e.g., "C major")
            key2: Second key (e.g., "G major")
            
        Returns:
            Distance in semitones
        """
        try:
            # Parse keys
            note1, mode1 = key1.split(' ')
            note2, mode2 = key2.split(' ')
            
            # Get note indices
            idx1 = self.NOTE_NAMES.index(note1)
            idx2 = self.NOTE_NAMES.index(note2)
            
            # Calculate semitone distance
            distance = abs(idx1 - idx2)
            distance = min(distance, 12 - distance)  # Take shorter path around circle
            
            # Add penalty for mode mismatch
            if mode1 != mode2:
                distance += 1
            
            return distance
            
        except Exception as e:
            self.logger.warning(f"Key distance calculation failed: {str(e)}")
            return 6  # Maximum distance
    
    def find_compatible_keys(self, target_key: str, tolerance: int = 2) -> List[str]:
        """
        Find all keys compatible with the target key within tolerance.
        
        Args:
            target_key: Target key (e.g., "C major")
            tolerance: Maximum semitone distance
            
        Returns:
            List of compatible keys
        """
        compatible_keys = []
        
        for note in self.NOTE_NAMES:
            for mode in ['major', 'minor']:
                test_key = f"{note} {mode}"
                distance = self._calculate_key_distance(target_key, test_key)
                
                if distance <= tolerance:
                    compatible_keys.append(test_key)
        
        return compatible_keys

# Example usage and testing
if __name__ == "__main__":
    # Set up logging
    logging.basicConfig(level=logging.INFO)
    
    analyzer = AudioAnalyzer()
    
    # Example usage (requires an actual audio file)
    print("AudioAnalyzer initialized successfully!")
    print("Compatible keys with C major (tolerance=2):", 
          analyzer.find_compatible_keys("C major", tolerance=2))