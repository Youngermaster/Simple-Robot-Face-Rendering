#!/usr/bin/env python3
"""
Temperature Sensor Simulator

Simulates a temperature sensor that sends readings via WebSocket.
Generates realistic temperature data with noise and slow drift.
"""

import asyncio
import json
import random
import time
import websockets

# Sensor configuration
WEBSOCKET_URL = "ws://localhost:8080/ws/temperature"
UPDATE_RATE = 0.1  # seconds between updates (10 Hz)

# Temperature simulation parameters
BASE_TEMP = 22.0  # Base temperature in Celsius
TEMP_RANGE = 5.0  # Temperature variation range
NOISE_LEVEL = 0.3  # Random noise amplitude

class TemperatureSensor:
    def __init__(self):
        self.current_temp = BASE_TEMP
        self.drift_offset = 0.0
        self.drift_speed = 0.01

    def read_temperature(self):
        """Simulate temperature reading with realistic behavior"""
        # Slow drift (simulates environmental changes)
        self.drift_offset += random.uniform(-self.drift_speed, self.drift_speed)
        self.drift_offset = max(-TEMP_RANGE, min(TEMP_RANGE, self.drift_offset))

        # Random noise
        noise = random.uniform(-NOISE_LEVEL, NOISE_LEVEL)

        # Calculate current temperature
        self.current_temp = BASE_TEMP + self.drift_offset + noise

        return round(self.current_temp, 2)


async def send_temperature_data():
    """Connect to WebSocket and stream temperature data"""
    sensor = TemperatureSensor()

    print(f"🌡️  Temperature Sensor Simulator")
    print(f"   Connecting to {WEBSOCKET_URL}...")

    try:
        async with websockets.connect(WEBSOCKET_URL) as websocket:
            print(f"   ✓ Connected!")
            print(f"   Streaming data at {1/UPDATE_RATE:.0f} Hz")
            print(f"   Press Ctrl+C to stop\n")

            while True:
                # Read sensor
                temperature = sensor.read_temperature()

                # Create message
                message = {
                    "value": temperature,
                    "unit": "C"
                }

                # Send via WebSocket
                await websocket.send(json.dumps(message))
                print(f"   📡 Sent: {temperature:.2f}°C", end='\r')

                # Wait before next update
                await asyncio.sleep(UPDATE_RATE)

    except websockets.exceptions.WebSocketException as e:
        print(f"\n   ❌ Connection error: {e}")
        print(f"   Make sure the Rust server is running!")
    except KeyboardInterrupt:
        print(f"\n\n   ⏹️  Sensor stopped")


if __name__ == "__main__":
    asyncio.run(send_temperature_data())
