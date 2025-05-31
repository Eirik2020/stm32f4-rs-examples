from serial import Serial
import time

ser = Serial(port='COM6', baudrate=115200, timeout=1)

print("Listening on COM6...")
try:
    while True:
        # Send message to MCU
        ser.write(b"thunder\n")

        # Wait for MCU to reply
        time.sleep(0.1)  # Small delay to allow MCU response

        # Read response if available
        if ser.in_waiting:
            data = ser.readline().decode('utf-8').strip()
            print(f"MCU replied: {data}")

        time.sleep(1)  # Send every 1 second

except KeyboardInterrupt:
    print("Stopped by user.")
finally:
    ser.close()