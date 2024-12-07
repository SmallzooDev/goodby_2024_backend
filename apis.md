# API 명세서

## 인증 관련 API

### 로그인
- **엔드포인트**: `POST /api/auth`
- **요청 본문**:
  ```json
  {
    "name": "string",
    "phone_number": "string"
  }
  ```
- **응답**:
  ```json
  {
    "token": "string",
    "iat": "number",
    "exp": "number"
  }
  ```

### 회원가입
- **엔드포인트**: `POST /api/register`
- **요청 본문**:
  ```json
  {
    "name": "string",
    "phone_number": "string"
  }
  ```
- **응답**:
  ```json
  {
    "id": "number",
    "name": "string",
    "role": "string"
  }
  ```

## 프로필 관련 API

### 프로필 조회
- **엔드포인트**: `GET /api/private/profile`
- **인증**: 필요
- **응답**:
  ```json
  {
    "id": "number",
    "name": "string",
    "role": "string",
    "phone_number": "string",
    "department_name": "string",
    "team": {
      "team_id": "number",
      "team_name": "string"
    },
    "ticket_count": "number",
    "tickets": [
      {
        "ticket_number": "string",
        "available": "boolean"
      }
    ]
  }
  ```

### 인증 확인
- **엔드포인트**: `GET /api/private/me`
- **인증**: 필요
- **응답**:
  ```json
  {
    "message": "string",
    "code": "number"
  }
  ```

## 티켓 관련 API

### 티켓 생성
- **엔드포인트**: `POST /api/admin/createTickets`
- **인증**: Admin 필요
- **요청 본문**:
  ```json
  {
    "users_id": ["number"]
  }
  ```
- **응답**:
  ```json
  [
    {
      "user_id": "number",
      "ticket_number": "string",
      "message": "string",
      "available": "boolean"
    }
  ]
  ```

### 티켓 랭킹 조회
- **엔드포인트**: `GET /api/admin/ticketRanking`
- **인증**: Admin 필요
- **응답**:
  ```json
  [
    {
      "user_id": "number",
      "name": "string",
      "ticket_count": "number"
    }
  ]
  ```

## 상품 관련 API

### 상품 생성
- **엔드포인트**: `POST /api/admin/create`
- **인증**: Admin 필요
- **요청 본문**:
  ```json
  {
    "name": "string",
    "stock": "number"
  }
  ```
- **응답**:
  ```json
  {
    "id": "number",
    "name": "string",
    "stock": "number"
  }
  ```

### 상품 목록 조회
- **엔드포인트**: `GET /api/list`
- **응답**:
  ```json
  [
    {
      "id": "number",
      "name": "string",
      "stock": "number"
    }
  ]
  ```

### 상품 추첨
- **엔드포인트**: `POST /api/admin/draw`
- **인증**: Admin 필요
- **요청 본문**:
  ```json
  {
    "prize_id": "number",
    "count": "number"
  }
  ```
- **응답**:
  ```json
  [
    {
      "id": "number",
      "prize_name": "string",
      "user_name": "string",
      "department_name": "string",
      "ticket_number": "string",
      "created_at": "string"
    }
  ]
  ```

### 추첨 결과 조회
- **엔드포인트**: `GET /api/draws`
- **응답**:
  ```json
  [
    {
      "id": "number",
      "prize_name": "string",
      "user_name": "string",
      "department_name": "string",
      "ticket_number": "string",
      "created_at": "string"
    }
  ]
  ```

## 팀 관련 API

### 팀 생성
- **엔드포인트**: `POST /api/admin/team/create`
- **인증**: Admin 필요
- **요청 본문**:
  ```json
  {
    "team_name": "string"
  }
  ```
- **응답**:
  ```json
  "number" // team_id
  ```

### 팀 할당
- **엔드포인트**: `POST /api/admin/team/assign`
- **인증**: Admin 필요
- **요청 본문**:
  ```json
  {
    "users_id": ["number"],
    "team_id": "number"
  }
  ```

### 팀 정보 조회
- **엔드포인트**: `GET /api/admin/team/users`
- **인증**: Admin 필요
- **응답**:
  ```json
  [
    {
      "team_id": "number",
      "team_name": "string",
      "users": [
        {
          "user_id": "number",
          "name": "string",
          "ticket_count": "number"
        }
      ]
    }
  ]
  ``` 