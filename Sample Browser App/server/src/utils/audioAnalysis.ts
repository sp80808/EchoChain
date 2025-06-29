import { spawn } from 'child_process';
import path from 'path';

interface AudioAnalysisResult {
  bpm?: number;
  key?: string;
  error?: string;
}

export const analyzeAudioFile = (filePath: string): Promise<AudioAnalysisResult> => {
  return new Promise((resolve, reject) => {
    const pythonScriptPath = path.join(__dirname, 'python_scripts', 'analyze_audio.py');
    const pythonProcess = spawn('python3', [pythonScriptPath, filePath]);

    let stdout = '';
    let stderr = '';

    pythonProcess.stdout.on('data', (data) => {
      stdout += data.toString();
    });

    pythonProcess.stderr.on('data', (data) => {
      stderr += data.toString();
    });

    pythonProcess.on('close', (code) => {
      if (code === 0) {
        try {
          const result: AudioAnalysisResult = JSON.parse(stdout);
          resolve(result);
        } catch (e) {
          reject(new Error(`Failed to parse Python script output: ${e.message}. Output: ${stdout}`));
        }
      } else {
        reject(new Error(`Python script exited with code ${code}. Error: ${stderr}`));
      }
    });

    pythonProcess.on('error', (err) => {
      reject(new Error(`Failed to start Python process: ${err.message}`));
    });
  });
};
