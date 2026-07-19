$ErrorActionPreference = "Stop"

# 统一 Windows 包构建脚本：
# - 准备 vcpkg / gperf
# - 源码编译 TDLib
# - 可选执行 fmt/test/clippy
# - 构建 release 二进制并打包

function Require-Env([string]$Name) {
    $value = [Environment]::GetEnvironmentVariable($Name)
    if ([string]::IsNullOrWhiteSpace($value)) {
        throw "Missing required env: $Name"
    }
}

Require-Env "CI_ARTIFACT_NAME"
Require-Env "TD_GIT_REF"
Require-Env "RUN_CHECKS"

$workspace = if ($env:GITHUB_WORKSPACE) { $env:GITHUB_WORKSPACE } else { $PWD.Path }
$localTdlibPath = if ($env:LOCAL_TDLIB_PATH) { $env:LOCAL_TDLIB_PATH } else { Join-Path $workspace "tdlib-install" }
$tdBuildDir = if ($env:TD_BUILD_DIR) { $env:TD_BUILD_DIR } else { Join-Path $env:RUNNER_TEMP "td-build" }
$tdSourceDir = if ($env:TD_SOURCE_DIR) { $env:TD_SOURCE_DIR } else { Join-Path $env:RUNNER_TEMP "td" }
$targetRoot = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $workspace "target" }
$packageRetentionKind = if ($env:PACKAGE_RETENTION_KIND) { $env:PACKAGE_RETENTION_KIND } else { "full" }
$packageMode = if ($env:PACKAGE_MODE) { $env:PACKAGE_MODE } else { "package" }

$vcpkgRoot = $env:VCPKG_INSTALLATION_ROOT
if ([string]::IsNullOrWhiteSpace($vcpkgRoot)) {
    $vcpkgRoot = Join-Path $env:RUNNER_TEMP "vcpkg"
}
$vcpkg = Join-Path $vcpkgRoot "vcpkg.exe"
if (-not (Test-Path $vcpkg)) {
    Remove-Item -Recurse -Force $vcpkgRoot -ErrorAction SilentlyContinue
    git clone --depth 1 https://github.com/microsoft/vcpkg.git $vcpkgRoot
    if ($LASTEXITCODE -ne 0) {
        throw "vcpkg clone failed"
    }
    & (Join-Path $vcpkgRoot "bootstrap-vcpkg.bat") -disableMetrics
    if ($LASTEXITCODE -ne 0) {
        throw "vcpkg bootstrap failed"
    }
}
$env:VCPKG_ROOT = $vcpkgRoot

& $vcpkg install openssl:x64-windows zlib:x64-windows
if ($LASTEXITCODE -ne 0) {
    throw "vcpkg dependency install failed"
}

if (-not (Get-Command gperf.exe -ErrorAction SilentlyContinue)) {
    choco install gperf -y --no-progress
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "Chocolatey gperf install failed, fallback to vcpkg"
    }
}
if (-not (Get-Command gperf.exe -ErrorAction SilentlyContinue)) {
    & $vcpkg install gperf:x64-windows
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "vcpkg gperf install failed, will search existing tools"
    }
    $env:Path = (Join-Path $vcpkgRoot "installed\x64-windows\tools\gperf") + ";$env:Path"
}

$gperf = Get-Command gperf.exe -ErrorAction SilentlyContinue
if (-not $gperf) {
    $searchRoots = @($vcpkgRoot, $env:ChocolateyInstall) |
        Where-Object { -not [string]::IsNullOrWhiteSpace($_) -and (Test-Path $_) }
    $gperf = Get-ChildItem $searchRoots -Recurse -Filter gperf.exe -ErrorAction SilentlyContinue |
        Select-Object -First 1
}
if (-not $gperf) {
    throw "gperf.exe was not found after dependency installation"
}

rustc --version
cargo --version
cmake --version
gperf --version

Remove-Item -Recurse -Force $tdSourceDir, $tdBuildDir, $localTdlibPath -ErrorAction SilentlyContinue
git clone --depth 1 https://github.com/tdlib/td.git $tdSourceDir
Push-Location $tdSourceDir
if ($env:TD_GIT_REF -ne "master") {
    git fetch --depth 1 origin $env:TD_GIT_REF
    if ($LASTEXITCODE -ne 0) {
        git fetch --depth 1 origin "refs/tags/$env:TD_GIT_REF:refs/tags/$env:TD_GIT_REF"
    }
    git checkout $env:TD_GIT_REF
}
$tdCommit = git rev-parse HEAD
Pop-Location

