# Display & Sound Profile Manager

Tauri 기반의 다중 디스플레이 및 사운드 프로필 관리 애플리케이션

## 기능

- 💻 다중 디스플레이 구성 저장 및 관리
- 🔊 오디오 입출력 장치 프로필 관리
- 🚀 시스템 트레이에서 빠른 프로필 전환
- 🎨 모던한 UI/UX
- 🔒 안전한 시스템 리소스 접근

## 필요 사항

### 개발 환경
- Node.js 16.x 이상
- Rust 1.70 이상
- 운영체제별 빌드 도구

### Windows
- Visual Studio 2022 Build Tools
- WebView2 (Windows 10/11에 기본 포함)

### macOS
- Xcode Command Line Tools
- Homebrew (선택사항)

### Linux
- `libwebkit2gtk-4.0-dev`
- `build-essential`
- `curl`
- `wget`
- `libssl-dev`
- `libgtk-3-dev`
- `libayatana-appindicator3-dev`
- `librsvg2-dev`

## 설치 방법

### 1. Rust 설치
```bash
# Windows/macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 프로젝트 클론
```bash
git clone https://github.com/yourusername/display-sound-manager.git
cd display-sound-manager
```

### 3. 의존성 설치
```bash
# 프론트엔드 의존성
npm install

# Tauri CLI 설치 (전역)
npm install -g @tauri-apps/cli
```

### 4. 개발 서버 실행
```bash
npm run tauri dev
```

### 5. 프로덕션 빌드
```bash
npm run tauri build
```

## 플랫폼별 추가 설정

### Windows
1. **nircmd** 다운로드 (오디오 제어용)
   - https://www.nirsoft.net/utils/nircmd.html
   - `C:\Windows\System32`에 복사

### macOS
1. **displayplacer** 설치 (디스플레이 제어용)
   ```bash
   brew tap jakehilborn/jakehilborn
   brew install displayplacer
   ```

2. **SwitchAudioSource** 설치 (오디오 제어용)
   ```bash
   brew install switchaudio-osx
   ```

### Linux
1. **xrandr** (대부분 기본 설치됨)
2. **PulseAudio** 도구
   ```bash
   sudo apt-get install pulseaudio-utils
   ```

## 사용 방법

1. 앱을 실행하면 시스템 트레이에 아이콘이 나타납니다
2. 현재 디스플레이와 오디오 설정을 프로필로 저장할 수 있습니다
3. 저장된 프로필을 클릭하여 즉시 적용할 수 있습니다
4. 시스템 트레이 메뉴에서도 빠르게 프로필을 전환할 수 있습니다

## 프로젝트 구조

```
display-sound-manager/
├── src/                    # React 프론트엔드
│   ├── App.tsx            # 메인 컴포넌트
│   ├── App.css            # 스타일
│   └── main.tsx           # 엔트리 포인트
├── src-tauri/             # Rust 백엔드
│   ├── src/
│   │   └── main.rs        # Tauri 메인 로직
│   ├── Cargo.toml         # Rust 의존성
│   └── tauri.conf.json    # Tauri 설정
├── package.json           # Node.js 의존성
└── vite.config.ts         # Vite 설정
```

## 라이선스

MIT License
  