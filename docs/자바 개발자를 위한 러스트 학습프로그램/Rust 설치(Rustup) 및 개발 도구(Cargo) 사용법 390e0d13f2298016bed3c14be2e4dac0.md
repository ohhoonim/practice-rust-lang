# Rust 설치(Rustup) 및 개발 도구(Cargo) 사용법

## 요약 목록

- **Rustup 설치**: Rust 개발 환경(컴파일러, 툴체인 등)을 관리하는 공식 설치 프로그램 사용
- **Cargo 필수 명령어**: 프로젝트 생성(`new`), 빌드(`build`), 실행(`run`), 체크(`check`)
- **환경 확인 및 업데이트**: `rustc --version`으로 확인, `rustup update`로 최신화

## 1. Rust 설치 (Rustup)

### OS별 설치 방법

- **Linux / macOS**: 터미널에서 아래 명령어를 실행합니다.
    
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    
- **Windows**: 공식 웹사이트에서 `rustup-init.exe` 파일을 다운로드하여 실행합니다. (Visual Studio C++ 빌드 도구 필요)

### 설치 확인 및 업데이트

설치가 완료되면 터미널을 다시 열고 아래 명령어로 버전을 확인합니다.

```bash
rustc --version
cargo --version
```

주기적으로 최신 버전으로 업데이트하려면 다음 명령어를 사용합니다.

```bash
rustup update
```

## 2. 개발 도구 (Cargo) 사용법

Cargo는 Rust의 빌드 시스템이자 패키지 매니저입니다.

### 프로젝트 생성

새로운 바이너리(실행 파일) 프로젝트를 생성합니다.

```bash
cargo new hello_rust
cd hello_rust
```

### 프로젝트 빌드 및 실행

- **디버그 빌드**: 소스 코드를 컴파일하여 `target/debug/` 디렉토리에 실행 파일을 생성합니다.
    
    ```bash
    cargo build
    ```
    
- **실행**: 컴파일과 실행을 한 번에 처리합니다. 코드 변경 사항이 있으면 자동으로 재빌드됩니다.
    
    ```bash
    cargo run
    ```
    
- **코드 검사**: 실행 파일을 생성하지 않고, 코드에 컴파일 에러가 없는지만 빠르게 확인합니다.
    
    ```bash
    cargo check
    ```
    
- **릴리즈 빌드**: 코드를 최적화하여 배포용 실행 파일을 `target/release/` 디렉토리에 생성합니다.
    
    ```bash
    cargo build --release
    ```