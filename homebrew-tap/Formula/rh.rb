class Rh < Formula
  desc "Unified CLI for FHIR processing tools"
  homepage "https://github.com/reason-healthcare/rh"
  version "0.2.0-beta.2"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/reason-healthcare/rh/releases/download/v0.2.0-beta.2/rh-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_ARM64_MACOS"
    else
      url "https://github.com/reason-healthcare/rh/releases/download/v0.2.0-beta.2/rh-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_X86_64_MACOS"
    end
  end

  on_linux do
    url "https://github.com/reason-healthcare/rh/releases/download/v0.2.0-beta.2/rh-x86_64-unknown-linux-musl.tar.gz"
    sha256 "PLACEHOLDER_SHA256_X86_64_LINUX"
  end

  def install
    bin.install "rh"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/rh --version")
  end
end