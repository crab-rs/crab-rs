build_ios_sim:
	cargo bundle --target aarch64-apple-ios-sim --features mobile
	#cp assets/Info.plist target/aarch64-apple-ios-sim/debug/bundle/ios/crab.rs.app/Info.plist

build_ios_device:
	cargo clean
	cargo bundle --release --target aarch64-apple-ios --features mobile

build_macos:
	cargo bundle --release --target aarch64-apple-darwin --features desktop

build_ipa:build_ios_device
	cp -r target/aarch64-apple-ios/release/bundle/ios target/Payload
	cd target &&\
	rm -rf crab.rs.ipa&&\
	zip -r crab.rs.ipa Payload&&\
	rm -rf ../Payload&&\
	cp -r  crab.rs.ipa /Users/zhouzhipeng/Library/Mobile\ Documents/com~apple~CloudDocs/crab.rs.ipa

build_wasm:
	cargo clean
    #brew install llvm
	export PATH="/opt/homebrew/opt/llvm/bin:$(PATH)" &&  export LDFLAGS="-L/opt/homebrew/opt/llvm/lib" && export CPPFLAGS="-I/opt/homebrew/opt/llvm/include" && \
	rm -rf dist && \
	dx build --release --features web

install_ios_sim:
	xcrun simctl install booted target/aarch64-apple-ios-sim/debug/bundle/ios/crab.rs.app

run_macos:
	cargo run --package crab.rs --bin crab.rs  --features desktop

run_ios_sim:build_ios_sim install_ios_sim
	xcrun simctl launch --console booted com.zhouzhipeng.crab.rs

upload_wasm:build_wasm
	cp -r dist crab.rs

	zip -r crab.rs.zip crab.rs

	rm -rf crab.rs

	mv crab.rs.zip dist/

	curl 'https://zhouzhipeng.com/files/upload?unzip=true' \
      -H "${AUTH_KEY}" \
      -F "file=@dist/crab.rs.zip"

	# clean CF cache

	curl --request POST \
      --url https://api.cloudflare.com/client/v4/zones/4e12ec626e551ffaa8fc17a27339fdf1/purge_cache \
      -H 'Content-Type: application/json' \
      -H "${CF_CACHE_KEY}" \
      --data '{"purge_everything": true}'


dev_server:
	lsof -ti:8080 | xargs kill -9
	export PATH="/opt/homebrew/opt/llvm/bin:${PATH}" && \
	export LDFLAGS="-L/opt/homebrew/opt/llvm/lib" &&\
	export CPPFLAGS="-I/opt/homebrew/opt/llvm/include" &&\
	cd crates/app && dx serve --hot-reload  --features web --platform web --open

