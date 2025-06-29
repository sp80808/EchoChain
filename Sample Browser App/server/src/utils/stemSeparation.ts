import { spawn } from 'child_process';
import path from 'path';

interface StemSeparationResult {
  success?: boolean;
  outputPath?: string;
  error?: string;
}

export const separateStems = (filePath: string, outputDir: string): Promise<StemSeparationResult> => {
  return new Promise((resolve, reject) => {
    const pythonScriptPath = path.join(__dirname, 'python_scripts', 'separate_stems.py');
    const pythonProcess = spawn('python3', [pythonScriptPath, filePath, outputDir]);

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
          const result: StemSeparationResult = JSON.parse(stdout);
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
