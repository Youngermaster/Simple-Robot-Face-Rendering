#!/usr/bin/env python3
"""
IMU Accelerometer Sensor Simulator

Simulates a 3-axis IMU (Inertial Measurement Unit) accelerometer.
Generates realistic acceleration data with gravity and motion patterns.
"""

import asyncio
import json
import math
import random
import time
import websockets

# Sensor configuration
WEBSOCKET_URL = "ws://localhost:8080/ws/imu"
UPDATE_RATE = 0.05  # seconds between updates (20 Hz)

# IMU simulation parameters
GRAVITY = 9.81  # m/s² (Earth gravity)
NOISE_LEVEL = 0.05  # Random noise amplitude

class IMUSensor:
    def __init__(self):
        self.time = 0.0
        self.motion_phase = 0.0

    def read_acceleration(self):
        """Simulate 3-axis acceleration with realistic motion patterns"""
        self.time += UPDATE_RATE
        self.motion_phase += 0.1

        # Simulate different motion patterns
        # X-axis: Sinusoidal motion (like swinging)
        x = 0.5 * math.sin(self.motion_phase)

        # Y-axis: Gravity + small oscillations (like standing with vibrations)
        y = GRAVITY + 0.3 * math.sin(self.motion_phase * 2.3)

        # Z-axis: Random walk (like random tilting)
        z = 0.2 * math.sin(self.motion_phase * 0.7)

        # Add noise to all axes
        x += random.uniform(-NOISE_LEVEL, NOISE_LEVEL)
        y += random.uniform(-NOISE_LEVEL, NOISE_LEVEL)
        z += random.uniform(-NOISE_LEVEL, NOISE_LEVEL)

        return round(x, 3), round(y, 3), round(z, 3)


async def send_imu_data():
    """Connect to WebSocket and stream IMU data"""
    sensor = IMUSensor()

    print(f"📊 IMU Accelerometer Simulator")
    print(f"   Connecting to {WEBSOCKET_URL}...")

    try:
        async with websockets.connect(WEBSOCKET_URL) as websocket:
            print(f"   ✓ Connected!")
            print(f"   Streaming data at {1/UPDATE_RATE:.0f} Hz")
            print(f"   Press Ctrl+C to stop\n")

            while True:
                # Read sensor
                x, y, z = sensor.read_acceleration()

                # Create message
                message = {
                    "x": x,
                    "y": y,
                    "z": z
                }

                # Send via WebSocket
                await websocket.send(json.dumps(message))
                print(f"   📡 X: {x:+.3f} | Y: {y:+.3f} | Z: {z:+.3f} m/s²", end='\r')

                # Wait before next update
                await asyncio.sleep(UPDATE_RATE)

    except websockets.exceptions.WebSocketException as e:
        print(f"\n   ❌ Connection error: {e}")
        print(f"   Make sure the Rust server is running!")
    except KeyboardInterrupt:
        print(f"\n\n   ⏹️  Sensor stopped")


if __name__ == "__main__":
    asyncio.run(send_imu_data())
