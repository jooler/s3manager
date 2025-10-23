#!/bin/bash
# 修改应用名、项目路径、证书名称
APP_NAME="R2Uploader"
PROJECT_DIR="/Users/vihv/Code/project/R2Uploader"

APPLE_DISTRIBUTION_CERT="Apple Distribution: Shanghai Zero Rust Software Development Co., Ltd. (BY52T9DUMZ)"
THIRD_PARTY_MAC_DEVELOPER_CERT="3rd Party Mac Developer Installer: Shanghai Zero Rust Software Development Co., Ltd. (BY52T9DUMZ)"


APP_BUNDLE="$APP_NAME.app"
APP_EXECUTABLE="$APP_BUNDLE/Contents/MacOS/$APP_NAME"
APP_PACKAGE="$APP_NAME.pkg"
APP_PATH="$PROJECT_DIR/src-tauri/target/universal-apple-darwin/release/bundle/macos/$APP_BUNDLE"

# Build, Sign, Package
bun tauri build --target universal-apple-darwin

# cp "/Users/vihv/Documents/证件信息/苹果证书/r2uploader.provisionprofile" "$APP_PATH/Contents/embedded.provisionprofile"

# codesign --deep --force \
# 	--sign "$APPLE_DISTRIBUTION_CERT" \
# 	--entitlements "entitlements.plist" \
# 	"$APP_PATH"

cd "$PROJECT_DIR/src-tauri/target/universal-apple-darwin/release/bundle/macos"

productbuild \
	--sign "$THIRD_PARTY_MAC_DEVELOPER_CERT" \
	--component "$APP_PATH" "/Applications" \
	"$APP_PACKAGE"
