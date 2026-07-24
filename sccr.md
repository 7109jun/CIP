# Sccr v1.0 Block Specification

## ScriptCreator Script Compiler Reference

Sccr은 블록 코딩 데이터를 Sccr 코드로 변환하기 위한
전용 컴파일 언어이다.

컴파일 과정:

블록 데이터
→ 블록 코드 확인
→ Sccr 변환표 검색
→ Sccr 코드 생성
→ 컴파일
→ 게임 실행


---

# 1. 이벤트(Event)

| 블록 | 변환 코드 | 설명 |
|---|---|---|
| 파트에 닿았을 때 | `event Touch:` | 파트 접촉 감지 |
| 클릭했을 때 | `event Click:` | 클릭 감지 |
| 플레이어 입장 | `event PlayerJoin:` | 입장 감지 |
| 플레이어 퇴장 | `event Leave:` | 퇴장 감지 |
| 공격했을 때 | `event Attack:` | 공격 감지 |
| 사망했을 때 | `event Death:` | 사망 감지 |
| 상호작용했을 때 | `event Interact:` | Prompt 입력 감지 |


---

# 2. 변수

| 블록 | 변환 코드 | 설명 |
|---|---|---|
| 변수 만들기 | `var [이름] = [값]` | 변수 생성 |
| 변수 값 설정 | `[이름] = [값]` | 값 변경 |
| 변수 더하기 | `[이름] = [이름] + [값]` | 증가 |
| 변수 빼기 | `[이름] = [이름] - [값]` | 감소 |

제한:

| 종류 | 제한 |
|---|---|
| 변수 개수 | 700개 |


---

# 3. 리스트

| 블록 | 변환 코드 | 설명 |
|---|---|---|
| 리스트 만들기 | `var [이름] = []` | 리스트 생성 |
| 리스트 추가 | `[이름].add([값])` | 데이터 추가 |
| 리스트 확인 | `[이름].contains([값])` | 존재 확인 |
| 리스트 삭제 | `[이름].remove([값])` | 삭제 |

제한:

| 종류 | 제한 |
|---|---|
| 리스트 개수 | 275개 |


---

# 4. 조건

| 블록 | 변환 코드 |
|---|---|
| 만약 | `if [조건]:` |
| 아니면 | `else:` |
| 같다 | `==` |
| 다르다 | `!=` |
| 크다 | `>` |
| 작다 | `<` |
| 크거나 같다 | `>=` |
| 작거나 같다 | `<=` |


---

# 5. 반복

| 블록 | 변환 코드 |
|---|---|
| N번 반복 | `loop [횟수]:` |
| 계속 반복 | `while true:` |
| 기다리기 | `wait([시간])` |


---

# 6. 함수

| 블록 | 변환 코드 |
|---|---|
| 함수 만들기 | `function [이름]():` |
| 함수 실행 | `[이름]()` |


---

# 7. 파트

| 블록 | 변환 코드 |
|---|---|
| 파트 이동 | `part.move()` |
| 파트 색 변경 | `part.color = [색]` |
| 파트 보이기 | `part.visible = true` |
| 파트 숨기기 | `part.visible = false` |
# Sccr v1.0 Block Specification

## ScriptCreator Script Compiler Reference

Sccr은 ScriptCreator의 블록 코딩을 해석하고 컴파일하기 위한 전용 스크립트 언어이다.

블록은 아래 변환표를 기준으로 Sccr 코드로 변환된다.

## 컴파일 과정
블록 코딩
↓
블록 변환표 확인
↓
Sccr 코드 생성
↓
Sccr 컴파일
↓
게임 실행

---

# 8. 플레이어 제어 (Player)

플레이어의 능력치와 상태를 제어한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 체력 설정 | `player.health = 값` | 플레이어 체력 변경 |
| 이동속도 설정 | `player.speed = 값` | 이동속도 변경 |
| 점프력 설정 | `player.jump = 값` | 점프력 변경 |
| 중력 설정 | `player.gravity = 값` | 플레이어 중력 변경 |
| 점프 한도 설정 | `player.jumpLimit = 값` | 점프 가능 횟수 설정 |
| 순간이동 | `player.teleport(위치)` | 플레이어 이동 |
| 래그돌 활성화 | `player.ragdoll = true` | 래그돌 적용 |
| 래그돌 해제 | `player.ragdoll = false` | 래그돌 해제 |
| 리스폰 | `player.respawn()` | 플레이어 재생성 |
| 아이템 지급 | `player.giveItem(아이템)` | 아이템 지급 |

---

# 9. 월드 제어 (World)

게임 환경을 변경한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 스카이박스 변경 | `world.skybox = 이름` | 하늘 배경 변경 |
| 시간 변경 | `world.time = 값` | 시간 설정 |
| 날씨 변경 | `world.weather = 종류` | 날씨 변경 |
| 전체 중력 변경 | `world.gravity = 값` | 월드 중력 변경 |

---

# 10. GUI 제어

게임 화면 UI를 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 글자 표시 | `gui.text(내용)` | 텍스트 출력 |
| 버튼 생성 | `gui.button()` | 버튼 생성 |
| 버튼 삭제 | `gui.button.remove()` | 버튼 제거 |
| 체력바 표시 | `gui.healthbar(값)` | 체력바 표시 |
| 이미지 표시 | `gui.image(파일)` | 이미지 출력 |
| UI 표시 | `gui.visible = true` | UI 표시 |
| UI 숨기기 | `gui.visible = false` | UI 숨김 |

---

# 11. 사운드 제어 (Sound)

소리와 음악을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 소리 재생 | `sound.play(이름)` | 효과음 재생 |
| 소리 정지 | `sound.stop(이름)` | 소리 정지 |
| 배경음 재생 | `sound.music(이름)` | BGM 재생 |
| 볼륨 변경 | `sound.volume = 값` | 볼륨 설정 |
| 음높이 변경 | `sound.pitch = 값` | 피치 설정 |

---

# 12. 이펙트 제어 (Effect)

게임 효과를 생성한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 이펙트 생성 | `effect.create()` | 효과 생성 |
| 이펙트 삭제 | `effect.remove()` | 효과 제거 |
| 효과 종류 설정 | `type = 종류` | 효과 선택 |
| 생성 위치 설정 | `position = 위치` | 위치 지정 |

---

# 13. 멀티플레이 제어 (Multiplayer)

