# Cargo
모든 내용 출처는 https://doc.rust-kr.org/ch01-03-hello-cargo.html 다.  

러스트 빌드 시스템 및 패키지 매니저 Go 에서 mod와 같은 느낌이다.

이 도구는 코드 빌드나, 코드 작성에 필요한 외부 라이브러리를 다운로드할 때나, 라이브러리를 제작할 때 겪는 귀찮은 일들을 상당수 줄여주는 편리한 도구

러스트가 설치되면 따로 설치할 필요는 없는 것 같다. 그래도 확인은 해봐야지.
```bash
$ cargo --version
```

## Cargo로 프로젝트 생성

```bash
$ cargo new hello_cargo
$ cd hello_cargo
```

첫 번째 명령어는 hello_cargo라는 디렉터리를 생성합니다. 우리는 프로젝트의 이름을 hello_cargo 로 지정했고 카고는 동일한 이름의 디렉터리 안에 파일들을 생성합니다.

hello_cargo 디렉터리로 이동해 파일을 살펴보면 Cargo.toml 파일과 src 디렉터리를 확인할 수 있으며, src 디렉터리 내에는 main.rs 파일이 있는 것도 볼 수 있습니다.

그 외에도 .gitignore 파일과 함께 새 Git 저장소가 초기화됩니다. 여러분이 이미 Git 저장소가 있는 디렉터리에서 cargo new를 실행시킨다면 Git 파일들은 생성되지 않을 것입니다. 이 동작은 cargo new --vcs=git 명령을 통해 덮어쓸 수 있습니다.

## Cargo project build
https://doc.rust-kr.org/ch01-03-hello-cargo.html  
카고로 프로젝트를 빌드하고 실행하기  
위치 `rust-self-study/ch1/hello_cargo`
```bash
$ cargo build
   Compiling hello_cargo v0.1.0 (/home/hhlee/rust-self-study/ch1/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
```

이 명령어는 현재 디렉터리가 아닌 target/debug/hello_cargo (Windows에서는 target\debug\hello_cargo.exe)에 실행 파일을 생성합니다. 기본 빌드가 디버그 빌드기 때문에, 카고는 debug라는 디렉터리에 바이너리를 생성합니다. 실행 파일은 다음 명령어로 실행할 수 있습니다:

```bash
$ target/debug/hello_cargo 
Hello, world!
```

처음 cargo build 명령어를 실행하면 최상위 디렉터리에 Cargo.lock 파일이 생성될 텐데, 이 파일은 프로젝트에서 사용하는 의존성의 정확한 버전을 자동으로 기록해 두는 파일이니 여러분이 직접 수정할 필요는 없음

컴파일과 실행을 한 번에 진행하는 cargo run 명령어도 있음

```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/hello_cargo`
Hello, world!
```
카고에는 cargo check라는 명령어도 있는데, 이는 실행 파일은 생성하지 않고 작성한 소스가 문제없이 컴파일되는지만 빠르게 확인하는 명령어

```bash
$ cargo check
    Checking hello_cargo v0.1.0 (/home/hhlee/rust-self-study/ch1/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
```
코드를 작성하는 동안 여러분의 프로젝트가 컴파일되는지 지속적으로 검사하려면, cargo check 사용이 코드가 계속 컴파일되는지 확인하는 과정을 빠르게 해줄 것입니다! 러스타시안은 대부분 주기적으로 이 명령어를 실행해 코드에서 컴파일 문제가 발생하지 않는지 확인하고, 실행 파일이 필요할 경우에만 cargo build를 사용합니다.
