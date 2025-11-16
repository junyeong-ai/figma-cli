# Figma CLI

[![CI](https://github.com/junyeong-ai/figma-cli/workflows/CI/badge.svg)](https://github.com/junyeong-ai/figma-cli/actions)
[![Lint](https://github.com/junyeong-ai/figma-cli/workflows/Lint/badge.svg)](https://github.com/junyeong-ai/figma-cli/actions)
[![Rust](https://img.shields.io/badge/rust-1.91.1%2B%20(2024%20edition)-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-0.1.0-blue?style=flat-square)](https://github.com/junyeong-ai/figma-cli/releases)

> **🌐 한국어** | **[English](README.en.md)**

---

> **🎨 고성능 Figma 디자인 추출 CLI**
>
> - 🚀 **병렬 처리** (50개 동시 요청)
> - 💾 **멀티레벨 캐시** (메모리 + 디스크)
> - 🔍 **스트리밍 파싱** (대용량 파일 메모리 효율적 처리)
> - 🛠️ **6개 명령어** (추출, 이미지, 검사, 인증, 설정)

---

## ⚡ 빠른 시작 (1분)

```bash
# 1. 설치
git clone https://github.com/junyeong-ai/figma-cli
cd figma-cli
cargo build --release

# 2. 전역 설치 (선택사항)
./scripts/install.sh

# 3. 토큰 설정
export FIGMA_TOKEN="figd_..."
# 또는
figma-cli auth login

# 4. 사용 시작! 🎉
figma-cli extract <FILE_KEY>
figma-cli images <FILE_KEY> --node-ids 123:456
```

**Tip**: Figma 토큰은 [Settings](https://www.figma.com/settings)에서 발급받을 수 있습니다.

---

## 🎯 주요 기능

### 디자인 추출
```bash
# 전체 파일 추출
figma-cli extract ABC123XYZ456789012345678

# URL에서 추출
figma-cli extract "https://www.figma.com/file/ABC123XYZ456789012345678/Design"

# 깊이 제한으로 추출 (성능 최적화)
figma-cli extract <FILE_KEY> --depth 3

# 특정 페이지만 추출
figma-cli extract <FILE_KEY> --pages "Page 1,Page 2"

# JSON 출력
figma-cli extract <FILE_KEY> --output design.json
```

### 이미지 생성
```bash
# 특정 노드 이미지 추출
figma-cli images <FILE_KEY> --node-ids "123:456,789:012"

# 고해상도 이미지 (2x, 3x)
figma-cli images <FILE_KEY> --node-ids 123:456 --scale 3

# 다양한 포맷 지원
figma-cli images <FILE_KEY> --node-ids 123:456 --format svg
figma-cli images <FILE_KEY> --node-ids 123:456 --format pdf

# Base64 인코딩 (AI 에이전트용)
figma-cli images <FILE_KEY> --node-ids 123:456 --base64

# 프레임 일괄 추출
figma-cli images <FILE_KEY> --frames "Frame 1,Frame 2"
```

### 파일 검사
```bash
# 파일 구조 확인
figma-cli inspect <FILE_KEY>

# 특정 깊이까지만 검사
figma-cli inspect <FILE_KEY> --depth 2

# JSON 형식으로 출력
figma-cli inspect <FILE_KEY> --json | jq
```

### 인증 관리
```bash
# 토큰 저장
figma-cli auth login

# 토큰 확인
figma-cli auth test

# 토큰 제거
figma-cli auth logout
```

### 설정 관리
```bash
# 설정 초기화
figma-cli config init

# 설정 확인
figma-cli config show

# JSON 형식으로 확인
figma-cli config show --json

# 설정 파일 편집
figma-cli config edit
```

---

## 🏗️ 아키텍처

### Hexagonal Architecture (Ports & Adapters)
```
src/
├── core/           # 핵심 도메인 (의존성 없음)
│   ├── config.rs   # 설정 시스템
│   ├── constants.rs # 상수 정의
│   ├── errors.rs   # 에러 타입
│   └── performance.rs # 캐시 & 병렬 처리
├── client/         # API 클라이언트 (Adapter)
│   ├── figma.rs    # Figma API
│   ├── retry.rs    # 재시도 로직
│   └── auth.rs     # 인증 관리
├── service/        # 비즈니스 로직 (Port)
│   ├── orchestrator.rs # 추출 조율
│   └── traversal.rs    # 트리 순회
└── cli/            # 사용자 인터페이스
    ├── commands.rs # 명령어 핸들러
    └── args.rs     # CLI 인자
```

### 성능 최적화
- **Zero-Copy Streaming**: `Bytes`, `Arc<RawValue>` 사용
- **Multi-Layer Cache**: L1 (메모리) + L2 (디스크)
- **Parallel Processing**: Rayon 기반 work-stealing
- **Link-Time Optimization**: LTO + strip으로 4.7MB 바이너리

---

## 📦 설치

### 소스에서 빌드
```bash
git clone https://github.com/junyeong-ai/figma-cli
cd figma-cli
cargo build --release
```

### 자동 설치 스크립트
```bash
curl -fsSL https://raw.githubusercontent.com/junyeong-ai/figma-cli/main/scripts/install.sh | bash
```

### Homebrew (macOS)
```bash
brew tap junyeong-ai/figma-cli
brew install figma-cli
```

---

## ⚙️ 설정

### 환경 변수
```bash
export FIGMA_TOKEN="figd_..."
```

### 설정 파일 (`~/.config/figma-cli/config.toml`)
```toml
token = "figd_..."

[extraction]
depth = 5
max_depth = 10
styles = true
components = true
vectors = false

[http]
timeout = 30
retries = 3
retry_delay = 1000

[images]
scale = 2.0
format = "png"

[performance]
concurrent = 50
chunk_size = 100

[cache]
ttl = 24
```

---

## 🔧 개발

### 요구사항
- Rust 1.91.1+ (2024 edition)
- Cargo

### 빌드
```bash
cargo build
```

### 테스트
```bash
cargo test --all
```

### 린팅
```bash
cargo fmt --all
cargo clippy --all-targets --all-features
```

---

## 📝 라이선스

MIT OR Apache-2.0

---

## 🤝 기여

이슈와 PR은 언제나 환영합니다!

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing`)
5. Open a Pull Request

---

## 📚 관련 문서

- [CLAUDE.md](CLAUDE.md) - AI 에이전트 개발 가이드
- [Figma API Documentation](https://www.figma.com/developers/api)

---

**Made with ❤️ and Rust**
