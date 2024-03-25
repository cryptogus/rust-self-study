# Cargo
러스트 빌드 시스템 및 패키지 매니저 Go 에서 mod와 같은 느낌이다.

이 도구는 코드 빌드나, 코드 작성에 필요한 외부 라이브러리를 다운로드할 때나, 라이브러리를 제작할 때 겪는 귀찮은 일들을 상당수 줄여주는 편리한 도구 - https://doc.rust-kr.org/ch01-03-hello-cargo.html

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