cmake -S $tdSourceDir -B $tdBuildDir `
    -A x64 `
    -DCMAKE_BUILD_TYPE=Release `
    -DCMAKE_INSTALL_PREFIX="$localTdlibPath" `
    -DCMAKE_TOOLCHAIN_FILE="$env:VCPKG_ROOT\scripts\buildsystems\vcpkg.cmake" `
    -DVCPKG_TARGET_TRIPLET=x64-windows `
    -DGPERF_EXECUTABLE="$($gperf.Source)" `
    -DTD_ENABLE_LTO=OFF
cmake --build $tdBuildDir --config Release --target install --parallel 2

if (-not (Test-Path "$localTdlibPath\bin\tdjson.dll")) {
    throw "tdjson.dll was not installed to $localTdlibPath\bin"
}
if (-not (Test-Path "$localTdlibPath\lib\tdjson.lib")) {
    throw "tdjson.lib was not installed to $localTdlibPath\lib"
}

$env:Path = "$localTdlibPath\bin;$env:VCPKG_ROOT\installed\x64-windows\bin;$env:Path"

if ($env:RUN_CHECKS -eq "true") {
    cargo fmt --all -- --check
    cargo test -p transfer_bot
    cargo clippy -p transfer_bot --all-targets --no-deps -- -D warnings
}

cargo build -p transfer_bot --release
if (-not (Test-Path (Join-Path $targetRoot "release\transfer_bot.exe"))) {
    throw "transfer_bot.exe was not built"
}

if ($packageMode -eq "build_only") {
    Write-Host "build_only completed: $(Join-Path $targetRoot 'release\transfer_bot.exe')"
    exit 0
}

$artifact = $env:CI_ARTIFACT_NAME
$distRoot = Join-Path $workspace "dist"
$dist = Join-Path $distRoot $artifact
Remove-Item -Recurse -Force $dist, "$dist.zip", "$dist.sha256" -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force "$dist\bin" | Out-Null

Copy-Item (Join-Path $targetRoot "release\transfer_bot.exe") "$dist\bin\transfer_bot.exe" -Force
Copy-Item "$localTdlibPath\bin\tdjson.dll" "$dist\bin\tdjson.dll" -Force

$vcpkgBin = Join-Path $env:VCPKG_ROOT "installed\x64-windows\bin"
if (Test-Path $vcpkgBin) {
    Copy-Item "$vcpkgBin\*.dll" "$dist\bin\" -Force -ErrorAction SilentlyContinue
}

@(
    '$ErrorActionPreference = "Stop"'
    '$AppDir = Split-Path -Parent $MyInvocation.MyCommand.Path'
    '$env:Path = (Join-Path $AppDir "bin") + ";$env:Path"'
    '& (Join-Path $AppDir "bin\transfer_bot.exe") @args'
    'exit $LASTEXITCODE'
) | Set-Content -Encoding utf8 "$dist\run.ps1"

@(
    '@echo off'
    'set "APP_DIR=%~dp0"'
    'set "PATH=%APP_DIR%bin;%PATH%"'
    '"%APP_DIR%bin\transfer_bot.exe" %*'
    'exit /b %ERRORLEVEL%'
) | Set-Content -Encoding ascii "$dist\run.cmd"

Get-ChildItem "$dist\bin" -Filter "*.dll" |
    Select-Object Name, Length |
    Format-Table -AutoSize |
    Out-String |
    Set-Content -Encoding utf8 "$dist\runtime-dlls.txt"

@(
    '# 运行说明'
    ''
    '1. 把 `config.example.json` 复制成自己的 `config.json` 并填写真实配置。'
    '2. PowerShell 使用 `.\run.ps1 -c .\config.json` 启动。'
    '3. cmd 使用 `run.cmd -c config.json` 启动。'
    '4. `run.ps1` / `run.cmd` 会自动把当前包内的 `bin\` 加入 `PATH`。'
    '5. 如果目标机器缺少 MSVC 运行库，请安装 Microsoft Visual C++ Redistributable。'
) | Set-Content -Encoding utf8 "$dist\README.RUN.md"

Copy-Item (Join-Path $workspace "README.md") "$dist\README.md" -Force
Copy-Item (Join-Path $workspace "config.example.json") "$dist\config.example.json" -Force
Copy-Item (Join-Path $workspace "LICENSE") "$dist\LICENSE" -Force

@(
    "artifact=$artifact"
    "distro=windows-2022"
    "image=windows-2022"
    "td_ref=$env:TD_GIT_REF"
    "td_commit=$tdCommit"
    "rustc=$(rustc --version)"
    "cargo=$(cargo --version)"
    "built_at_utc=$((Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ'))"
) | Set-Content -Encoding utf8 "$dist\BUILD_INFO.txt"

Compress-Archive -Path $dist -DestinationPath "$dist.zip" -Force
if ($packageRetentionKind -eq "full") {
    $hash = Get-FileHash -Algorithm SHA256 "$dist.zip"
    ("{0}  {1}" -f $hash.Hash.ToLowerInvariant(), (Split-Path -Leaf $hash.Path)) |
        Set-Content -Encoding utf8 "$dist.sha256"
}
