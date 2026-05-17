# AgentsToZ_byCS

> **에이전트 관리 A to Z** — 로컬 개발 서버, AI 에이전트, 북마크를 한 화면에서.

[![Bun](https://img.shields.io/badge/Bun-1.x-F9F1E1?logo=bun)](https://bun.sh)
[![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB?logo=tauri)](https://tauri.app)
[![React](https://img.shields.io/badge/React-19-61DAFB?logo=react)](https://react.dev)
[![Supabase](https://img.shields.io/badge/Supabase-sync-3ECF8E?logo=supabase)](https://supabase.com)

로컬 개발 서버 포트 관리부터 Claude Code 에이전트 실행, 북마크 포털까지 — **A에서 Z까지** 개발 환경을 한 곳에서 관리합니다.  
맥 앱으로도, 웹 브라우저(`localhost:9000`)로도 동작하며, Supabase 연결 한 번으로 여러 기기가 자동 동기화됩니다.

---

## 30초 시작

```bash
# Bun 설치
curl -fsSL https://bun.sh/install | bash   # macOS
# powershell -c "irm bun.sh/install.ps1 | iex"   # Windows

# 저장소 받기 & 실행
git clone https://github.com/intenet1001-commits/AgentsToZ_byCS.git
cd portmanagement
bun install
bun run start
```

→ 브라우저에서 **http://localhost:9000** 열기

> Supabase 없이도 포트 관리 기능은 모두 사용할 수 있습니다.  
> 처음 실행 시 초기 설정 마법사가 뜹니다 — "건너뛰기"를 누르면 바로 시작.

---

## 주요 기능

| 기능 | 설명 |
|---|---|
| 포트 실행 / 중지 | `.command` 파일 또는 폴더 경로만으로 서버 기동. 강제 재실행 지원 |
| 실시간 상태 | 10초 자동 폴링으로 외부에서 켠 서버도 감지 |
| Claude 에이전트 통합 | `--bg` bypass, `--resume` TUI, Agents 전역 뷰 (macOS, cmux 필요) |
| 워크트리 관리 | 터미널형 패널에서 git 워크트리 직접 관리 |
| 북마크 포털 | 자주 쓰는 링크·폴더 카테고리 관리 + Vercel 외부 배포 가능 |
| 다기기 동기화 | Supabase Push / Pull — 기기별 `device_id` 독립 관리 |
| AI 이름 생성 | Claude Code로 프로젝트명·카테고리 자동 추천 |

---

## 어떤 경로로 시작할까요?

| 나의 상황 | 시작 방법 |
|---|---|
| 처음 써본다 | 위 30초 시작 → 마법사 "건너뛰기" |
| 여러 기기에서 쓰고 싶다 | 30초 시작 → 마법사 "처음 사용" → Supabase 무료 계정 |
| 다른 기기에 추가 연결 | 30초 시작 → 마법사 "추가 기기 연결" |
| 포크해서 직접 관리 | [AGENTS.md](AGENTS.md) → Fork 체크리스트 참고 |

---

## 기술 문서

개발, 기여, AI 에이전트가 이 앱을 다룰 때 필요한 상세 문서는 **[AGENTS.md](AGENTS.md)** 를 보세요.

- API 엔드포인트, 데이터 구조, Supabase 스키마
- 빌드 시스템, 개발 명령어
- 설치 마법사 플로우, Tauri ACL 권한
- 트러블슈팅

---

© 2025 CS & Company. All rights reserved.
