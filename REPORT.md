# REPORT

## [2026-07-22] 미지원 charset 패닉 → java.io.UnsupportedEncodingException
- 무엇을: `String.getBytes(charset)`/`new String(byte[], charset)`/`InputStreamReader.read()`의
  `unimplemented!()` 패닉을 `java.io.UnsupportedEncodingException`(신설, IOException 하위) throw 로
  전환하고, String↔Reader 의 지원 charset 목록을 공용 `charset::Charset` 으로 일원화했다.
- 왜: charset 이름은 자바 코드가 넘기는 완전한 사용자 입력인데 미지원 이름 한 줄에 호스트
  프로세스가 죽었다. Reader 쪽은 ISO-8859-1 조차 못 받는 String 쪽과의 불일치도 있었다.
- 사용자 영향: `"hi".getBytes("UTF-16")` 류가 이제 try/catch 로 잡히는 자바 예외가 되고,
  `file.encoding=ISO-8859-1` 후 InputStreamReader 도 정상 동작한다. 부수 교정:
  `System.setProperty` 반환 시그니처를 JDK 규격(`...)Ljava/lang/String;`)으로 수정,
  `Throwable.getMessage()` 신설.
- 후속 추천: ① `Charset` 공용화를 계기로 UTF-16/Shift_JIS 등 실제 인코딩 추가는 별건 티켓으로.
  ② InputStreamReader 가 read 마다 스트림 디코더를 새로 만들어 버퍼 경계의 multibyte 부분
    시퀀스가 유실될 수 있는 기존 문제(EUC-KR)가 남아 있다 — 별건 조사 권장.
