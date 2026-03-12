import { exec } from "child_process";
import { promisify } from "util";
import path from "path";

const execAsync = promisify(exec);

// Path to the waysted binary. Assuming it's in the workspace target directory for development.
// Or we can assume it's in the system PATH.
// Given we're in the workspace, we'll try to use the one in target/debug/waysted or just 'waysted'
const PROJECT_ROOT = path.resolve(process.cwd(), "..");
const CLI_PATH = path.join(PROJECT_ROOT, "target", "debug", "waysted");

export interface AppScreentime {
  id: number;
  app_name: string;
  duration: number; // in ms
  percentage: number;
}

export interface ScreenTimeInstance {
  id: number;
  title: string;
  app_name: string;
  duration: number; // in ms
  start_timestamp: number;
  end_timestamp: number;
}

export interface AppGroup {
  app_name: string;
  duration: number;
  instances: {
    title: string;
    app_name: string;
    duration: number;
  }[];
}

async function runCli<T>(args: string[]): Promise<T> {
  try {
    // We'll try to use the binary in target/debug first, then fall back to 'waysted' in PATH
    const cmd = `${CLI_PATH} ${args.join(" ")}`;
    try {
      const { stdout } = await execAsync(cmd, { maxBuffer: 5 * 1024 * 1024 });
      return JSON.parse(stdout);
    } catch {
      // Fallback to system PATH
      const { stdout } = await execAsync(`waysted ${args.join(" ")}`, {
        maxBuffer: 5 * 1024 * 1024,
      });
      return JSON.parse(stdout);
    }
  } catch (error) {
    console.error("CLI Error:", error);
    throw new Error(
      "Failed to execute waysted CLI. Make sure it is built and in your PATH.",
    );
  }
}

export async function getScreentime(date: string): Promise<AppScreentime[]> {
  // date format: YYYY-MM-DD
  return runCli<AppScreentime[]>(["screentime", date, "--json"]);
}

export async function getLogs(date: string): Promise<ScreenTimeInstance[]> {
  return runCli<ScreenTimeInstance[]>(["screentime", date, "--json", "--logs"]);
}

export async function getTitleBreakdown(date: string): Promise<AppGroup[]> {
  return runCli<AppGroup[]>(["screentime", date, "--json", "--titles"]);
}
