require "json"

package = JSON.parse(File.read(File.join(__dir__, "package.json")))

Pod::Spec.new do |s|
  s.name         = "react-native-message_signing-library"
  s.version      = package["version"]
  s.summary      = package["description"]
  s.description  = <<-DESC
                 Cardano message signing lib bridge
                   DESC
  s.homepage     = "https://github.com/Emurgo/msl-mobile-bridge"
  s.license      = "MIT"
  # optional - use expanded license entry instead:
  # s.license    = { :type => "MIT", :file => "LICENSE" }
  s.authors      = { "emurgo" => "contact@emurgo.io" }
  s.platforms    = { :ios => "12.4" }
  s.source       = { :git => "https://github.com/Emurgo/msl-mobile-bridge.git", :tag => "#{s.version}" }

  s.source_files = "ios/**/*.{h,c,m,swift,sh}"
  s.requires_arc = true

  s.module_name = 'MessageSigningLib'

  s.script_phase = {
    :name => "Build Rust Binary",
    :script => 'bash "${PODS_TARGET_SRCROOT}/ios/build.sh"',
    :execution_position => :before_compile
  }

  s.pod_target_xcconfig = {
    "HEADER_SEARCH_PATHS" => "$(CONFIGURATION_BUILD_DIR)",
    "OTHER_LIBTOOLFLAGS" => "-lreact_native_message_signing_library",
    "ENABLE_BITCODE" => "NO"
  }

  s.preserve_paths = "rust/**/*"

  s.dependency "React"

end
