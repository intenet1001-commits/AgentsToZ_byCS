# 포트 관리 프로그램

> **개발자의 로컬 환경을, 어디서나 쓸 수 있게.** — Build v75

[![Bun](https://img.shields.io/badge/Bun-1.x-F9F1E1?logo=bun)](https://bun.sh)
[![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB?logo=tauri)](https://tauri.app)
[![React](https://img.shields.io/badge/React-19-61DAFB?logo=react)](https://react.dev)
[![Supabase](https://img.shields.io/badge/Supabase-sync-3ECF8E?logo=supabase)](https://supabase.com)

터미널을 열고, 포트 번호를 외우고, 어떤 서버가 켜져 있는지 확인하는 데 지치셨나요?  
포트 관리 프로그램은 로컬 개발 서버를 **한 화면에서 실행·중지·모니터링**하고,  
자주 쓰는 링크와 폴더를 **북마크로 정리해 Vercel에 배포**하여 맥, 윈도우, 스마트폰 어디서든 꺼내 씁니다.  
여러 대 기기를 쓰더라도 Supabase 한 번 연결로 **자동 동기화** — 세팅은 한 번, 편의는 영원히.

![포트 관리 화면](.github/images/app-overview.png)

---

로컬 개발 서버의 포트와 프로젝트를 관리하는 **Tauri + React 앱**입니다.  
웹 브라우저(`http://localhost:9000`)에서도 동작하며, 북마크 탭은 Vercel을 통해 외부 배포도 가능합니다.

---

## ⚡ 30초 시작 (처음이세요?)

> **Supabase 없이도 포트 관리 기능은 모두 사용할 수 있습니다.**  
> 다기기 동기화·북마크 포털이 필요할 때 Supabase를 연결하면 됩니다.

### 최소 설치 (macOS)

```bash
# 1. Bun 설치 (Node.js 대신 사용하는 빠른 JS 런타임)
curl -fsSL https://bun.sh/install | bash

# 2. 저장소 받기
git clone https://github.com/intenet1001-commits/portmanagement.git
cd portmanagement
bun install

# 3. 실행
bun run start
```

브라우저에서 **http://localhost:9000** 열기 → 완료!

### 최소 설치 (Windows)

```powershell
# PowerShell에서 실행
powershell -c "irm bun.sh/install.ps1 | iex"
git clone https://github.com/intenet1001-commits/portmanagement.git
cd portmanagement
bun install
bun run start
```

> **다음 단계**: 앱 첫 실행 시 **초기 설정 마법사**가 자동으로 뜹니다.  
> "건너뛰기"를 누르면 Supabase 없이 바로 사용할 수 있습니다.

---

## 🗺️ 어디서 시작할까요?

나에게 맞는 경로를 고르세요.

```
나는 어떤 사용자인가요?
│
├── 🆕 이 앱을 처음 써보는 사람
│   └── → 위 "30초 시작" 3개 명령어만 실행 후 http://localhost:9000 열기
│       → 마법사가 뜨면 "건너뛰기" 눌러도 바로 사용 가능
│
├── 📱 맥·윈도우·폰 여러 기기에서 쓰고 싶은 사람
│   └── → 30초 시작 먼저 실행 → 마법사 "🆕 처음 사용" 클릭
│       → Supabase 무료 계정 만들기 (약 10분)
│
├── 💻 이미 다른 기기에서 쓰고 있고 새 기기에 추가하는 사람
│   └── → 30초 시작 실행 → 마법사 "🔗 추가 기기 연결" 클릭
│
└── 🛠️ 이 앱을 본인 깃허브에 포크해서 직접 관리하고 싶은 사람
    └── → 아래 "Fork 후 5분 체크리스트" 읽기
```

---

## 📖 처음 보는 단어 빠르게 이해하기

| 단어 | 한 줄 설명 |
|---|---|
| **Bun** | Node.js보다 빠른 JavaScript 실행 도구. `bun install` = 패키지 설치, `bun run start` = 앱 실행 |
| **Supabase** | 무료 클라우드 데이터베이스. 여러 기기 간 데이터 동기화에 사용. 회원가입만 하면 됨 |
| **Supabase Anon Key** | Supabase 프로젝트의 공개 API 키. 비밀번호 아님. 앱에서 DB 읽기/쓰기에 사용 |
| **Tauri** | 웹 기술(React)로 맥/윈도우 앱을 만드는 도구. 자동으로 처리됨, 따로 설정 불필요 |
| **cmux** | Claude Code 전용 macOS 터미널 앱. 없어도 포트 관리 기능은 모두 사용 가능 |
| **포트(Port)** | 서버가 통신하는 번호. 예: 3000, 8080 — 브라우저에서 `localhost:3000`으로 접속하는 그 숫자 |
| **Fork** | GitHub에서 남의 저장소를 내 계정으로 복사하는 것. 단순 사용이면 불필요 |
| **Vercel** | 웹앱을 인터넷에 무료로 배포하는 서비스. 북마크 포털을 외부에서 열고 싶을 때만 필요 |

---

## Fork 후 5분 체크리스트

> **이 섹션은 "내 GitHub 계정에 저장소를 복사해서 직접 관리하고 싶은 분"만 필요합니다.**  
> 단순히 앱을 쓰고 싶은 분은 건너뛰고 [macOS 시작하기](#macOS-시작하기)로 이동하세요.

포크해서 본인의 저장소로 옮긴 직후, 다음 파일만 업데이트하면 그대로 사용할 수 있습니다.

1. **`src-tauri/tauri.conf.json`** — `identifier`를 본인 도메인으로 변경 (예: `com.yourname.portmanager`). 그대로 두면 macOS 서명 충돌 발생.
2. **`.env.example` → `.env`** 복사 후 값 채우기
   - `VITE_SUPABASE_URL`, `VITE_SUPABASE_ANON_KEY` — Supabase 프로젝트 API Settings에서 복사
   - `VITE_REPO_URL` — 포크한 저장소 URL (SetupWizard의 git clone / Fork 링크 자동 교체)
   - `PORTMGR_GITHUB_OWNER` / `PORTMGR_GITHUB_REPO` — 포크한 owner/repo (Windows 빌드 자동화용)
3. **`package.json`** — `homepage` / `repository.url`을 본인 저장소 URL로.
4. **`.github/workflows/build-windows.yml`** — GitHub Actions를 그대로 쓸 거면 포크한 저장소에 Secrets 재설정.

검증:

```bash
bun install
bun run start      # 로컬(데스크톱/웹) 동작 확인
bun run build:portal
# Vercel 연결 후 환경변수 등록 → vercel --prod
```

문제 발생 시 [docs/ERRORS.md](docs/ERRORS.md), 전체 동작 확인은 [docs/DOGFOOD.md](docs/DOGFOOD.md) 참고.

---

## 주요 기능

| 기능 | 설명 |
|---|---|
| 포트 실행/중지 | 실행 파일 연동, 강제 재실행 지원 |
| 실시간 상태 감지 | 포트 점유 여부 자동 감지 (10초 자동 폴링) |
| 폴더 기반 서버 실행 | `.command` 없이 `folderPath`만으로 자동 기동 (package.json / pyproject.toml / Cargo.toml 탐색) |
| 다기기 동기화 | Supabase Push/Pull — 기기별 독립 ID (`device_id`) |
| 다른 기기 데이터 보기 | 설정 → 고급 설정 → 단말 조회 → 기기 선택 → Pull |
| AI 추천 이름 | Claude Code로 프로젝트명·카테고리 자동 생성 (단일 호출 일괄 처리) |
| 북마크 포털 | 자주 쓰는 링크·폴더 카테고리 관리 + Vercel 외부 배포 |
| Google OAuth 포털 | Google 계정으로 포털 접근 제한 (Client ID만 필요) |
| **cmux 통합 (macOS)** | 메인 로우·점3개 메뉴·헤더 툴바 3곳에서 cmux 1-click 실행 |
| **Claude Agent View** | cmux에서 claude agents 전역/프로젝트별 통합 뷰 |
| **claude --bg bypass** | `--dangerously-skip-permissions`로 Claude 무중단 백그라운드 실행 |
| **claude --resume TUI** | 프로젝트 폴더에서 claude 인터랙티브 세션 재개 (TUI) |
| 작업루트 패널 | 터미널형 뷰에서 작업루트 직접 관리 |
| 실시간 로그 | 앱 내 모달에서 서버 로그 실시간 확인 (파일 재생성 자동 감지, 500줄 슬라이딩 윈도우) |
| 검색 | 이름, AI 별칭, URL, 경로 통합 검색 |

---

## Claude Agent 통합

포트 관리기는 Claude Code와 깊이 통합되어, 코딩 에이전트 워크플로를 포트 카드에서 직접 실행합니다.

> **플랫폼별 지원**: macOS는 cmux, Windows는 WSL2 + tmux 경로를 사용합니다.  
> Windows에서는 헤더의 터미널 선택기를 **`wsl`로 전환**하면 `agents` / `--bg` 버튼이 활성화됩니다.

### claude --bg (bypass 모드)

`--dangerously-skip-permissions` 옵션으로 Claude를 백그라운드에서 실행합니다. 권한 프롬프트 없이 자동화된 에이전트 작업에 적합합니다.

| 위치 | 동작 |
|---|---|
| 포트 카드 `▼` → `claude --bg` | 프로젝트 폴더에서 bypass Claude 실행 |
| 터미널뷰 `[--bg]` 버튼 | 현재 선택 포트에서 bypass Claude 실행 |
| 헤더 `[--bg]` 버튼 | HOME(`~`)에서 전역 bypass Claude 실행 |

**bypass 토글 (`bypass ON`)** 활성화 시 모든 cmux 항목이 `--dangerously-skip-permissions` 옵션으로 실행됩니다.

### claude --resume (TUI 세션 재개)

프로젝트 폴더에서 `claude --resume`으로 인터랙티브 Claude 세션을 재개합니다. cmux 내에서 TUI 모드로 열립니다.

| 위치 | 동작 |
|---|---|
| 포트 카드 `▼` → `Project Agents` | 프로젝트 폴더에서 claude --resume TUI 실행 |

### Claude Agent View (전역 Agents)

헤더 툴바의 `[Agents]` 버튼으로 `claude agents` 뷰를 엽니다.

| 버튼 | macOS | Windows (WSL 모드) |
|---|---|---|
| 헤더 `[Agents]` | cmux에서 전역 claude agents | WSL 터미널에서 claude agents |
| 포트 카드 `▼` → `Agent View` | cmux 워크스페이스 agent 뷰 | WSL에서 프로젝트 폴더 claude agents |

> **Windows**: 헤더 터미널 선택기를 `wsl`로 전환 시 `[Agents]` 버튼이 나타납니다.  
> macOS: cmux 미설치 시 설치 안내가 표시됩니다.

---

## cmux 통합 (macOS 전용)

가장 자주 쓰는 cmux 워크플로를 여러 진입점에서 1-click으로 호출합니다.

| 위치 | 동작 |
|---|---|
| 포트 카드 메인 로우 `[⚡cmux]` | 기존 cmux 워크스페이스에서 Claude 실행 |
| 포트 카드 메인 로우 `[⚡cmux ↺]` | 새 cmux 워크스페이스 생성 후 Claude 실행 |
| 포트 카드 `▼` → `cmux 터미널` | 폴더만 cmux로 열기 (Claude 미실행) |
| 포트 카드 `▼` → `claude --bg` | bypass 모드로 Claude 백그라운드 실행 |
| 포트 카드 `▼` → `Project Agents` | 프로젝트 폴더에서 claude --resume TUI |
| 헤더 툴바 `[>_ cmux]` | HOME(`~`) 디렉토리에서 cmux 워크스페이스 |
| 헤더 툴바 `[Agents]` | 전역 claude agents 뷰 (cmux 통합) |
| 헤더 툴바 `[--bg]` | HOME에서 bypass Claude 실행 |
| `▼` → `cmux (Mac 전용)` / `cmux ↺ 새창` | bypass/일반 모드 모두 지원 |

bypass 모드 토글(`bypass ON`) 활성화 시 모든 cmux 항목이 `--dangerously-skip-permissions` 옵션으로 Claude를 실행합니다.

cmux 미설치 시 자동 안내:
```bash
brew tap manaflow-ai/cmux && brew install --cask cmux
```

> **Socket Control 설정 필수**: cmux 메뉴 → Settings → Socket Control → **Allow All**  
> 또는 터미널에서: `defaults write com.cmuxterm.app socketControlMode -string "allowAll"`

> **Windows에서는 cmux 버튼이 자동으로 숨겨집니다.** Windows 플랫폼에서는 WSL2 + tmux 워크플로를 사용하세요.

---

## macOS 시작하기

### Step 1. 사전 설치

```bash
# Bun (JavaScript 런타임)
curl -fsSL https://bun.sh/install | bash

# Git (Xcode Command Line Tools)
xcode-select --install
# 또는
brew install git
```

설치 확인:
```bash
bun --version
git --version
```

> **Claude Code (AI 기능 사용 시)**
> ```bash
> npm install -g @anthropic-ai/claude-code
> claude
> ```
> Claude Code 없이도 포트 관리 기능은 모두 사용 가능합니다.

---

### Step 2. 저장소 받기

```bash
git clone https://github.com/intenet1001-commits/portmanagement.git
cd portmanagement
bun install
```

---

### Step 3. 실행

```bash
bun run start
```

API 서버(3001) + 개발 서버(9000)가 동시 시작됩니다.

**간편 실행 (더블클릭)**
```
실행.command   ← Finder에서 더블클릭
```

브라우저에서 **http://localhost:9000** 열기

---

### Step 4. 초기 설정 마법사

앱을 처음 실행하면 **초기 설정 마법사**가 자동으로 시작됩니다.

![초기 설정 마법사](.github/images/setup-wizard.png)

```
🚀 세팅 버튼 → 처음 사용 / 추가 단말 중 선택 → 단계별 안내
```

마법사가 자동으로 처리하는 항목:

| 단계 | 내용 |
|---|---|
| 1. Supabase 계정 | 가입 안내 (이미 있으면 스킵) |
| 2. CLI 설치 | Supabase CLI 설치 + 브라우저 로그인 |
| 3. 프로젝트 선택 | 기존 프로젝트 선택 또는 새 프로젝트 생성 |
| 4. 테이블 생성 | SQL 마이그레이션 자동 실행 |
| 5. Key 입력 | CLI 인증 시 URL + Anon Key 원클릭 자동 입력 |
| 6. 연결 테스트 | Supabase 연결 확인 |

> **이미 CLI 로그인 상태라면**: 마법사에서 "CLI 자동 가져오기" 버튼으로 URL + Anon Key를 한 번에 입력합니다.

---

### Step 5. 데이터 동기화 (다기기 사용 시)

| 동작 | 방법 |
|---|---|
| 이 기기 데이터 → Supabase | 프로젝트 관리 탭 → **Push** |
| Supabase → 이 기기 복원 | **Pull** |
| 다른 기기 데이터 보기 | ⚙ 설정 → 고급 설정 → 단말 조회 → 기기 선택 → 저장 → Pull |

> Pull 시 다른 기기의 경로(`folderPath`)는 비어있는 상태로 가져옵니다. Pull 완료 후 경로 설정 창이 자동으로 열립니다.

---

### 기존 기기 → 새 맥으로 이전

```bash
git clone https://github.com/intenet1001-commits/portmanagement.git
cd portmanagement
bun install
bun run start
```

앱 실행 후:
```
🚀 세팅 버튼 → 추가 단말 등록 → URL & Key 입력 (또는 CLI 자동 가져오기) → 이 기기 이름 입력 → Pull
```

---

## Windows 시작하기

### Step 1. 사전 설치

PowerShell을 **관리자 권한**으로 열고 실행:

```powershell
# Bun 설치
powershell -c "irm bun.sh/install.ps1 | iex"

# Git 설치
winget install Git.Git
```

> `winget` 명령이 없는 경우 → [트러블슈팅](#windows-트러블슈팅) 참고

새 PowerShell 창을 열고 설치 확인:
```powershell
bun --version
git --version
```

> **Claude Code (AI 기능 사용 시)**
> ```powershell
> npm install -g @anthropic-ai/claude-code
> claude
> ```

---

### Step 2. 저장소 받기

```powershell
git clone https://github.com/intenet1001-commits/portmanagement.git
cd portmanagement
bun install
```

---

### Step 3. 실행

```powershell
bun run start
```

**간편 실행 (더블클릭)**
```
start.bat   ← 탐색기에서 더블클릭
```

> 방화벽 허용 팝업이 뜨면 **허용**을 선택하세요.

브라우저에서 **http://localhost:9000** 열기

> **Windows 플랫폼 자동 감지**: cmux 관련 버튼은 Windows에서 자동으로 숨겨지고,  
> 플랫폼에 맞는 Windows 빌드 버튼이 표시됩니다.

---

### Step 4. 초기 설정 마법사

macOS와 동일하게 앱 첫 실행 시 **초기 설정 마법사**가 시작됩니다.

**Windows 전용 — Supabase CLI 사전 설치**

마법사 실행 전 CLI를 먼저 설치하면 "CLI 자동 가져오기" 기능을 사용할 수 있습니다.

```powershell
# Scoop 패키지 매니저 설치 (PowerShell 관리자 권한)
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser -Force
irm get.scoop.sh | iex

# 새 PowerShell 창을 열고 Supabase CLI 설치
scoop bucket add supabase https://github.com/supabase/scoop-bucket.git
scoop install supabase

# 확인
supabase --version
```

Scoop 없이 직접 설치:
```powershell
irm https://github.com/supabase/cli/releases/latest/download/supabase_windows_amd64.zip -OutFile supabase.zip
Expand-Archive supabase.zip -DestinationPath "$env:USERPROFILE\supabase-cli"
$env:Path += ";$env:USERPROFILE\supabase-cli"
```

> CLI 설치 후 반드시 **새 터미널 창**을 열어야 `supabase` 명령이 인식됩니다.

---

### Claude Agents / --bg (WSL2 사용 시)

WSL2가 설치되어 있으면 Windows에서도 `claude agents`와 `claude --bg`를 사용할 수 있습니다.

**1. WSL2 + Ubuntu 설치**
```powershell
# PowerShell 관리자 권한
wsl --install
# 재시작 후 Ubuntu 실행 → 사용자 이름/비밀번호 설정
```

**2. Ubuntu에서 Claude Code 설치**
```bash
# WSL Ubuntu 터미널에서
npm install -g @anthropic-ai/claude-code
claude  # 로그인
```

**3. 앱에서 WSL 모드 전환**
```
헤더 터미널 선택기 → [wsl] 클릭
→ [Agents] 버튼과 [bg] 버튼이 나타남
```

| 버튼 | 동작 |
|---|---|
| `[Agents]` | WSL Ubuntu에서 `claude agents` 실행 |
| `[bg]` 활성 → Claude 버튼 | WSL에서 `claude --bg '프로젝트명'` 백그라운드 실행 |

> WSL2 없이 `[powershell]` 모드에서는 Windows Terminal에서 `claude` 인터랙티브 세션만 사용 가능합니다.

---

### 기존 기기 → 새 Windows PC로 이전

```powershell
git clone https://github.com/intenet1001-commits/portmanagement.git
cd portmanagement
bun install
bun run start
```

앱 실행 후:
```
🚀 세팅 버튼 → 추가 단말 등록 → URL & Key 입력 → 이 기기 이름 입력 → Pull
```

---

## 초기 설정 마법사 (온보딩)

앱을 처음 사용한다면 설정 마법사에서 Supabase 연결, 기기 등록, 환경 설정을 한 번에 완료하세요.

> **포털 URL은 본인이 직접 Vercel에 배포한 후 생성됩니다.**  
> 저장소를 Fork → Vercel에 배포 → `https://your-app-name.vercel.app` 형태의 고유 URL이 발급됩니다.  
> 배포 방법은 앱 내 설정 마법사 → **북마크 포털 배포** 탭에서 단계별로 안내합니다.

로컬에서 설정 마법사 실행:
```bash
bun run start   # → http://localhost:9000/setup
```

---

## 앱 빌드 (배포 패키지 생성)

### macOS

```bash
bun run tauri:build      # .app 번들
bun run tauri:build:dmg  # DMG 배포 패키지
```

빌드 결과물:
```
~/cargo-targets/portmanager/release/bundle/macos/포트관리기.app
~/cargo-targets/portmanager/release/bundle/dmg/포트관리기_YYYY.M.D_aarch64.dmg
```

> 빌드 버전은 마지막 git 커밋 날짜 기준으로 자동 생성됩니다.  
> 오늘 날짜로 DMG를 만들려면 빌드 전에 커밋을 먼저 완료하세요.  
> 빌드 완료 후 앱 내 "DMG 출시하기" 버튼으로 Desktop에 자동 복사됩니다.

---

### Windows

**사전 준비 (PowerShell 관리자 권한):**

```powershell
# Rust 설치
winget install Rustlang.Rustup

# Visual Studio C++ Build Tools 설치
winget install Microsoft.VisualStudio.2022.BuildTools

# WebView2 런타임 (Windows 10만 필요, Windows 11은 기본 내장)
# https://developer.microsoft.com/en-us/microsoft-edge/webview2/
```

**빌드:**
```powershell
bun run tauri:build:win
```

빌드 결과물:
```
%USERPROFILE%\cargo-targets\portmanager\release\bundle\nsis\*.exe
```

> Windows 빌드는 Windows 환경에서 직접 실행해야 합니다 (크로스 컴파일 미지원).  
> `CARGO_TARGET_DIR`이 홈 디렉토리로 고정되어 시스템 경로 권한 문제를 자동 우회합니다.

---

## macOS vs Windows 비교

| 항목 | macOS | Windows |
|---|---|---|
| 간편 실행 | `실행.command` 더블클릭 | `start.bat` 더블클릭 |
| 포트 상태 감지 | `lsof` 기반 (절대경로) | `netstat` 기반 (자동 처리) |
| 프로세스 강제 종료 | `SIGKILL` | `taskkill /F` (자동 처리) |
| 실행 파일 등록 | `.command` 파일 | `.bat` 또는 `.ps1` 파일 |
| Tauri 앱 빌드 | `.app` / `.dmg` | `.exe` (NSIS) |
| 데이터 경로 | `~/Library/Application Support/...` | `%APPDATA%\...` |
| 터미널 선택기 | cmux / iTerm / Terminal | **powershell / wsl** |
| cmux 통합 | ✅ 지원 | ❌ (자동 숨김) |
| Claude 열기 | cmux / iTerm / Terminal | PowerShell 또는 WSL |
| Claude Agent View | ✅ cmux 통합 | ✅ **WSL2 모드에서 지원** |
| claude --bg bypass | ✅ 버튼 제공 | ✅ **WSL2 모드에서 지원** |

---

## 데이터 저장 위치

**macOS**
```
~/Library/Application Support/com.portmanager.portmanager/ports.json
~/Library/Application Support/com.portmanager.portmanager/logs/{portId}.log
```

**Windows**
```
%APPDATA%\com.portmanager.portmanager\ports.json
%APPDATA%\com.portmanager.portmanager\logs\{portId}.log
```

> Windows 탐색기 주소창에 `%APPDATA%\com.portmanager.portmanager` 입력으로 바로 이동 가능.

---

## 기술 스택

| 영역 | 기술 |
|---|---|
| 런타임 | Bun |
| 프론트엔드 | React 19 + TypeScript + Vite |
| 데스크탑 | Tauri 2 (Rust) |
| 스타일링 | Tailwind CSS |
| API 서버 | Bun.serve() (포트 3001) |
| DB 동기화 | Supabase |
| 인증 | Google OAuth 2.0 (포털) |
| 웹 배포 | Vercel |
| 에이전트 터미널 | cmux (macOS) / tmux (macOS·Windows) |

---

## ❓ 자주 묻는 질문 (FAQ)

**Q. Supabase 없이도 쓸 수 있나요?**  
A. 네. 포트 실행·중지·모니터링·로그 보기 등 핵심 기능은 모두 Supabase 없이 사용 가능합니다. 여러 기기에서 데이터를 공유하고 싶을 때만 Supabase가 필요합니다.

**Q. bun이 뭔가요? Node.js랑 달라요?**  
A. Bun은 Node.js보다 빠른 JavaScript 실행 환경입니다. `npm install` 대신 `bun install`, `node index.js` 대신 `bun index.js`를 씁니다. 이 앱은 Bun으로만 동작합니다.

**Q. 포트(Port)가 뭔가요?**  
A. 서버가 통신하는 번호입니다. `bun run dev`를 실행하면 보통 `localhost:3000` 또는 `localhost:5173` 같은 주소로 접속하는데, 여기서 3000, 5173이 포트 번호입니다. 이 앱은 그런 포트들을 한눈에 관리합니다.

**Q. cmux가 없으면 못 쓰나요?**  
A. 아니요. cmux는 macOS 전용 부가 기능이며, 없어도 포트 관리 기능 전체를 사용할 수 있습니다. Claude Code와 함께 쓸 때 편리한 추가 도구입니다.

**Q. 설정 마법사를 건너뛰었는데 다시 열 수 있나요?**  
A. 네. 앱 우측 상단 **⚙ 설정** 버튼 → **초기 설정** 또는 **Supabase 설정** 메뉴에서 언제든 다시 열 수 있습니다.

**Q. `bun run start`를 실행했는데 아무것도 안 보여요.**  
A. 브라우저에서 **http://localhost:9000** 을 직접 열어보세요. 앱이 자동으로 브라우저를 열지 않는 경우가 있습니다.

**Q. 포트 9000이 이미 사용 중이라는 오류가 나요.**  
A. 다른 프로그램이 9000번 포트를 쓰고 있습니다. 터미널에서 `lsof -ti:9000 | xargs kill -9` (macOS) 또는 `netstat -ano | findstr :9000` (Windows)으로 확인 후 종료하세요.

**Q. Windows에서 `bun`이 인식이 안 돼요.**  
A. 설치 후 반드시 **새 PowerShell 창**을 열어야 합니다. 현재 창에서는 PATH가 갱신되지 않습니다.

---

## Windows 트러블슈팅

### ❌ "running scripts is disabled on this system"

`bun`, `claude` 등 실행 시 스크립트 비활성화 오류가 뜨는 경우:

```powershell
# PowerShell 관리자 권한으로 실행
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

`Y` 입력 후 Enter. 새 PowerShell 창에서 다시 시도하세요.

현재 세션에서만 임시 허용:
```powershell
Set-ExecutionPolicy -ExecutionPolicy Bypass -Scope Process
```

---

### ❌ `winget` 명령을 찾을 수 없는 경우

`winget`은 Windows 10 1709+ / Windows 11에 기본 내장입니다.  
없는 경우:
- **Microsoft Store** → "앱 설치 관리자" 검색 → 업데이트/설치
- 또는 https://github.com/microsoft/winget-cli/releases 에서 `.msixbundle` 직접 설치

---

### ❌ API 서버가 시작되지 않는 경우 (PATH 문제)

`lsof`, `which` 등 시스템 명령이 실행되지 않으면 API 서버의 PATH 설정 문제일 수 있습니다.

```bash
# api-server.ts는 /usr/sbin/lsof 절대경로를 사용하므로
# 별도 PATH 설정 없이 동작합니다.
# 문제 지속 시 실행.command로 재시작:
./실행.command
```

---

### ❌ claude 명령을 찾을 수 없는 경우 (DMG 빌드 후)

Tauri 앱(DMG 빌드)은 일반 셸 PATH를 상속하지 않습니다.  
앱은 자동으로 `which claude` → `/usr/local/bin/claude` → `~/.npm-global/bin/claude` 순으로 탐색합니다.

```bash
# claude 설치 경로 확인
which claude

# 심볼릭 링크가 없으면 수동으로 추가
ln -sf $(which claude) /usr/local/bin/claude
```

---

### ❌ cmux 버튼 클릭 시 반응 없음 (Socket Control 문제)

```bash
# Socket Control을 Allow All로 변경
defaults write com.cmuxterm.app socketControlMode -string "allowAll"

# cmux 재시작
pkill -f "cmux.app/Contents/MacOS/cmux" 2>/dev/null
sleep 2
open -a cmux

# 연결 확인 (PONG이 오면 성공)
cmux ping
```

---

© 2025 CS & Company. All rights reserved.
