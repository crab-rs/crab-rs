build_ios_sim:
	cargo bundle --target aarch64-apple-ios-sim --features mobile
	#cp assets/Info.plist target/aarch64-apple-ios-sim/debug/bundle/ios/Wallet.app/Info.plist

build_ios_device:
	cargo clean
	cargo bundle --release --target aarch64-apple-ios --features mobile

build_macos:
	cargo bundle --release --target aarch64-apple-darwin --features desktop

build_ipa:build_ios_device
	cp -r target/aarch64-apple-ios/release/bundle/ios target/Payload
	cd target &&\
	rm -rf wallet.ipa&&\
	zip -r wallet.ipa Payload&&\
	rm -rf ../Payload&&\
	cp -r  wallet.ipa /Users/zhouzhipeng/Library/Mobile\ Documents/com~apple~CloudDocs/wallet.ipa

build_wasm:
	cargo clean
    #brew install llvm
	export PATH="/opt/homebrew/opt/llvm/bin:$(PATH)" &&  export LDFLAGS="-L/opt/homebrew/opt/llvm/lib" && export CPPFLAGS="-I/opt/homebrew/opt/llvm/include" && \
	rm -rf dist && \
	dx build --release --features web

install_ios_sim:
	xcrun simctl install booted target/aarch64-apple-ios-sim/debug/bundle/ios/Wallet.app

run_macos:
	cargo run --package Wallet --bin Wallet  --features desktop

run_ios_sim:build_ios_sim install_ios_sim
	xcrun simctl launch --console booted com.zhouzhipeng.wallet

upload_wasm:build_wasm
	cp -r dist wallet

	zip -r wallet.zip wallet

	rm -rf wallet

	mv wallet.zip dist/

	curl 'https://zhouzhipeng.com/files/upload?unzip=true' \
      -H "${AUTH_KEY}" \
      -F "file=@dist/wallet.zip"


dev_server:
	export PATH="/opt/homebrew/opt/llvm/bin:$(PATH)" && \
	export LDFLAGS="-L/opt/homebrew/opt/llvm/lib" &&\
	export CPPFLAGS="-I/opt/homebrew/opt/llvm/include" &&\
	dx serve --hot-reload --features web --platform web
