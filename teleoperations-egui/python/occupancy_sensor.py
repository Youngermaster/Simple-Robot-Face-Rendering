#!/usr/bin/env python3
"""
Room Occupancy Sensor Simulator

Simulates a room occupancy sensor that sends state updates via HTTP POST.
Simulates people entering and leaving a room with realistic patterns.
"""

import requests
import json
import random
import time

# Sensor configuration
HTTP_URL = "http://localhost:8080/api/occupancy"
UPDATE_RATE = 2.0  # seconds between updates

# Occupancy simulation parameters
ENTRY_PROBABILITY = 0.15  # Chance someone enters per update
EXIT_PROBABILITY = 0.20   # Chance someone exits per update
MAX_OCCUPANTS = 10        # Maximum room capacity

class OccupancySensor:
    def __init__(self):
        self.current_count = 0
        self.occupied = False

    def simulate_motion(self):
        """Simulate people entering and leaving the room"""
        # Someone might enter
        if self.current_count < MAX_OCCUPANTS and random.random() < ENTRY_PROBABILITY:
            self.current_count += random.randint(1, 2)
            self.current_count = min(self.current_count, MAX_OCCUPANTS)

        # Someone might leave
        if self.current_count > 0 and random.random() < EXIT_PROBABILITY:
            self.current_count -= random.randint(1, min(2, self.current_count))
            self.current_count = max(0, self.current_count)

        # Update occupancy state
        self.occupied = self.current_count > 0

        return self.occupied, self.current_count


def send_occupancy_update(sensor):
    """Send occupancy state via HTTP POST"""
    try:
        # Simulate motion
        occupied, count = sensor.simulate_motion()

        # Create payload
        payload = {
            "occupied": occupied,
            "count": count
        }

        # Send HTTP POST request
        response = requests.post(HTTP_URL, json=payload, timeout=5)

        if response.status_code == 200:
            status = "🟢 OCCUPIED" if occupied else "🔴 VACANT"
            print(f"   📡 {status} | Count: {count:2d} | ✓ Sent")
        else:
            print(f"   ❌ Server returned status {response.status_code}")

    except requests.exceptions.ConnectionError:
        print(f"   ❌ Connection failed - is the server running?")
    except requests.exceptions.Timeout:
        print(f"   ❌ Request timed out")
    except Exception as e:
        print(f"   ❌ Error: {e}")


def main():
    """Main sensor loop"""
    sensor = OccupancySensor()

    print(f"🚪 Room Occupancy Sensor Simulator")
    print(f"   Target: {HTTP_URL}")
    print(f"   Update rate: {UPDATE_RATE}s")
    print(f"   Press Ctrl+C to stop\n")

    try:
        while True:
            send_occupancy_update(sensor)
            time.sleep(UPDATE_RATE)

    except KeyboardInterrupt:
        print(f"\n\n   ⏹️  Sensor stopped")


if __name__ == "__main__":
    main()
