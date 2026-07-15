@AGENTS.md

<!-- BEGIN autonomous-sop -->
## 자율운영·보고 SOP
- 시작 시 STATE.md와 `git status`/`git log --oneline -5`를 읽고, 진행 중이던 작업을 확인 없이 이어서 완료한다. STATE.md 없으면 생성한다.
- 태스크 착수/완료마다 STATE.md의 "진행중/완료/다음"을 갱신한다.
- 완료 시 REPORT.md 상단에 [YYYY-MM-DD] 요약 3줄(무엇을·왜·사용자 영향) + 후속 추천을 append 한다.
- 클라우드 표면(claude --remote, Cowork, 앱 채팅/Projects) 사용 금지. 로컬전용.
- 위험 변경(대량 삭제·스키마 변경·배포) 전 git 체크포인트 커밋을 먼저 남긴다. force-push/rebase 금지.
<!-- END autonomous-sop -->
