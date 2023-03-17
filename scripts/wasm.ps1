$ScriptPath = Split-Path -Path $MyInvocation.MyCommand.Path -Parent
$ProjectPath = Split-Path -Path $ScriptPath -Parent

Set-Location $ProjectPath

cargo build --release --target wasm32-unknown-unknown

$WasmSourcePath = Join-Path -Path $ProjectPath -ChildPath 'target/wasm32-unknown-unknown/release/wasm.wasm'
$WasmDestinationPath = Join-Path -Path $ProjectPath -ChildPath 'wasm'

Copy-Item -Path $WasmSourcePath -Destination $WasmDestinationPath

python -m http.server -d $WasmDestinationPath -b localhost 8001
