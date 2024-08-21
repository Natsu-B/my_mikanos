#!/bin/sh

mkdir -p bin/EFI/BOOT/
#mv $1 bin/EFI/BOOT/BOOTX64.EFI
mv ~/osbook/day01/c/hello.efi bin/EFI/BOOT/BOOTX64.EFI

echo "Run qemu..."

qemu-img create -f raw disk.img 200M

mkfs.fat -n 'MIKAN OS' -s 2 -f 2 -R 32 -F 32 disk.img
mkdir -p bin
sudo mount -o loop disk.img bin
sudo mkdir -p bin/EFI/BOOT
sudo cp target/x86_64-unknown-uefi/release/kernel.efi bin/EFI/BOOT/BOOTX64.EFI
sleep 0.5
sudo umount bin

qemu-system-x86_64 \
    -m 1G \
    -drive if=pflash,format=raw,readonly,file=../file/OVMF_CODE.fd \
    -drive if=pflash,format=raw,file=../file/OVMF_VARS.fd \
    -drive if=ide,index=0,media=disk,format=raw,file=disk.img \
    -device nec-usb-xhci,id=xhci \
    -device usb-mouse -device usb-kbd \
    -monitor stdio

qemu-system-x86_64 \
    -drive if=pflash,file=../file/OVMF_CODE.fd \
    -drive if=pflash,file=../file/OVMF_VARS.fd \
    -hda disk.img

qemu-system-x86_64 \
    -drive if=pflash,file=$HOME/osbook/devenv/OVMF_CODE.fd \
    -drive if=pflash,file=$HOME/osbook/devenv/OVMF_VARS.fd \
    -hda disk.img

clang -target x86_64-pc-win32-coff \
    -mno-red-zone -fno-stack-protector -fshort-wchar -Wall -c hello.c

lld-link /subsystem:efi_application /entry:EfiMain /out:hello.efi hello.o