플레이어 간 기능을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 전체 메시지 | `server.message(내용)` | 전체 플레이어에게 메시지 |
| 내 이름 가져오기 | `player.name` | 자신의 이름 |
| 다른 사람 이름 | `other.name` | 다른 플레이어 이름 |
| 다른 사람 체력 | `other.health` | 다른 플레이어 체력 |
| 다른 사람 위치 | `other.position` | 다른 플레이어 위치 |
| 팀 생성 | `team.create(이름)` | 팀 생성 |
| 팀 변경 | `player.team = 팀` | 팀 변경 |
| 아이템 지급 | `player.giveItem(아이템)` | 아이템 지급 |

---

# 14. 무기 시스템 (Weapon)

전투 시스템을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 무기 생성 | `weapon 이름:` | 무기 생성 |
| 공격력 설정 | `damage = 값` | 공격력 설정 |
| 사거리 설정 | `range = 값` | 공격 거리 설정 |
| 공격 이벤트 | `event Attack:` | 공격 감지 |
| 대상 피해 | `target.health -= 값` | 대상 체력 감소 |
| 탄약 설정 | `ammo = 값` | 탄약 설정 |
| 재장전 | `weapon.reload()` | 재장전 |

---

# 15. 래그돌 시스템 (Ragdoll)

물리 기반 캐릭터 효과를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 래그돌 활성화 | `player.ragdoll = true` | 물리 상태 적용 |
| 래그돌 해제 | `player.ragdoll = false` | 원상 복구 |
| 힘 추가 | `ragdoll.force()` | 물리 힘 추가 |
| 회전 힘 추가 | `ragdoll.torque()` | 회전 힘 추가 |
| 리스폰 | `player.respawn()` | 재생성 |

---

# 16. 상호작용 시스템 (Interaction)

물건, 문, NPC 등의 상호작용을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 물건 생성 | `object 이름:` | 오브젝트 생성 |
| 상호작용 활성화 | `interact = true` | 사용 가능 |
| 상호작용 비활성화 | `interact = false` | 사용 불가 |
| Prompt 설정 | `prompt = 내용` | 안내 문구 표시 |
| 상호작용 이벤트 | `event Interact:` | 입력 감지 |
| 열기 | `open()` | 오브젝트 열기 |
| 닫기 | `close()` | 오브젝트 닫기 |
| 물건 제거 | `object.remove()` | 삭제 |
| 아이템 지급 | `player.giveItem()` | 보상 지급 |

---

# Sccr v1.0 지원 시스템

| 번호 | 시스템 |
|---|---|
| 1 | 이벤트 |
| 2 | 변수 |
| 3 | 리스트 |
| 4 | 조건문 |
| 5 | 반복문 |
| 6 | 함수 |
| 7 | 파트 제어 |
| 8 | 플레이어 제어 |
| 9 | 월드 제어 |
| 10 | GUI |
| 11 | 사운드 |
| 12 | 이펙트 |
| 13 | 멀티플레이 |
| 14 | 무기 |
| 15 | 래그돌 |
| 16 | 상호작용 |
---

# 17. 파트 제어 (Part)

게임 내부의 파트를 생성하고 조작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 파트 생성 | `part.create()` | 새로운 파트 생성 |
| 파트 삭제 | `part.remove()` | 파트 삭제 |
| 파트 이동 | `part.move(x,y,z)` | 위치 이동 |
| 파트 회전 | `part.rotate(x,y,z)` | 회전 변경 |
| 파트 크기 변경 | `part.size = 값` | 크기 변경 |
| 파트 색 변경 | `part.color = 색` | 색상 변경 |
| 파트 투명도 변경 | `part.transparency = 값` | 투명도 변경 |
| 파트 표시 | `part.visible = true` | 표시 |
| 파트 숨김 | `part.visible = false` | 숨김 |
| 충돌 설정 | `part.canCollide = true/false` | 충돌 여부 설정 |

---

# 18. 물리 제어 (Physics)

게임 물리 효과를 제어한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 힘 추가 | `physics.force()` | 물리 힘 적용 |
| 속도 설정 | `physics.velocity = 값` | 이동 속도 설정 |
| 질량 설정 | `physics.mass = 값` | 질량 변경 |
| 고정 설정 | `physics.lock = true` | 움직임 제한 |
| 고정 해제 | `physics.lock = false` | 움직임 허용 |

---

# 19. 아이템 시스템 (Item)

게임 아이템을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 아이템 생성 | `item.create(이름)` | 아이템 생성 |
| 아이템 삭제 | `item.remove()` | 아이템 삭제 |
| 아이템 확인 | `player.hasItem(이름)` | 보유 여부 확인 |
| 아이템 제거 | `player.removeItem(이름)` | 아이템 제거 |
| 아이템 개수 | `item.count` | 개수 확인 |
| 아이템 이름 | `item.name` | 이름 확인 |

---

# 20. NPC 시스템

NPC 캐릭터를 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| NPC 생성 | `npc.create(이름)` | NPC 생성 |
| NPC 대화 | `npc.say(내용)` | 대화 출력 |
| NPC 이동 | `npc.move(위치)` | 이동 |
| NPC 삭제 | `npc.remove()` | 제거 |
| NPC 상호작용 | `event NPCInteract:` | NPC 사용 |

---

# 21. 저장 시스템 (Data)

플레이어 데이터를 저장한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 데이터 저장 | `data.save(이름,값)` | 데이터 저장 |
| 데이터 불러오기 | `data.load(이름)` | 데이터 불러오기 |
| 데이터 삭제 | `data.remove(이름)` | 데이터 삭제 |
| 저장 확인 | `data.exists(이름)` | 존재 확인 |

---

# 22. 채널 시스템 (Channel)

게임 구역과 서버 공간을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 채널 생성 | `channel.create(이름)` | 새로운 구역 생성 |
| 채널 입장 | `channel.join(이름)` | 이동 |
| 채널 퇴장 | `channel.leave()` | 나가기 |
| 채널 플레이어 확인 | `channel.players` | 플레이어 목록 |
| 채널 제한 설정 | `channel.limit = 값` | 최대 인원 설정 |

---

# 23. 카메라 시스템 (Camera)

플레이어 시점을 제어한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 카메라 이동 | `camera.move(위치)` | 카메라 이동 |
| 카메라 회전 | `camera.rotate(값)` | 방향 변경 |
| 카메라 고정 | `camera.lock = true` | 시점 고정 |
| 카메라 해제 | `camera.lock = false` | 일반 시점 |
| 1인칭 설정 | `camera.firstPerson()` | 1인칭 변경 |
| 3인칭 설정 | `camera.thirdPerson()` | 3인칭 변경 |

