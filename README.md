# HyperX Mouse Software

!!!! DEVELOPMENT WILL NOT CONTINUE !!!!
This is due to me replacing my HyperX mouse.

However, if you want to continue working on this you can fork it :)


## Workflow
### Tools
- Wireshark (with USBPcap)

### General info
The mouse and the software only communicate through INTERRUPT packets of 64 byte length.
Just because all the packets are padded to be 64 bytes doesn't mean we have to, we can just pad them to the next closest multiple of 8.

Example:
Setting the mouse polling rate is done by sending `[0xd0, 0x00, 0x00, 0x01, <RATE as u8>]` to the mouse.
This only has to be padded to be `[0xd0, 0x00, 0x00, 0x01, <RATE as u8>, 0x00, 0x00, 0x00]` which can be done using the `command!()` macro.

### Steps
1. Plug in your mouse
2. Start the [NGENUITY](https://hyperx.com/pages/ngenuity) software. 
    - You should now see that your mouse shows up in the device list.
3. Start capturing USB packets with wireshark
4. Make changes to the mouse using its software
5. Find the relevant packets and figure out how they work

You can now implement your own setter for the packet in code!
