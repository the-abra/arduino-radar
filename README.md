# Arduino Radar

This project uses an Arduino Uno (or equivalent), an `SG90` servo motor, and an `HC-SR04` ultrasonic distance sensor to create a `radar-like` scanning system. The servo motor sweeps the sensor back and forth, detecting objects in its path and providing distance(*100CM~*) data.

## Requirements
### Hardware

    Arduino Uno (or equivalent)
    SG90 Servo Motor
    HC-SR04 Ultrasonic Distance Sensor

### Wiring Connections

Follow the wiring instructions below to set up your circuit:
**HC-SR04 Ultrasonic Distance Sensor**:

    TRIG => Arduino pin 3 (PWM)
    ECHO => Arduino pin 2 (PWM)
    VCC  => Arduino 5V
    GND  => Arduino GND

**SG90 Servo Motor**:

    Signal (orange wire) => Arduino pin 9 (PWM)
    GND (black wire)     => Arduino GND
    VCC (red wire)       => Arduino 5V

## Upload the Code

Once the hardware is set up, upload the code to your Arduino. Download the `arduino-radar-code.ino` file and use the Arduino IDE to upload it to your board.

    In Arduino IDE, select your board and port, then upload the code:
    File -> Open -> arduino-radar-code.ino -> Upload

## Run TUI via pre-build (Rust:Linux)

    ./build/serial_monitor.build /dev/ttyUSB0
    
  Make sure your Arduino is connected via USB to /dev/ttyUSB0 (or the correct port for your system).<br/>
  **Note**: You probably need root to access the serial connection.

### Building the TUI (Rust)

The TUI for monitoring the radar system is built in ***Rust***. The build includes a serial monitor that communicates with the Arduino to visualize the radar data. To use it, follow these steps:

**1. Build the Project**

    cargo build --release

**2. Run the Serial Monitor**

Make sure your Arduino is connected via USB to `/dev/ttyUSB0` (or the correct port for your system) and run the serial monitor:

**3. Adjust the port:**
     
     cargo run -- /dev/ttyUSB0
 
The radar's movement and distance readings will be displayed in the terminal.

# Notes

 Ensure your Arduino board is correctly connected to your computer and that the correct serial port is selected. <br/>
 The servo motor will rotate back and forth, scanning for objects in front of the `HC-SR04` sensor. The TUI displays distance readings as the servo moves.<br/>
 If you encounter any issues, check the wiring and ensure your `Arduino IDE` and `Rust` environment are properly set up.
