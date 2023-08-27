Code was tested with a OLED screen containing driver for SH1106.
A SH1106 display will work when using a SSD1306 library but shifted left 2 columns. Some artifacts are observed on the right side, but only one column.

display.clear is not working, and thus this line:
--->      let _ = display.write_str("                                                                               ");
is doing the clearing. Sure this is not working 100% as intended, but will do the basic job.

LED blink is also implemented.