---

# 24. 시간 시스템 (Time)

게임 시간을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 시간 가져오기 | `time.now` | 현재 시간 |
| 시간 변경 | `time.set(값)` | 시간 설정 |
| 타이머 시작 | `timer.start()` | 타이머 시작 |
| 타이머 종료 | `timer.stop()` | 타이머 종료 |
| 타이머 확인 | `timer.value` | 시간 확인 |

---

# 25. 관리자 시스템 (Admin)

게임 관리 기능을 제공한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 관리자 확인 | `player.admin` | 관리자 여부 |
| 추방 | `admin.kick(player)` | 플레이어 추방 |
| 메시지 제한 | `admin.mute(player)` | 채팅 제한 |
| 관리자 명령 | `admin.command()` | 명령 실행 |

---

# Sccr v1.0 전체 시스템 목록

| 번호 | 시스템 |
|---|---|
| 1 | 이벤트 |
| 2 | 변수 |
| 3 | 리스트 |
| 4 | 조건문 |
| 5 | 반복문 |
| 6 | 함수 |
| 7 | 파트 제어 |
| 8 | 플레이어 제어 |
| 9 | 월드 제어 |
| 10 | GUI |
| 11 | 사운드 |
| 12 | 이펙트 |
| 13 | 멀티플레이 |
| 14 | 무기 |
| 15 | 래그돌 |
| 16 | 상호작용 |
| 17 | 파트 |
| 18 | 물리 |
| 19 | 아이템 |
| 20 | NPC |
| 21 | 저장 |
| 22 | 채널 |
| 23 | 카메라 |
| 24 | 시간 |
| 25 | 관리자 |
---

# 26. 애니메이션 시스템 (Animation)

캐릭터와 오브젝트의 움직임을 제어한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 애니메이션 재생 | `animation.play(이름)` | 애니메이션 실행 |
| 애니메이션 정지 | `animation.stop(이름)` | 애니메이션 종료 |
| 애니메이션 반복 | `animation.loop = true` | 반복 실행 |
| 애니메이션 속도 | `animation.speed = 값` | 재생 속도 변경 |
| 애니메이션 불러오기 | `animation.load(파일)` | 애니메이션 추가 |

---

# 27. 파티클 시스템 (Particle)

시각 효과용 파티클을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 파티클 생성 | `particle.create()` | 파티클 생성 |
| 파티클 삭제 | `particle.remove()` | 파티클 제거 |
| 파티클 종류 | `particle.type = 종류` | 효과 종류 설정 |
| 파티클 크기 | `particle.size = 값` | 크기 설정 |
| 파티클 속도 | `particle.speed = 값` | 이동 속도 설정 |
| 파티클 색상 | `particle.color = 색` | 색 변경 |

---

# 28. 카운트 시스템 (Counter)

숫자 기반 게임 기능을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 카운트 생성 | `counter.create(이름)` | 카운터 생성 |
| 값 증가 | `counter.add(값)` | 증가 |
| 값 감소 | `counter.sub(값)` | 감소 |
| 값 설정 | `counter.set(값)` | 값 변경 |
| 값 확인 | `counter.value` | 현재 값 확인 |

---

# 29. 업적 시스템 (Achievement)

플레이어 업적을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 업적 생성 | `achievement.create(이름)` | 업적 생성 |
| 업적 달성 | `achievement.complete(이름)` | 달성 처리 |
| 업적 확인 | `achievement.has(이름)` | 달성 여부 확인 |
| 업적 목록 | `achievement.list` | 업적 목록 확인 |

---

# 30. 퀘스트 시스템 (Quest)

게임 목표와 임무를 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 퀘스트 생성 | `quest.create(이름)` | 퀘스트 생성 |
| 퀘스트 시작 | `quest.start(이름)` | 시작 |
| 퀘스트 완료 | `quest.complete(이름)` | 완료 |
| 퀘스트 실패 | `quest.fail(이름)` | 실패 처리 |
| 진행도 변경 | `quest.progress = 값` | 진행 상태 변경 |

---

# 31. 상점 시스템 (Shop)

아이템 판매 시스템을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 상점 생성 | `shop.create(이름)` | 상점 생성 |
| 상품 추가 | `shop.addItem(아이템)` | 상품 등록 |
| 구매 | `shop.buy(아이템)` | 구매 처리 |
| 판매 | `shop.sell(아이템)` | 판매 처리 |
| 가격 설정 | `shop.price = 값` | 가격 설정 |

---

# 32. 팀 시스템 (Team)

플레이어 팀 기능을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 팀 생성 | `team.create(이름)` | 팀 생성 |
| 팀 삭제 | `team.remove(이름)` | 팀 제거 |
| 팀 설정 | `player.team = 팀` | 팀 변경 |
| 팀 확인 | `player.team` | 현재 팀 확인 |
| 팀 인원 확인 | `team.players` | 팀원 목록 |

---

# 33. 승패 시스템 (Game Result)

게임 결과를 처리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 승리 처리 | `game.win(player)` | 승리 처리 |
| 패배 처리 | `game.lose(player)` | 패배 처리 |
| 게임 종료 | `game.end()` | 게임 종료 |
| 재시작 | `game.restart()` | 게임 초기화 |

---

# 34. 랭킹 시스템 (Ranking)

점수와 순위를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 점수 추가 | `rank.addScore(값)` | 점수 증가 |
| 점수 설정 | `rank.score = 값` | 점수 변경 |
| 순위 확인 | `rank.position` | 순위 확인 |
| 랭킹 표시 | `rank.show()` | 순위 표시 |

---

# 35. 채팅 시스템 (Chat)

플레이어 채팅을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 채팅 보내기 | `chat.send(내용)` | 메시지 전송 |
| 채팅 감지 | `event Chat:` | 채팅 이벤트 |
| 채팅 차단 | `chat.block(player)` | 차단 |
| 채팅 허용 | `chat.allow(player)` | 허용 |

---

# Sccr v1.0 확장 시스템 목록

| 번호 | 시스템 |
|---|---|
| 26 | 애니메이션 |
| 27 | 파티클 |
| 28 | 카운터 |
| 29 | 업적 |
| 30 | 퀘스트 |
| 31 | 상점 |
| 32 | 팀 |
| 33 | 승패 |
| 34 | 랭킹 |
| 35 | 채팅 |
---

