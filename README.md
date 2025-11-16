# Figma CLI

[![CI](https://github.com/junyeong-ai/figma-cli/workflows/CI/badge.svg)](https://github.com/junyeong-ai/figma-cli/actions)
[![Rust](https://img.shields.io/badge/rust-1.91.1%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-0.1.0-blue?style=flat-square)](https://github.com/junyeong-ai/figma-cli/releases)

> **🎨 Figma 디자인 추출 & 쿼리 CLI**

**🌐 [English](README.en.md) | 한국어**

---

## ⚡ 핵심 기능

- 🚀 **자동 캐싱** - 반복 작업 즉시 완료 (첫 실행 후 0ms)
- 🔍 **JMESPath 쿼리** - 복잡한 데이터 탐색
- 🖼️ **이미지 추출** - Base64 인코딩 지원
- 📦 **다양한 포맷** - JSON, Markdown, Text 출력
- ⚙️ **유연한 필터링** - 페이지, 프레임 패턴 매칭

---

## 🚀 빠른 시작

### 1. 설치

```bash
git clone https://github.com/junyeong-ai/figma-cli
cd figma-cli
./scripts/install.sh
```

### 2. 인증

```bash
figma-cli auth login
```

**토큰 발급**: [Figma Settings](https://www.figma.com/settings) → Personal Access Tokens

### 3. 사용

```bash
figma-cli extract <FILE_KEY>
figma-cli query <FILE_KEY> "name"
figma-cli images <FILE_KEY> --frames "123:456"
```

---

## 📖 명령어

### `extract` - 디자인 추출

```bash
# 기본 추출
figma-cli extract <FILE_KEY>

# URL 지원
figma-cli extract "https://figma.com/file/<FILE_KEY>/Design"

# 페이지 필터링
figma-cli extract <FILE_KEY> --pages "Page 1,Page 2"
figma-cli extract <FILE_KEY> --page-pattern ".*Mobile.*"

# 프레임 필터링
figma-cli extract <FILE_KEY> --frame-pattern "^Component/.*"

# 출력 포맷
figma-cli extract <FILE_KEY> --format json --output design.json
figma-cli extract <FILE_KEY> --format markdown --output design.md
figma-cli extract <FILE_KEY> --format text

# Pretty JSON
figma-cli extract <FILE_KEY> --pretty

# 이미지 포함
figma-cli extract <FILE_KEY> --with-images --image-dir ./images

# 숨겨진 노드 포함
figma-cli extract <FILE_KEY> --include-hidden
```

### `query` - JMESPath 쿼리

```bash
# 단순 필드
figma-cli query <FILE_KEY> "name"

# 배열 프로젝션
figma-cli query <FILE_KEY> "document.children[*].name"

# 필터링
figma-cli query <FILE_KEY> "document.children[?name=='Cover']"

# 복합 쿼리
figma-cli query <FILE_KEY> "{fileName: name, version: version}" --pretty

# 특정 노드 쿼리
figma-cli query <FILE_KEY> --nodes "30:71,0:1" "nodes"

# 깊이 제한
figma-cli query <FILE_KEY> "name" --depth 3
```

### `images` - 이미지 추출

```bash
# 프레임 추출
figma-cli images <FILE_KEY> --frames "123:456,789:012"

# 포맷 지정
figma-cli images <FILE_KEY> --frames "123:456" --format png
figma-cli images <FILE_KEY> --frames "123:456" --format svg
figma-cli images <FILE_KEY> --frames "123:456" --format pdf

# 스케일 조정
figma-cli images <FILE_KEY> --frames "123:456" --scale 2.0
figma-cli images <FILE_KEY> --frames "123:456" --scale 3.0

# Base64 인코딩
figma-cli images <FILE_KEY> --frames "123:456" --base64

# Pretty JSON 출력
figma-cli images <FILE_KEY> --frames "123:456" --pretty
```

### `cache` - 캐시 관리

```bash
# 통계
figma-cli cache stats

# 목록
figma-cli cache list
figma-cli cache list --json

# 삭제
figma-cli cache clear --yes
```

### `inspect` - 파일 검사

```bash
# 기본 검사
figma-cli inspect <FILE_KEY>

# 깊이 제한
figma-cli inspect <FILE_KEY> --depth 2
```

### `auth` - 인증

```bash
figma-cli auth login   # 토큰 저장
figma-cli auth test    # 토큰 확인
figma-cli auth logout  # 토큰 삭제
```

### `config` - 설정

```bash
figma-cli config init  # 설정 초기화
figma-cli config show  # 설정 확인
figma-cli config edit  # 설정 편집
```

---

## 💡 활용 사례

### AI 에이전트

```bash
# 디자인 데이터 추출
figma-cli extract <FILE_KEY> --output design.json

# 이미지 Base64 추출
figma-cli images <FILE_KEY> --frames "123:456" --base64 --output images.json

# 쿼리로 필요한 데이터만 추출
figma-cli query <FILE_KEY> "{pages: document.children[*].name, meta: {name, version}}"
```

### 디자인 분석

```bash
# 모든 페이지 이름
figma-cli query <FILE_KEY> "document.children[*].name"

# 특정 패턴 페이지 찾기
figma-cli query <FILE_KEY> "document.children[?contains(name, 'Mobile')]"

# 통계
figma-cli query <FILE_KEY> "length(document.children)"
```

---

## ⚙️ 설정

### 우선순위

1. CLI 인자 (`--format json`)
2. 환경 변수 (`FIGMA_TOKEN`)
3. 프로젝트 설정 (`./figma-cli.toml`)
4. 전역 설정 (`~/.config/figma-cli/config.toml`)

### 설정 파일

**위치**: `~/.config/figma-cli/config.toml`

```toml
token = "figd_..."

[extraction]
depth = 5
styles = true
components = true

[images]
scale = 2.0
format = "png"

[cache]
ttl = 24

[http]
timeout = 30
retries = 3
```

---

## 🎯 성능

실제 테스트 결과 (파일 키: kAP6ItdoLNNJ7HLOWMnCUf, depth=2):

| 작업 | 첫 실행 | 캐시 사용 |
|------|---------|-----------|
| Extract | 8754ms | 0ms |
| Query | ~3000ms | 22ms |

**캐시 위치**: `~/Library/Caches/figma-cli` (macOS)

---

## 🛠️ 개발

### 요구사항

- Rust 1.91.1+ (2024 edition)

### 빌드 & 테스트

```bash
cargo build --release
cargo test
cargo fmt
cargo clippy
```

---

## 📄 라이선스

MIT OR Apache-2.0

---

## 📚 문서

- [CLAUDE.md](CLAUDE.md) - AI 에이전트 개발 가이드
- [Figma API](https://www.figma.com/developers/api)

---

**Made with ❤️ and Rust**
