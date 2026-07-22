# STATE

## 진행중
- (없음)

## 완료
- [rustjava-runtime-time-todo-impl] RuntimeImpl 시간 API `todo!()` 3건 제거(now/sleep/yield) +
  test_utils `r#yield` 구현 + tokio `time` 피처 추가 + 회귀 잠금 픽스처(`test_data/TimeApi`).
  브랜치 `runtime-time-impl`, PR #2 게이트② 대기.
- [rustjava-classfile-parse-error-propagation] 클래스파일 파싱 실패를 패닉 대신
  `java.lang.ClassFormatError` 로 전파(절단/매직 불일치/미지원 상수풀 태그 구분).
  브랜치 `classfile-parse-error-propagation`, PR #3 게이트② 대기.
- [rustjava-tracing-attributes-pin-removal] `#[tracing::instrument]` 1건을 수동 span 으로 대체,
  `tracing-attributes` 상한 핀 제거(tracing 0.1.41→0.1.44 언프리즈), wasm32 clippy CI 커버리지
  교정. 브랜치 `tracing-attributes-pin-removal`, PR 게이트② 대기.
- [rustjava-unsupported-charset-exception] 미지원 charset `unimplemented!()` 패닉 3지점을
  `java.io.UnsupportedEncodingException`(신설) throw 로 전환, String↔InputStreamReader 지원
  charset 을 공용 `charset::Charset` 으로 일치(ISO-8859-1/US-ASCII 가 Reader 에서도 동작).
  부수: `System.setProperty` 반환 시그니처 JDK 규격화(Object→String, jvm 부트스트랩 포함),
  `Throwable.getMessage()` 신설, 픽스처 `test_data/UnsupportedCharset`. 브랜치
  `unsupported-charset-exception`, PR 게이트② 대기.

## 다음
- PR approve 후 머지, 브랜치 정리(`gh pr merge --delete-branch` → `git branch -D` → `git fetch --prune`)
- ★네 PR 모두 STATE.md/REPORT.md 를 추가하므로 나중에 머지되는 쪽마다 add/add 충돌 예상 —
  선행 PR 머지 후 후행 브랜치에 `git merge main` 하고 최신(superset) 내용 채택으로 해소.
- (범위 밖 잔여) `jvm_rust/src/interpreter.rs:629` `todo!()` (invokedynamic) — 별건 티켓 필요
- (신규 발견) javac 21 산출 익명 내부 클래스(.class)가 "Malformed class file" 로 파싱 실패 —
  원인 미조사(태그 15~18 아님). 별건 티켓 필요.
- (신규 발견) InputStreamReader 가 read 마다 스트림 디코더를 새로 생성 — EUC-KR 등 multibyte
  가 버퍼 경계에 걸리면 부분 시퀀스 유실 가능(기존 문제, 이번 범위 밖). 별건 티켓 권장.