# 36. 탈것 시스템 (Vehicle)

탈것과 이동 수단을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 탈것 생성 | `vehicle.create(이름)` | 탈것 생성 |
| 탑승 가능 설정 | `vehicle.mount = true` | 탑승 활성화 |
| 탑승 | `vehicle.enter(player)` | 탑승 처리 |
| 내리기 | `vehicle.exit(player)` | 하차 처리 |
| 속도 설정 | `vehicle.speed = 값` | 이동 속도 변경 |
| 탈것 제거 | `vehicle.remove()` | 삭제 |

---

# 37. 스킬 시스템 (Skill)

플레이어 스킬을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 스킬 생성 | `skill.create(이름)` | 스킬 생성 |
| 스킬 사용 | `skill.use(이름)` | 스킬 실행 |
| 쿨타임 설정 | `skill.cooldown = 값` | 재사용 시간 설정 |
| 마나 소모 | `skill.mana = 값` | 마나 사용량 설정 |
| 스킬 해제 | `skill.disable()` | 스킬 비활성화 |

---

# 38. 체력 시스템 (Health)

체력 관련 기능을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 최대 체력 설정 | `health.max = 값` | 최대 HP 변경 |
| 회복 | `health.add(값)` | 체력 증가 |
| 피해 | `health.damage(값)` | 피해 적용 |
| 사망 확인 | `health.dead` | 사망 상태 확인 |
| 체력 초기화 | `health.reset()` | 체력 복구 |

---

# 39. 물품 보관함 시스템 (Storage)

아이템 저장 공간을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 보관함 생성 | `storage.create(이름)` | 저장 공간 생성 |
| 아이템 넣기 | `storage.add(아이템)` | 저장 |
| 아이템 꺼내기 | `storage.get(아이템)` | 가져오기 |
| 저장 개수 확인 | `storage.count` | 개수 확인 |
| 보관함 삭제 | `storage.remove()` | 제거 |

---

# 40. 문 시스템 (Door)

문과 잠금 기능을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 문 생성 | `door.create()` | 문 생성 |
| 문 열기 | `door.open()` | 열기 |
| 문 닫기 | `door.close()` | 닫기 |
| 잠금 설정 | `door.lock = true` | 잠금 |
| 잠금 해제 | `door.lock = false` | 해제 |
| 열쇠 요구 | `door.key = 아이템` | 필요한 아이템 설정 |

---

# 41. 체크포인트 시스템 (Checkpoint)

플레이어 진행 위치를 저장한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 체크포인트 생성 | `checkpoint.create()` | 생성 |
| 저장 | `checkpoint.save()` | 위치 저장 |
| 이동 | `checkpoint.teleport()` | 저장 위치 이동 |
| 삭제 | `checkpoint.remove()` | 제거 |

---

# 42. 스폰 시스템 (Spawn)

플레이어와 오브젝트 생성 위치를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 스폰 생성 | `spawn.create()` | 스폰 위치 생성 |
| 스폰 위치 설정 | `spawn.position = 위치` | 위치 변경 |
| 스폰 이동 | `spawn.teleport()` | 이동 |
| 기본 스폰 설정 | `spawn.default = true` | 기본 위치 설정 |

---

# 43. 몬스터 시스템 (Monster)

적 캐릭터를 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 몬스터 생성 | `monster.create(이름)` | 몬스터 생성 |
| 공격 설정 | `monster.damage = 값` | 공격력 설정 |
| 이동 설정 | `monster.speed = 값` | 이동속도 설정 |
| 추적 | `monster.follow(player)` | 플레이어 추적 |
| 제거 | `monster.remove()` | 삭제 |
| 사망 이벤트 | `event MonsterDeath:` | 처치 감지 |

---

# 44. AI 시스템 (AI)

NPC와 몬스터 행동을 제어한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| AI 생성 | `ai.create()` | AI 생성 |
| 이동 목표 | `ai.target = 대상` | 목표 설정 |
| 따라가기 | `ai.follow()` | 추적 |
| 도망가기 | `ai.escape()` | 회피 |
| 순찰 | `ai.patrol()` | 반복 이동 |
| AI 정지 | `ai.stop()` | 행동 중지 |

---

# 45. 이벤트 연결 시스템 (Signal)

여러 기능을 연결한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 신호 생성 | `signal.create(이름)` | 신호 생성 |
| 신호 보내기 | `signal.send(이름)` | 실행 전달 |
| 신호 받기 | `event Signal:` | 감지 |
| 신호 삭제 | `signal.remove()` | 제거 |

---

# Sccr v1.0 확장 시스템 목록

| 번호 | 시스템 |
|---|---|
| 36 | 탈것 |
| 37 | 스킬 |
| 38 | 체력 |
| 39 | 물품 보관함 |
| 40 | 문 |
| 41 | 체크포인트 |
| 42 | 스폰 |
| 43 | 몬스터 |
| 44 | AI |
| 45 | 이벤트 연결 |
---

# 46. 로컬 플레이어 시스템 (Local Player)

현재 플레이어 개인 데이터를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 현재 플레이어 가져오기 | `local.player` | 자신의 플레이어 정보 |
| 현재 위치 가져오기 | `local.position` | 현재 좌표 확인 |
| 현재 장비 확인 | `local.equipment` | 장착 아이템 확인 |
| 현재 상태 확인 | `local.status` | 상태 확인 |
| 개인 메시지 | `local.message(내용)` | 자신에게만 출력 |

---

# 47. 좌표 시스템 (Position)

위치 데이터를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| X 좌표 | `position.x` | X 위치 |
| Y 좌표 | `position.y` | Y 위치 |
| Z 좌표 | `position.z` | Z 위치 |
| 위치 설정 | `position.set(x,y,z)` | 좌표 변경 |
| 위치 복사 | `position.copy()` | 위치 저장 |
| 거리 계산 | `position.distance(대상)` | 거리 확인 |

---

# 48. 영역 시스템 (Region)

특정 공간을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 영역 생성 | `region.create(이름)` | 영역 생성 |
| 영역 입장 이벤트 | `event RegionEnter:` | 진입 감지 |
| 영역 퇴장 이벤트 | `event RegionLeave:` | 나감 감지 |
| 영역 크기 설정 | `region.size = 값` | 크기 설정 |
| 영역 삭제 | `region.remove()` | 제거 |

