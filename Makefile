export TIZEN_STUDIO=$(HOME)/Tizen/tizen-studio
export TIZEN_STUDIO_DATA=$(HOME)/Tizen/tizen-studio-data
export TIZEN_VERSION=5.5
export TIZEN_PROFILE=wearable
export TIZEN_ROOTSTRAP_PATH=$(TIZEN_STUDIO)/platforms/tizen-$(TIZEN_VERSION)/$(TIZEN_PROFILE)/rootstraps/$(TIZEN_PROFILE)-$(TIZEN_VERSION)-emulator.core
export TIZEN_TOOLCHAIN=gcc-6.2
#export TIZEN_TRIPLE=arm-linux-gnueabi
#export RUST_TRIPLE=arm-unknown-linux-gnueabi
export TIZEN_TRIPLE=i586-linux-gnueabi
export RUST_TRIPLE=i586-unknown-linux-gnu
#export RUST_TRIPLE=armv5te-unknown-linux-gnueabi 
export TIZEN_TOOLCHAIN_PATH=$(TIZEN_STUDIO)/tools/$(TIZEN_TRIPLE)-$(TIZEN_TOOLCHAIN)/bin
export TIZEN_IDE_BIN_PATH=$(TIZEN_STUDIO)/tools/ide/bin
export PATH:=$(TIZEN_IDE_BIN_PATH):$(TIZEN_TOOLCHAIN_PATH):$(PATH)
export PKG_CONFIG_SYSROOT_DIR=$(TIZEN_ROOTSTRAP_PATH)
export PKG_CONFIG_LIBDIR=$(TIZEN_ROOTSTRAP_PATH)/usr/lib/pkgconfig
export PKG_CONFIG_PATH=
export PKG_CONFIG_ALLOW_CROSS=1
export RUSTFLAGS=-C link-args=--sysroot=$(TIZEN_ROOTSTRAP_PATH)
#export CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_LINKER=$(TIZEN_TOOLCHAIN_PATH)/$(TIZEN_TRIPLE)-gcc
export CARGO_TARGET_I586_UNKNOWN_LINUX_GNU_LINKER=$(TIZEN_TOOLCHAIN_PATH)/$(TIZEN_TRIPLE)-gcc

export TIZEN_SECURITY_PROFILE=andersondanilo

export RUST_TARGET_OUT=target/$(RUST_TRIPLE)/debug
export RUST_APP_NAME=tizen-example
export TIZEN_DEBUG_OUT=./Debug
export TIZEN_PKG_NAME=com.andersondanilo.tizen-example
export TIZEN_TPK_NAME=$(TIZEN_PKG_NAME)-0.1.0.tpk 
export TIZEN_TPK_NAME2=$(TIZEN_PKG_NAME)-0.1.0-x86.tpk 

all = package-debug

package-debug:
	echo $(PATH)
	cargo build --target=$(RUST_TRIPLE)
	rm -rf $(TIZEN_DEBUG_OUT) && mkdir $(TIZEN_DEBUG_OUT)
	# cp $(RUST_TARGET_OUT)/$(RUST_APP_NAME) $(TIZEN_DEBUG_OUT)
	cp -r $(RUST_TARGET_OUT)/* $(TIZEN_DEBUG_OUT)

	echo project-path=$(PWD) > $(TIZEN_DEBUG_OUT)/build.info
	echo profile=Wearable >> $(TIZEN_DEBUG_OUT)/build.info
	echo profile-version=$(TIZEN_VERSION) >> $(TIZEN_DEBUG_OUT)/build.info
	echo type=app >> $(TIZEN_DEBUG_OUT)/build.info
	echo config=Debug >> $(TIZEN_DEBUG_OUT)/build.info
	echo toolchain=$(TIZEN_TOOLCHAIN) >> $(TIZEN_DEBUG_OUT)/build.info
	echo architecture=x86 >> $(TIZEN_DEBUG_OUT)/build.info

	tizen package -t tpk -s $(TIZEN_SECURITY_PROFILE) -- $(TIZEN_DEBUG_OUT)

install-pkg:
	tizen install -n $(TIZEN_TPK_NAME2) -- ./Debug

run-pkg:
	tizen run -p $(TIZEN_PKG_NAME)

certificate:
	tizen certificate -a $(TIZEN_SECURITY_PROFILE) -p 1234 -c BR -s "Sao Paulo" -ct "Sao Paulo" -o OpenSource -n "Anderson Danilo" -e contact@andersondanilo.com -f $(TIZEN_SECURITY_PROFILE)-cert

security-profile:
	tizen security-profiles add -n $(TIZEN_SECURITY_PROFILE) -a $(TIZEN_STUDIO_DATA)/keystore/author/andersondanilo-cert.p12 -p 1234

clean:
	cargo clean
