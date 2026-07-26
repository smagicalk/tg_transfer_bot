#!/usr/bin/env sh
set -eu

# 统一 Linux 包构建脚本：
# - 安装发行版依赖
# - 源码编译 TDLib
# - 可选执行 fmt/test/clippy
# - 构建 release 二进制并打包

install_packages() {
  . /etc/os-release
  case "$ID" in
    alpine)
      apk add --no-cache \
        bash \
        ca-certificates \
        clang \
        cmake \
        curl \
        file \
        g++ \
        gperf \
        git \
        gzip \
        linux-headers \
        make \
        musl-dev \
        openssl-dev \
        pkgconf \
        sqlite-dev \
        tar \
        zlib-dev
      ;;
    debian|ubuntu)
      export DEBIAN_FRONTEND=noninteractive
      apt-get update
      apt-get install -y --no-install-recommends \
        ca-certificates \
        clang \
        cmake \
        curl \
        file \
        g++ \
        gperf \
        git \
        gzip \
        libsqlite3-dev \
        libssl-dev \
        make \
        pkg-config \
        tar \
        zlib1g-dev
      rm -rf /var/lib/apt/lists/*
      ;;
    *)
      echo "Unsupported distro: $ID" >&2
      exit 1
      ;;
  esac

  if ! command -v cargo >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --profile minimal --default-toolchain 1.88.0
    . "$HOME/.cargo/env"
  fi

  if command -v rustup >/dev/null 2>&1; then
    rustup component add rustfmt clippy
  fi
}

copy_runtime_libs() {
  awk '
    $2 == "=>" && $3 ~ /^\// { print $3; next }
    $1 ~ /^\// { print $1; next }
  ' "$DIST_DIR/runtime-libs.txt" | sort -u | while IFS= read -r lib; do
    [ -n "$lib" ] || continue
    [ -f "$lib" ] || continue
    base="$(basename "$lib")"
    case "$base" in
      ld-linux*|ld-musl*|libc.so*|libdl.so*|libm.so*|libpthread.so*|librt.so*)
        continue
        ;;
    esac
    if [ ! -e "$DIST_DIR/bin/$base" ]; then
      cp -L "$lib" "$DIST_DIR/bin/$base"
    fi
  done
}

assert_packaged_runtime() {
  if LD_LIBRARY_PATH="$DIST_DIR/bin${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
    ldd "$DIST_DIR/bin/transfer_bot" | grep -q 'not found'; then
    echo "transfer_bot has unresolved runtime dependencies" >&2
    exit 1
  fi

  for so in "$DIST_DIR"/bin/libtdjson.so*; do
    [ -e "$so" ] || continue
    if LD_LIBRARY_PATH="$DIST_DIR/bin${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
      ldd "$so" | grep -q 'not found'; then
      echo "$(basename "$so") has unresolved runtime dependencies" >&2
      exit 1
    fi
  done
}

require_env() {
  var_name="$1"
  eval "var_value=\${$var_name:-}"
  if [ -z "$var_value" ]; then
    echo "Missing required env: $var_name" >&2
    exit 1
  fi
}

require_env CI_DISTRO_ID
require_env CI_IMAGE
require_env CI_ARTIFACT_NAME
require_env TD_GIT_REF
require_env RUN_CHECKS

WORK_ROOT="${WORK_ROOT:-/work}"
LOCAL_TDLIB_PATH="${LOCAL_TDLIB_PATH:-/opt/tdlib}"
TD_SOURCE_DIR="${TD_SOURCE_DIR:-/tmp/td}"
TD_BUILD_DIR="${TD_BUILD_DIR:-/tmp/td-build}"
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$WORK_ROOT/target/$CI_DISTRO_ID}"
PACKAGE_RETENTION_KIND="${PACKAGE_RETENTION_KIND:-full}"
PACKAGE_MODE="${PACKAGE_MODE:-package}"

install_packages

export PATH="$HOME/.cargo/bin:$PATH"
export LOCAL_TDLIB_PATH
export TD_SOURCE_DIR
export TD_BUILD_DIR
export CARGO_TARGET_DIR
export LD_LIBRARY_PATH="$LOCAL_TDLIB_PATH/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

rustc --version
cargo --version
cmake --version
g++ --version

rm -rf "$LOCAL_TDLIB_PATH" "$TD_SOURCE_DIR" "$TD_BUILD_DIR"
git clone --depth 1 https://github.com/tdlib/td.git "$TD_SOURCE_DIR"
cd "$TD_SOURCE_DIR"
if [ "$TD_GIT_REF" != "master" ]; then
  git fetch --depth 1 origin "$TD_GIT_REF" || true
  git fetch --depth 1 origin "refs/tags/$TD_GIT_REF:refs/tags/$TD_GIT_REF" || true
  git checkout "$TD_GIT_REF"
fi
TD_COMMIT="$(git rev-parse HEAD)"

cmake -S "$TD_SOURCE_DIR" -B "$TD_BUILD_DIR" \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_INSTALL_PREFIX="$LOCAL_TDLIB_PATH" \
  -DCMAKE_INSTALL_LIBDIR=lib \
  -DTD_ENABLE_LTO=OFF
cmake --build "$TD_BUILD_DIR" --target install --parallel 2
test -f "$LOCAL_TDLIB_PATH/lib/libtdjson.so"

cd "$WORK_ROOT"
if [ "$RUN_CHECKS" = "true" ]; then
  cargo fmt --all -- --check
  cargo test -p transfer_bot
  cargo clippy -p transfer_bot --all-targets --no-deps -- -D warnings
fi

cargo build -p transfer_bot --release
test -x "$CARGO_TARGET_DIR/release/transfer_bot"

# 仅验证构建时，到此即可，不再生成压缩包和校验文件。
if [ "$PACKAGE_MODE" = "build_only" ]; then
  echo "build_only completed: $CARGO_TARGET_DIR/release/transfer_bot"
  exit 0
fi

DIST_ROOT="$WORK_ROOT/dist"
DIST_DIR="$DIST_ROOT/$CI_ARTIFACT_NAME"
rm -rf "$DIST_DIR" "$DIST_ROOT/$CI_ARTIFACT_NAME.tar.gz" "$DIST_ROOT/$CI_ARTIFACT_NAME.sha256"
mkdir -p "$DIST_DIR/bin"

cp "$CARGO_TARGET_DIR/release/transfer_bot" "$DIST_DIR/bin/transfer_bot"
find "$LOCAL_TDLIB_PATH/lib" -maxdepth 1 -name 'libtdjson.so*' -exec cp -a {} "$DIST_DIR/bin/" \;
test -e "$DIST_DIR/bin/libtdjson.so"

{
  ldd "$DIST_DIR/bin/transfer_bot" || true
  for so in "$DIST_DIR"/bin/libtdjson.so*; do
    [ -e "$so" ] || continue
    ldd "$so" || true
  done
} > "$DIST_DIR/runtime-libs.txt"
copy_runtime_libs
assert_packaged_runtime

if command -v strip >/dev/null 2>&1; then
  strip "$DIST_DIR/bin/transfer_bot" || true
  find "$DIST_DIR/bin" -type f -name '*.so*' -exec strip --strip-unneeded {} \; || true
fi

cat > "$DIST_DIR/run.sh" <<'RUN_SCRIPT'
#!/usr/bin/env sh
set -eu
APP_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
export LD_LIBRARY_PATH="$APP_DIR/bin${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
exec "$APP_DIR/bin/transfer_bot" "$@"
RUN_SCRIPT
chmod +x "$DIST_DIR/run.sh"

cat > "$DIST_DIR/README.RUN.md" <<'README_RUN'
# 运行说明

1. 把 `config.example.json` 复制成自己的 `config.json` 并填写真实配置。
2. 使用 `./run.sh -c config.json` 启动。
3. `run.sh` 会自动把当前包内的 `bin/` 加入 `LD_LIBRARY_PATH`；TDLib 与其运行时依赖和 `transfer_bot` 位于同一目录。
4. 该产物只保证在同名或兼容发行版上运行，例如 Alpine 包优先用于 Alpine。
README_RUN

cp README.md "$DIST_DIR/README.md"
cp config.example.json "$DIST_DIR/config.example.json"
cp LICENSE "$DIST_DIR/LICENSE"

{
  echo "artifact=$CI_ARTIFACT_NAME"
  echo "distro=$CI_DISTRO_ID"
  echo "image=$CI_IMAGE"
  echo "td_ref=$TD_GIT_REF"
  echo "td_commit=$TD_COMMIT"
  echo "rustc=$(rustc --version)"
  echo "cargo=$(cargo --version)"
  echo "built_at_utc=$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
} > "$DIST_DIR/BUILD_INFO.txt"

tar -czf "$DIST_ROOT/$CI_ARTIFACT_NAME.tar.gz" -C "$DIST_ROOT" "$CI_ARTIFACT_NAME"
if [ "$PACKAGE_RETENTION_KIND" = "full" ]; then
  (
    cd "$DIST_ROOT"
    sha256sum "$CI_ARTIFACT_NAME.tar.gz" > "$CI_ARTIFACT_NAME.sha256"
  )
fi