---

# 49. 버튼 시스템 (Button)

게임용 버튼을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 버튼 생성 | `button.create(이름)` | 버튼 생성 |
| 버튼 클릭 이벤트 | `event ButtonClick:` | 클릭 감지 |
| 버튼 활성화 | `button.enabled = true` | 사용 가능 |
| 버튼 비활성화 | `button.enabled = false` | 사용 불가 |
| 버튼 삭제 | `button.remove()` | 삭제 |

---

# 50. 레버 시스템 (Lever)

스위치와 레버 기능을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 레버 생성 | `lever.create()` | 레버 생성 |
| 레버 켜기 | `lever.on()` | 활성화 |
| 레버 끄기 | `lever.off()` | 비활성화 |
| 상태 확인 | `lever.state` | 현재 상태 |
| 레버 이벤트 | `event Lever:` | 작동 감지 |

---

# 51. 문서 시스템 (Text)

게임 내 텍스트 데이터를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 텍스트 생성 | `text.create(내용)` | 텍스트 생성 |
| 텍스트 변경 | `text.value = 내용` | 내용 변경 |
| 텍스트 삭제 | `text.remove()` | 삭제 |
| 텍스트 가져오기 | `text.value` | 내용 확인 |

---

# 52. 랜덤 시스템 (Random)

랜덤 값을 생성한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 랜덤 숫자 | `random.number(최소,최대)` | 숫자 생성 |
| 랜덤 선택 | `random.choose(목록)` | 목록 중 선택 |
| 랜덤 확률 | `random.chance(값)` | 확률 검사 |

---

# 53. 확률 시스템 (Probability)

확률 기반 기능을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 확률 생성 | `chance.create(값)` | 확률 생성 |
| 성공 확인 | `chance.check()` | 성공 여부 확인 |
| 확률 변경 | `chance.value = 값` | 확률 수정 |

---

# 54. 카메라 효과 시스템 (Camera Effect)

화면 연출을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 화면 흔들기 | `camera.shake(값)` | 흔들림 효과 |
| 확대 | `camera.zoom(값)` | 확대 효과 |
| 축소 | `camera.zoomOut()` | 축소 |
| 효과 제거 | `camera.effect.remove()` | 제거 |

---

# 55. 환경 효과 시스템 (Environment)

게임 환경 효과를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 안개 설정 | `environment.fog = 값` | 안개 변경 |
| 밝기 설정 | `environment.light = 값` | 조명 변경 |
| 색상 필터 | `environment.filter = 색` | 화면 효과 |
| 환경 초기화 | `environment.reset()` | 기본값 복구 |

---

# Sccr v1.0 확장 시스템 목록

| 번호 | 시스템 |
|---|---|
| 46 | 로컬 플레이어 |
| 47 | 좌표 |
| 48 | 영역 |
| 49 | 버튼 |
| 50 | 레버 |
| 51 | 텍스트 |
| 52 | 랜덤 |
| 53 | 확률 |
| 54 | 카메라 효과 |
| 55 | 환경 효과 |
---

# 56. 폭발 시스템 (Explosion)

폭발 효과와 피해를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 폭발 생성 | `explosion.create()` | 폭발 생성 |
| 폭발 위치 설정 | `explosion.position = 위치` | 생성 위치 설정 |
| 폭발 범위 설정 | `explosion.range = 값` | 피해 범위 설정 |
| 폭발 피해 설정 | `explosion.damage = 값` | 피해량 설정 |
| 폭발 제거 | `explosion.remove()` | 폭발 삭제 |

---

# 57. 발사체 시스템 (Projectile)

총알, 마법탄 등 이동하는 물체를 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 발사체 생성 | `projectile.create(이름)` | 발사체 생성 |
| 발사 | `projectile.fire()` | 발사 |
| 속도 설정 | `projectile.speed = 값` | 이동 속도 |
| 방향 설정 | `projectile.direction = 방향` | 이동 방향 |
| 충돌 이벤트 | `event ProjectileHit:` | 충돌 감지 |
| 발사체 제거 | `projectile.remove()` | 제거 |

---

# 58. 탄환 시스템 (Bullet)

무기 탄환을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 탄환 생성 | `bullet.create()` | 탄환 생성 |
| 탄환 개수 설정 | `bullet.count = 값` | 탄환 수 설정 |
| 탄환 발사 | `bullet.fire()` | 발사 |
| 탄환 삭제 | `bullet.remove()` | 제거 |
| 탄환 충돌 | `event BulletHit:` | 충돌 감지 |

---

# 59. 장비 시스템 (Equipment)

플레이어 장비를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 장비 장착 | `equipment.equip(아이템)` | 장착 |
| 장비 해제 | `equipment.unequip()` | 해제 |
| 장비 확인 | `equipment.current` | 현재 장비 |
| 장비 슬롯 | `equipment.slot = 값` | 슬롯 설정 |
| 장비 변경 이벤트 | `event EquipmentChange:` | 변경 감지 |

---

# 60. 상태 시스템 (Status)

플레이어 상태 효과를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 상태 추가 | `status.add(이름)` | 상태 추가 |
| 상태 제거 | `status.remove(이름)` | 상태 제거 |
| 상태 확인 | `status.has(이름)` | 상태 확인 |
| 지속시간 설정 | `status.time = 값` | 유지 시간 |
| 상태 초기화 | `status.clear()` | 전체 제거 |

---

# 61. 버프 시스템 (Buff)

능력 강화 효과를 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 버프 추가 | `buff.add(이름)` | 버프 적용 |
| 버프 제거 | `buff.remove(이름)` | 제거 |
| 지속시간 | `buff.duration = 값` | 유지 시간 |
| 효과 값 | `buff.power = 값` | 강화 수치 |

---

# 62. 디버그 시스템 (Debug)

개발 테스트 기능을 제공한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 로그 출력 | `debug.log(내용)` | 개발 로그 |
| 값 확인 | `debug.print(값)` | 값 출력 |
| 오류 출력 | `debug.error(내용)` | 오류 표시 |
| 디버그 모드 | `debug.mode = true` | 테스트 활성화 |

---

# 63. 명령어 시스템 (Command)

사용자 명령 기능을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 명령 생성 | `command.create(이름)` | 명령 추가 |
| 명령 실행 이벤트 | `event Command:` | 명령 감지 |
| 명령 인자 | `command.args` | 입력 값 |
| 명령 제거 | `command.remove()` | 삭제 |

