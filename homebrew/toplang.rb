class Toplang < Formula
  desc "TopLang - A simple, readable programming language compiler"
  homepage "https://github.com/taufiksoleh/toplang"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/taufiksoleh/toplang/releases/download/v#{version}/toplang-macos-x64"
      sha256 "" # Will be filled when you create a release
    elsif Hardware::CPU.arm?
      url "https://github.com/taufiksoleh/toplang/releases/download/v#{version}/toplang-macos-arm64"
      sha256 "" # Will be filled when you create a release
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/taufiksoleh/toplang/releases/download/v#{version}/toplang-linux-x64"
      sha256 "" # Will be filled when you create a release
    elsif Hardware::CPU.arm?
      url "https://github.com/taufiksoleh/toplang/releases/download/v#{version}/toplang-linux-arm64"
      sha256 "" # Will be filled when you create a release
    end
  end

  def install
    bin.install "toplang-macos-x64" => "topc" if OS.mac? && Hardware::CPU.intel?
    bin.install "toplang-macos-arm64" => "topc" if OS.mac? && Hardware::CPU.arm?
    bin.install "toplang-linux-x64" => "topc" if OS.linux? && Hardware::CPU.intel?
    bin.install "toplang-linux-arm64" => "topc" if OS.linux? && Hardware::CPU.arm?
  end

  test do
    # Create a simple test program
    (testpath/"test.top").write <<~EOS
      function main() {
          print "Hello from TopLang"
          return 0
      }
    EOS

    # Run the test program
    output = shell_output("#{bin}/topc #{testpath}/test.top")
    assert_match "Hello from TopLang", output
  end
end
