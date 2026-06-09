$ErrorActionPreference = 'Stop'

$binPath = Join-Path (Split-Path -Parent $MyInvocation.MyCommand.Definition) 'rh.exe'

if (Test-Path $binPath) {
  Remove-Item $binPath -Force
}