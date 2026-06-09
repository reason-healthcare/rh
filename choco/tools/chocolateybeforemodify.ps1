$ErrorActionPreference = 'Stop'

$binPath = Join-Path (Split-Path -Parent $MyInvocation.MyCommand.Definition) 'rh.exe'

if (Test-Path $binPath) {
  Write-Output "rh is installed at $binPath"
  exit 0
} else {
  Write-Output "rh is not installed"
  exit 1
}