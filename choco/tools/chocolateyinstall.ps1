$version = $env:ChocolateyPackageVersion
if (-not $version) { $version = "0.2.1" }

$url = "https://github.com/reason-healthcare/rh/releases/download/v$version/rh-x86_64-pc-windows-msvc.zip"

$packageArgs = @{
  packageName   = 'rh'
  url           = $url
  unzipLocation = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"
  checksum      = 'PLACEHOLDER_SHA256'
  checksumType  = 'sha256'
}

Install-ChocolateyZipPackage @packageArgs