---

# 64. 권한 시스템 (Permission)

사용자 권한을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 권한 추가 | `permission.add(이름)` | 권한 부여 |
| 권한 제거 | `permission.remove(이름)` | 권한 삭제 |
| 권한 확인 | `permission.has(이름)` | 권한 검사 |
| 권한 목록 | `permission.list` | 권한 확인 |

---

# 65. 게임 설정 시스템 (Game Setting)

게임 기본 설정을 변경한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 게임 이름 설정 | `game.name = 이름` | 게임 이름 변경 |
| 최대 인원 설정 | `game.maxPlayers = 값` | 인원 제한 |
| 난이도 설정 | `game.difficulty = 값` | 난이도 변경 |
| 게임 시작 | `game.start()` | 시작 |
| 게임 종료 | `game.end()` | 종료 |

---

# Sccr v1.0 확장 시스템 목록

| 번호 | 시스템 |
|---|---|
| 56 | 폭발 |
| 57 | 발사체 |
| 58 | 탄환 |
| 59 | 장비 |
| 60 | 상태 |
| 61 | 버프 |
| 62 | 디버그 |
| 63 | 명령어 |
| 64 | 권한 |
| 65 | 게임 설정 |
---

# 66. 상자 시스템 (Chest)

보상 상자와 저장 상자를 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 상자 생성 | `chest.create()` | 상자 생성 |
| 상자 열기 | `chest.open()` | 상자 열기 |
| 상자 닫기 | `chest.close()` | 상자 닫기 |
| 아이템 넣기 | `chest.addItem(아이템)` | 아이템 추가 |
| 아이템 가져오기 | `chest.getItem(아이템)` | 아이템 획득 |
| 상자 삭제 | `chest.remove()` | 상자 제거 |

---

# 67. 열쇠 시스템 (Key)

잠금 기능에 사용하는 열쇠를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 열쇠 생성 | `key.create(이름)` | 열쇠 생성 |
| 열쇠 확인 | `key.has(이름)` | 보유 확인 |
| 열쇠 사용 | `key.use(이름)` | 사용 처리 |
| 열쇠 삭제 | `key.remove(이름)` | 제거 |

---

# 68. 거래 시스템 (Trade)

플레이어 간 거래 기능을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 거래 요청 | `trade.request(player)` | 거래 요청 |
| 거래 수락 | `trade.accept()` | 승인 |
| 거래 취소 | `trade.cancel()` | 취소 |
| 아이템 교환 | `trade.exchange()` | 교환 실행 |
| 거래 이벤트 | `event Trade:` | 거래 감지 |

---

# 69. 파티 시스템 (Party)

협동 플레이 그룹을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 파티 생성 | `party.create()` | 파티 생성 |
| 파티 초대 | `party.invite(player)` | 초대 |
| 파티 참가 | `party.join()` | 참가 |
| 파티 탈퇴 | `party.leave()` | 탈퇴 |
| 파티원 목록 | `party.members` | 목록 확인 |

---

# 70. 매치 시스템 (Match)

게임 매칭 기능을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 매치 생성 | `match.create()` | 매치 생성 |
| 참가 | `match.join()` | 참가 |
| 나가기 | `match.leave()` | 퇴장 |
| 시작 | `match.start()` | 시작 |
| 종료 | `match.end()` | 종료 |
| 참가자 확인 | `match.players` | 플레이어 목록 |

---

# 71. 라운드 시스템 (Round)

라운드 기반 게임을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 라운드 시작 | `round.start()` | 라운드 시작 |
| 라운드 종료 | `round.end()` | 라운드 종료 |
| 다음 라운드 | `round.next()` | 다음 진행 |
| 현재 라운드 | `round.current` | 번호 확인 |
| 승리 조건 | `round.winCondition` | 조건 설정 |

---

# 72. 웨이브 시스템 (Wave)

적 등장 방식의 게임을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 웨이브 시작 | `wave.start()` | 시작 |
| 웨이브 종료 | `wave.end()` | 종료 |
| 다음 웨이브 | `wave.next()` | 진행 |
| 웨이브 번호 | `wave.number` | 현재 번호 |
| 적 생성 | `wave.spawn(monster)` | 적 생성 |

---

# 73. 보스 시스템 (Boss)

보스 전투를 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 보스 생성 | `boss.create(이름)` | 보스 생성 |
| 보스 체력 | `boss.health = 값` | HP 설정 |
| 보스 스킬 | `boss.skill(이름)` | 스킬 실행 |
| 보스 상태 | `boss.status` | 상태 확인 |
| 보스 처치 이벤트 | `event BossDeath:` | 처치 감지 |

---

# 74. 대화 시스템 (Dialogue)

스토리와 NPC 대화를 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 대화 시작 | `dialogue.start()` | 대화 시작 |
| 문장 출력 | `dialogue.say(내용)` | 문장 표시 |
| 선택지 생성 | `dialogue.choice()` | 선택지 생성 |
| 선택 확인 | `dialogue.answer` | 선택 결과 |
| 대화 종료 | `dialogue.end()` | 종료 |

---

# 75. 튜토리얼 시스템 (Tutorial)

초보자 안내 기능을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 안내 표시 | `tutorial.show(내용)` | 안내 출력 |
| 단계 생성 | `tutorial.step(번호)` | 단계 설정 |
| 완료 처리 | `tutorial.complete()` | 완료 |
| 초기화 | `tutorial.reset()` | 초기화 |

---

# Sccr v1.0 확장 시스템 목록

| 번호 | 시스템 |
|---|---|
| 66 | 상자 |
| 67 | 열쇠 |
| 68 | 거래 |
| 69 | 파티 |
| 70 | 매치 |
| 71 | 라운드 |
| 72 | 웨이브 |
| 73 | 보스 |
| 74 | 대화 |
| 75 | 튜토리얼 |
---

# 76. 이동 시스템 (Movement)

플레이어와 오브젝트 이동 기능을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 이동 시작 | `movement.start()` | 이동 시작 |
| 이동 정지 | `movement.stop()` | 이동 중지 |
| 이동 방향 설정 | `movement.direction = 방향` | 이동 방향 설정 |
| 이동 속도 설정 | `movement.speed = 값` | 속도 변경 |
| 자동 이동 | `movement.auto = true` | 자동 이동 |

