<div align="center">

# 🛡️ Safety Instrumented System (SIS) for Pressure Vessels

### Bare-Metal Rust Implementation on ESP32-S3 for Industrial Safety

<br>

<img src="https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust">
<img src="https://img.shields.io/badge/Hardware-ESP32--S3-blue?style=for-the-badge&logo=espressif">
<img src="https://img.shields.io/badge/Framework-esp--hal%20(no__std)-red?style=for-the-badge">
<img src="https://img.shields.io/badge/System-SIS-success?style=for-the-badge">

<br><br>

A high-reliability safety system for pressure vessel monitoring developed using **Bare-Metal Rust**. This project implements real-time signal processing, noise reduction, and safety interlocking to prevent overpressure accidents.

</div>

---

# 👨‍💻 Author

| Name | Student ID |
|---|---|
| Ahmad Fauzi Abdul Razzaq | 2042241017 |

Department of Instrumentation Engineering  
Institut Teknologi Sepuluh Nopember (ITS)

---

# 📘 Project Overview

This project implements a **Safety Instrumented System (SIS)** designed to protect a pressure vessel from overpressure conditions. Unlike a Basic Process Control System (BPCS), this system acts as a dedicated safety layer that triggers a "Trip" (Solenoid Valve activation) when the pressure exceeds safe limits.

### Key Components:
- **Pressure Vessel:** The physical container being monitored.
- **Pressure Transmitter:** Analog sensor providing real-time pressure data.
- **Solenoid Valve:** The Final Control Element (FCE) for pressure relief.
- **ESP32-S3 Controller:** Running a memory-safe Rust firmware.

<p align="center">
  <img src="Plant Structure.jpg" alt="Plant Structure" width="600px">
  <br>
  <em>Physical System Prototype</em>
</p>

---

# 🏗️ System Logic & Architecture

The system follows a strict **Input -> Process -> Output** pipeline designed for safety-critical response.

### Function Block Diagram (FBD)
```mermaid
graph LR
    A["Pressure Sensor<br>(GPIO 1)"] --> B["Averaging Filter<br>(1000 Samples)"]
    B --> C["Scaling Logic<br>P = 3(V - 0.5)"]
    C --> D{"Pressure >= 4.0?"}
    C --> E{"Pressure >= 5.0?"}
    D -- Yes --> F["Warning LED<br>(GPIO 37)"]
    E -- Yes --> G["SR LATCH<br>(Alarm Active)"]
    G --> H["Solenoid Trip<br>(GPIO 35)"]
    I["Reset Button<br>(GPIO 36)"] -- Reset --> G

    style A fill:#f9f,stroke:#333
    style G fill:#f66,stroke:#333
    style H fill:#f66,stroke:#333
✨ Core Features
🦀 Memory-Safe Firmware
Built with no_std Rust, eliminating the risk of garbage collection pauses or memory leaks.
Uses esp-hal for direct hardware register access.
📊 Advanced Signal Processing
Oversampling & Averaging: Collects 1000 samples per calculation to eliminate electrical noise.
Linear Scaling: Converts voltage signals to Bar units using calibrated equations.
🛡️ Two-Stage Safety Logic
Warning Stage (4.0 Bar): Activates a visual LED indicator.
SIS Trip Stage (5.0 Bar): Latches the safety alarm and activates the solenoid valve.
🔓 Latching Mechanism
Once a trip occurs, the system remains in a safe state (Solenoid ON) until a manual hardware reset is performed via the Reset Button, ensuring operator intervention before restarting.
📉 System Response Analysis
The following graphs demonstrate the system's performance during a pressure ramp test:
<p align="center">
<img src="Graph.png" alt="System Analysis" width="800px">
<br>
<em>Full System Response Cycle</em>
</p>
<p align="center">
<img src="Detailed_Graph._png" alt="Detailed Response" width="800px">
<br>
<em>Detailed Trigger Timing: Observe the delay-free interaction between Pressure and Alarm Logic</em>
</p>
🛠️ Technologies Used
Component	Technology
Controller	ESP32-S3 (Xtensa LX7)
Programming Language	Rust (Edition 2021)
Hardware Abstraction	esp-hal / esp-println
Development Tool	espflash
Calibration Logic	AdcCalCurve (ADC1)
📂 Project Structure
code
Text
sis_pressure_vessels/
├── src/
│   └── main.rs         # Core safety logic and HAL implementation
├── .cargo/
│   └── config.toml     # Build target configurations (xtensa-esp32s3)
├── Cargo.toml          # Project dependencies
└── README.md
🚀 Build & Flash
Prerequisites
Install Rust and espflash:
code
Bash
cargo install espflash
Execution
code
Bash
# To build and flash the firmware to ESP32-S3
cargo run --release
📊 Logic Parameters
Parameter	Value	Description
Sampling Rate	1000 samples/cycle	Noise reduction
Scaling Equation	
P
=
3.0
×
(
V
−
0.5
)
P=3.0×(V−0.5)
Voltage to Bar conversion
Warning Threshold	4.0 Bar	LED Alert
Trip Threshold	5.0 Bar	SIS Activation
Fail-Safe Mode	Latched	Manual Reset Required
<div align="center">
Developed for the Algorithm Programming Course
Department of Instrumentation Engineering
Institut Teknologi Sepuluh Nopember (ITS)
</div>
```
