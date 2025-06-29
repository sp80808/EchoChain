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
          if (result.error) {
            console.error(`Python script reported an error: ${result.error}`);
            reject(new Error(`Audio analysis failed: ${result.error}`));
          } else {
            resolve(result);
          }
        } catch (e: any) {
          console.error(`Failed to parse Python script output: ${e.message}. Output: ${stdout}`);
          reject(new Error(`Failed to parse Python script output: ${e.message}`));
        }
      } else {
        console.error(`Python script exited with code ${code}. Stderr: ${stderr}`);
        reject(new Error(`Python script exited with code ${code}. Stderr: ${stderr}`));
      }
    });

    pythonProcess.on('error', (err) => {
      console.error(`Failed to start Python process: ${err.message}`);
      reject(new Error(`Failed to start Python process: ${err.message}`));
    });
  });
};
