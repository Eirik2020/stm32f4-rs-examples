from serial import Serial
ser = Serial(port='COM6', baudrate=115200, timeout=1)

print("Listening on COM6...")
try:
    while True:
        if ser.in_waiting:
            data = ser.readline().decode('utf-8').strip()
            print(f"Received: {data}")
except KeyboardInterrupt:
    print("Stopped by user.")
finally:
    ser.close()
