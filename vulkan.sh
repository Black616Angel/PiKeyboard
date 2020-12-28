#!/usr/bin/env bash
# taken from https://qengineering.eu/install-vulkan-on-raspberry-pi.html
# and altered into a bash script
set -x # Echo commands to stdout.
set -e # Exit on first error.
set -u # Treat undefined environment variables as errors.

# check for updates
sudo apt-get update -y
sudo apt-get upgrade -y
# install dependencies
sudo apt-get install -qq libxcb-randr0-dev libxrandr-dev -y
sudo apt-get install -qq libxcb-xinerama0-dev libxinerama-dev libxcursor-dev -y
sudo apt-get install -qq libxcb-cursor-dev libxkbcommon-dev xutils-dev -y
sudo apt-get install -qq xutils-dev libpthread-stubs0-dev libpciaccess-dev -y
sudo apt-get install -qq libffi-dev x11proto-xext-dev libxcb1-dev libxcb-*dev -y
sudo apt-get install -qq bison flex libssl-dev libgnutls28-dev x11proto-dri2-dev -y
sudo apt-get install -qq x11proto-dri3-dev libx11-dev libxcb-glx0-dev -y
sudo apt-get install -qq libx11-xcb-dev libxext-dev libxdamage-dev libxfixes-dev -y
sudo apt-get install -qq libva-dev x11proto-randr-dev x11proto-present-dev -y
sudo apt-get install -qq libclc-dev libelf-dev git build-essential mesa-utils -y
sudo apt-get install -qq libvulkan-dev ninja-build libvulkan1 python-mako -y
sudo apt-get install -qq libdrm-dev libxshmfence-dev libxxf86vm-dev libunwind-dev -y
sudo apt-get install -qq valgrind libzstd-dev vulkan-tools vulkan-utils -y
sudo apt-get install -qq ninja-build -y
# remove old versions first
sudo rm -rf ~/mesa_vulkan

# (re)install meson
sudo apt purge meson -y
sudo apt install meson -y
sudo pip3 install meson
# install mako
sudo pip3 install mako
# install v3dv
cd ~
git clone https://gitlab.freedesktop.org/mesa/mesa.git mesa_vulkan

# now build v3dv
cd mesa_vulkan

arm32=$( grep armv7l <<<$(uname -a) )
arch64=$( grep aarch64 <<<$(uname -a) )
if [[ "$arm32" != "" ]]; then
	CFLAGS="-mcpu=cortex-a72 -mfpu=neon-fp-armv8" CXXFLAGS="-mcpu=cortex-a72 -mfpu=neon-fp-armv8" \ meson --prefix /usr \ -D platforms=x11 \ -D vulkan-drivers=broadcom \ -D dri-drivers= \ -D gallium-drivers=kmsro,v3d,vc4 \ -D buildtype=release build
elif [[ "$arch64" != "" ]]; then
	CFLAGS="-mcpu=cortex-a72" CXXFLAGS="-mcpu=cortex-a72" \ meson --prefix /usr \ -D platforms=x11 \ -D vulkan-drivers=broadcom \ -D dri-drivers= \-D gallium-drivers=kmsro,v3d,vc4 \-D buildtype=release build
else
	return 1
fi
ninja -C build -j4
sudo ninja -C build install
# check your driver
vulkaninfo