---

# 77. 플랫폼 시스템 (Platform)

움직이는 발판과 이동 플랫폼을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 플랫폼 생성 | `platform.create()` | 플랫폼 생성 |
| 이동 설정 | `platform.moveTo(위치)` | 이동 위치 설정 |
| 반복 이동 | `platform.loop = true` | 반복 이동 |
| 이동 속도 | `platform.speed = 값` | 속도 설정 |
| 플랫폼 삭제 | `platform.remove()` | 제거 |

---

# 78. 순간이동 시스템 (Teleport)

맵 이동 기능을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 순간이동 생성 | `teleport.create()` | 텔레포트 생성 |
| 이동 실행 | `teleport.go(위치)` | 이동 |
| 목적지 설정 | `teleport.target = 위치` | 목적지 설정 |
| 사용 제한 | `teleport.enabled = false` | 비활성화 |
| 사용 가능 | `teleport.enabled = true` | 활성화 |

---

# 79. 영역 효과 시스템 (Zone Effect)

특정 지역 효과를 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 효과 영역 생성 | `zone.create()` | 영역 생성 |
| 입장 효과 | `event ZoneEnter:` | 진입 감지 |
| 퇴장 효과 | `event ZoneLeave:` | 퇴장 감지 |
| 효과 적용 | `zone.effect(이름)` | 효과 적용 |
| 영역 제거 | `zone.remove()` | 제거 |

---

# 80. 상태 변화 시스템 (Transform)

플레이어와 오브젝트 변화를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 크기 변경 | `transform.scale = 값` | 크기 변경 |
| 위치 변경 | `transform.position = 위치` | 위치 변경 |
| 회전 변경 | `transform.rotation = 값` | 회전 변경 |
| 초기화 | `transform.reset()` | 원래 상태 복구 |

---

# 81. 복사 시스템 (Clone)

오브젝트 복제 기능을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 복제 생성 | `clone.create(대상)` | 복제 생성 |
| 위치 지정 | `clone.position = 위치` | 생성 위치 |
| 개수 설정 | `clone.count = 값` | 생성 개수 |
| 복제 삭제 | `clone.remove()` | 제거 |

---

# 82. 생성 시스템 (Spawn Object)

게임 중 오브젝트를 생성한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 오브젝트 생성 | `spawn.object(이름)` | 오브젝트 생성 |
| 위치 지정 | `spawn.position = 위치` | 위치 설정 |
| 회전 지정 | `spawn.rotation = 값` | 회전 설정 |
| 생성 제한 | `spawn.limit = 값` | 최대 생성 개수 |

---

# 83. 파괴 시스템 (Destroy)

오브젝트 제거 기능을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 파괴 실행 | `destroy.object(대상)` | 제거 |
| 파괴 이벤트 | `event Destroy:` | 파괴 감지 |
| 파괴 효과 | `destroy.effect = 효과` | 효과 설정 |
| 자동 삭제 | `destroy.timer = 값` | 시간 후 삭제 |

---

# 84. 경고 시스템 (Warning)

플레이어에게 경고를 표시한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 경고 표시 | `warning.show(내용)` | 경고 출력 |
| 경고 제거 | `warning.remove()` | 제거 |
| 위험 표시 | `warning.danger = true` | 위험 상태 |
| 경고 시간 | `warning.time = 값` | 표시 시간 |

---

# 85. 목표 시스템 (Goal)

게임 목표를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 목표 생성 | `goal.create(내용)` | 목표 생성 |
| 목표 완료 | `goal.complete()` | 완료 처리 |
| 목표 실패 | `goal.fail()` | 실패 처리 |
| 목표 확인 | `goal.current` | 현재 목표 |

---

# Sccr v1.0 확장 시스템 목록

| 번호 | 시스템 |
|---|---|
| 76 | 이동 |
| 77 | 플랫폼 |
| 78 | 순간이동 |
| 79 | 영역 효과 |
| 80 | 상태 변화 |
| 81 | 복사 |
| 82 | 생성 |
| 83 | 파괴 |
| 84 | 경고 |
| 85 | 목표 |
---

# 86. 미션 시스템 (Mission)

게임 내 목표와 임무를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 미션 생성 | `mission.create(이름)` | 미션 생성 |
| 미션 시작 | `mission.start(이름)` | 미션 시작 |
| 미션 완료 | `mission.complete(이름)` | 완료 처리 |
| 미션 실패 | `mission.fail(이름)` | 실패 처리 |
| 진행도 설정 | `mission.progress = 값` | 진행 상태 변경 |
| 미션 확인 | `mission.current` | 현재 미션 확인 |

---

# 87. 보상 시스템 (Reward)

게임 보상을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 보상 생성 | `reward.create(이름)` | 보상 생성 |
| 보상 지급 | `reward.give(player)` | 지급 |
| 경험치 지급 | `reward.exp = 값` | 경험치 추가 |
| 코인 지급 | `reward.coin = 값` | 재화 추가 |
| 보상 삭제 | `reward.remove()` | 제거 |

---

# 88. 경험치 시스템 (Experience)

레벨과 성장 시스템을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 경험치 추가 | `exp.add(값)` | 경험치 증가 |
| 레벨 설정 | `level = 값` | 레벨 변경 |
| 레벨업 | `level.up()` | 레벨 증가 |
| 현재 레벨 | `level.current` | 레벨 확인 |
| 필요 경험치 | `level.needExp` | 필요량 확인 |

---

# 89. 레벨 시스템 (Level)

플레이어 성장 시스템을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 최대 레벨 설정 | `level.max = 값` | 제한 설정 |
| 레벨 초기화 | `level.reset()` | 초기화 |
| 레벨 확인 | `level.check()` | 확인 |
| 레벨 이벤트 | `event LevelUp:` | 상승 감지 |

---

# 90. 재화 시스템 (Currency)

게임 내 돈과 재화를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 재화 생성 | `currency.create(이름)` | 재화 생성 |
| 추가 | `currency.add(값)` | 증가 |
| 감소 | `currency.remove(값)` | 감소 |
| 확인 | `currency.value` | 값 확인 |
| 설정 | `currency.set(값)` | 값 변경 |

---

# 91. 상호작용 UI 시스템 (Interaction UI)

상호작용 화면을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 안내 표시 | `interaction.text(내용)` | 문구 표시 |
| 키 표시 | `interaction.key = 키` | 입력키 설정 |
| 버튼 표시 | `interaction.button()` | 버튼 생성 |
| UI 제거 | `interaction.remove()` | 제거 |

