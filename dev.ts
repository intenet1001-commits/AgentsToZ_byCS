#!/usr/bin/env bun

/**
 * 개발 서버 러너 — api-server(3001) + vite(5173)를 자식 프로세스로 함께 실행.
 *
 * 기존 `bun api-server.ts & vite` 방식은 종료 시 api-server가 고아 프로세스로
 * 남아 포트 3001이 누수되는 문제가 있었음. 이 러너는:
 * 1. 시작 전 포트 3001의 기존 리스너를 정리 (darwin: lsof)
 * 2. 두 서버를 자식으로 spawn (stdio inherit)
 * 3. 종료/SIGINT/SIGTERM 시 두 자식을 모두 kill
 * 4. vite의 exit code로 종료
 */

const API_PORT = 3001;

// 시작 전: 포트 3001 기존 리스너 정리 (macOS만 — win32는 스킵)
if (process.platform === "darwin") {
  try {
    const lsof = Bun.spawnSync(["lsof", "-ti:" + API_PORT]);
    const pids = lsof.stdout.toString().trim().split("\n").filter(Boolean);
    for (const pid of pids) {
      try {
        process.kill(parseInt(pid, 10), "SIGKILL");
        console.log(`[dev] killed stale listener on :${API_PORT} (pid ${pid})`);
      } catch {}
    }
  } catch {}
}

const apiServer = Bun.spawn(["bun", "api-server.ts"], {
  cwd: import.meta.dir,
  stdio: ["inherit", "inherit", "inherit"],
});

const vite = Bun.spawn(["./node_modules/.bin/vite"], {
  cwd: import.meta.dir,
  stdio: ["inherit", "inherit", "inherit"],
});

let shuttingDown = false;
function shutdown(code?: number) {
  if (shuttingDown) return;
  shuttingDown = true;
  try { apiServer.kill(); } catch {}
  try { vite.kill(); } catch {}
  if (code !== undefined) process.exit(code);
}

process.on("SIGINT", () => shutdown(130));
process.on("SIGTERM", () => shutdown(143));
process.on("exit", () => shutdown());

// vite 종료 시 api-server도 함께 종료하고 vite의 exit code로 종료
const viteExitCode = await vite.exited;
shutdown(viteExitCode);
