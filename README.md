# Kernel Project built in rust

### Running in rust

Building

`$ cargo build`

Running

`$ cargo run`

### Booting in QEMU
`qemu-system-x86_64 -drive format=raw,file=target/x86_64-os/debug/bootimage-os.bin`

### Booting in VirtualBox

Needs to be padded first before converting to vdi

`dd if=target/x86_64-os/debug/bootimage-os.bin of=target/x86_64-os/debug/bootimage-os-padded.bin bs=100m conv=sync`

Convert to VDI

`VBoxManage convertfromraw target/x86_64-os/debug/bootimage-os-padded.bin target/x86_64-os/debug/bootimage-os.vdi --format VDI`

### Credits

Thanks to phil-opp for creating an awesome article on how to create kernel in rust [https://os.phil-opp.com/](https://os.phil-opp.com/)