---

# 92. 입력 시스템 (Input)

키보드와 입력을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 키 입력 이벤트 | `event Key:` | 입력 감지 |
| 키 확인 | `input.key` | 입력 확인 |
| 마우스 클릭 | `event Mouse:` | 클릭 감지 |
| 입력 차단 | `input.lock = true` | 입력 제한 |
| 입력 허용 | `input.lock = false` | 입력 허용 |

---

# 93. 모바일 입력 시스템 (Mobile)

모바일 조작을 지원한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 터치 이벤트 | `event Touch:` | 터치 감지 |
| 버튼 생성 | `mobile.button()` | 모바일 버튼 |
| 터치 위치 | `touch.position` | 위치 확인 |
| 모바일 확인 | `mobile.enabled` | 지원 여부 확인 |

---

# 94. 마우스 시스템 (Mouse)

마우스 입력을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 클릭 이벤트 | `event Click:` | 클릭 감지 |
| 마우스 위치 | `mouse.position` | 위치 확인 |
| 클릭 버튼 | `mouse.button` | 버튼 확인 |
| 커서 표시 | `mouse.visible = true` | 표시 |

---

# 95. 저장 슬롯 시스템 (Save Slot)

여러 저장 데이터를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 슬롯 생성 | `saveSlot.create(번호)` | 슬롯 생성 |
| 저장 | `saveSlot.save(번호)` | 저장 |
| 불러오기 | `saveSlot.load(번호)` | 불러오기 |
| 삭제 | `saveSlot.remove(번호)` | 삭제 |
| 목록 확인 | `saveSlot.list` | 슬롯 목록 |

---

# Sccr v1.0 확장 시스템 목록

| 번호 | 시스템 |
|---|---|
| 86 | 미션 |
| 87 | 보상 |
| 88 | 경험치 |
| 89 | 레벨 |
| 90 | 재화 |
| 91 | 상호작용 UI |
| 92 | 입력 |
| 93 | 모바일 입력 |
| 94 | 마우스 |
| 95 | 저장 슬롯 |
---

# 96. 키 설정 시스템 (Key Binding)

사용자 입력 키를 설정한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 키 등록 | `key.bind(키,기능)` | 키 연결 |
| 키 삭제 | `key.unbind(키)` | 키 제거 |
| 키 확인 | `key.check(키)` | 입력 확인 |
| 기본 키 설정 | `key.default()` | 기본값 복구 |

---

# 97. 마우스 조작 시스템 (Mouse Control)

마우스 동작을 제어한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 마우스 이동 | `mouse.move(위치)` | 커서 이동 |
| 클릭 감지 | `event MouseClick:` | 클릭 확인 |
| 휠 감지 | `event MouseWheel:` | 휠 입력 |
| 마우스 잠금 | `mouse.lock = true` | 커서 고정 |
| 마우스 해제 | `mouse.lock = false` | 커서 해제 |

---

# 98. 설정 메뉴 시스템 (Settings)

게임 설정 화면을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 설정 생성 | `settings.create()` | 설정창 생성 |
| 값 저장 | `settings.save(이름,값)` | 설정 저장 |
| 값 불러오기 | `settings.load(이름)` | 설정 불러오기 |
| 설정 초기화 | `settings.reset()` | 기본값 복구 |
| 설정 변경 이벤트 | `event SettingChange:` | 변경 감지 |

---

# 99. 언어 시스템 (Language)

다국어 기능을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 언어 설정 | `language.set(언어)` | 언어 변경 |
| 번역 불러오기 | `language.get(문장)` | 번역 가져오기 |
| 현재 언어 | `language.current` | 언어 확인 |
| 언어 목록 | `language.list` | 지원 언어 확인 |

---

# 100. 알림 시스템 (Notification)

게임 알림을 표시한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 알림 생성 | `notify.create(내용)` | 알림 생성 |
| 알림 표시 | `notify.show()` | 표시 |
| 알림 제거 | `notify.remove()` | 제거 |
| 알림 시간 | `notify.time = 값` | 표시 시간 설정 |

---

# 101. 팝업 시스템 (Popup)

화면 팝업을 제작한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 팝업 생성 | `popup.create(내용)` | 팝업 생성 |
| 확인 버튼 | `popup.confirm()` | 확인 버튼 |
| 취소 버튼 | `popup.cancel()` | 취소 버튼 |
| 팝업 닫기 | `popup.close()` | 닫기 |

---

# 102. 로딩 시스템 (Loading)

게임 로딩 화면을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 로딩 시작 | `loading.start()` | 로딩 시작 |
| 진행도 설정 | `loading.progress = 값` | 진행 표시 |
| 로딩 종료 | `loading.end()` | 종료 |
| 로딩 화면 변경 | `loading.image = 파일` | 이미지 변경 |

---

# 103. 파일 시스템 (File)

게임 데이터를 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 파일 생성 | `file.create(이름)` | 파일 생성 |
| 파일 읽기 | `file.read(이름)` | 데이터 읽기 |
| 파일 저장 | `file.write(내용)` | 저장 |
| 파일 삭제 | `file.remove(이름)` | 삭제 |

---

# 104. 네트워크 시스템 (Network)

온라인 통신 기능을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 데이터 보내기 | `network.send(내용)` | 데이터 전송 |
| 데이터 받기 | `event Network:` | 수신 이벤트 |
| 연결 확인 | `network.connected` | 연결 상태 |
| 연결 종료 | `network.disconnect()` | 연결 종료 |

---

# 105. 서버 시스템 (Server)

서버 기능을 관리한다.

| 블록 이름 | Sccr 코드 | 설명 |
|---|---|---|
| 서버 시작 | `server.start()` | 서버 시작 |
| 서버 종료 | `server.stop()` | 서버 종료 |
| 서버 인원 | `server.players` | 접속자 확인 |
| 서버 정보 | `server.info` | 정보 확인 |

---

# Sccr v1.0 확장 시스템 목록

| 번호 | 시스템 |
|---|---|
| 96 | 키 설정 |
| 97 | 마우스 조작 |
| 98 | 설정 메뉴 |
| 99 | 언어 |
| 100 | 알림 |
| 101 | 팝업 |
| 102 | 로딩 |
| 103 | 파일 |
| 104 | 네트워크 |
| 105 | 서버 |
