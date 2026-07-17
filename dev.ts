#!/usr/bin/env bun

/**
 * 개발 서버 러너 — api-server(3001) + vite(9000)를 자식 프로세스로 함께 실행.
 *
 * 기존 `bun api-server.ts & vite` 방식은 종료 시 api-server가 고아 프로세스로
 * 남아 포트 3001이 누수되는 문제가 있었음. 이 러너는:
 * 1. 시작 전 포트 3001의 기존 리스너를 정리 (darwin: lsof)
 * 2. 두 서버를 자식으로 spawn (stdio inherit)
 * 3. 종료/SIGINT/SIGTERM 시 두 자식을 모두 kill
 * 4. vite의 exit code로 종료
 */

const API_PORT = Number(process.env.API_PORT) || 3001;
const VITE_PORT = Number(process.env.PORT) || 9000;

// 시작 전: API_PORT/VITE_PORT(override 반영된 실제 타겟 포트)의 기존 리스너 정리 (macOS만 — win32는 스킵)
// vite는 strictPort라 대상 포트가 점유되면 즉시 실패 — 고아 프로세스가 쌓이는 것 방지
// PORT/API_PORT가 override된 경우(워크트리 실행 등)에도 API_PORT/VITE_PORT는 override 값을 담고 있으므로
// 아래 루프는 항상 "실제로 이번 세션이 쓰려는 포트"만 정리한다 — 다른 세션의 리스너는 건드리지 않음
if (process.platform === "darwin") {
  for (const port of [API_PORT, VITE_PORT]) {
    try {
      const lsof = Bun.spawnSync(["lsof", "-ti:" + port]);
      const pids = lsof.stdout.toString().trim().split("\n").filter(Boolean);
      for (const pid of pids) {
        try {
          process.kill(parseInt(pid, 10), "SIGKILL");
          console.log(`[dev] killed stale listener on :${port} (pid ${pid})`);
        } catch {}
      }
    } catch {}
  }
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

// 어느 한쪽이라도 먼저 종료되면 나머지도 함께 정리 — 반쪽만 살아있는 페어가 남지 않도록
const [exitCode, who] = await Promise.race([
  vite.exited.then((c) => [c, "vite"] as const),
  apiServer.exited.then((c) => [c, "api-server"] as const),
]);
console.log(`[dev] ${who} exited (code ${exitCode}) — shutting down the other`);
shutdown(exitCode);
