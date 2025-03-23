import serial
import struct
import numpy as np

PORT = '/dev/ttyUSB0'
BAUDRATE = 230400

try:
    ser = serial.Serial(PORT, BAUDRATE, timeout=1)
except Exception as e:
    print(f"Error abriendo el puerto serie: {e}")
    exit()

def parse_ldrobot_packet(packet):
    if len(packet) < 47 or packet[0] != 0x54:
        return [], []

    try:
        start_angle = struct.unpack('<H', packet[4:6])[0] / 100.0
        end_angle = struct.unpack('<H', packet[-5:-3])[0] / 100.0
        num_points = (len(packet) - 10) // 3

        angles = []
        distances = []

        for i in range(num_points):
            offset = 6 + i * 3
            dist = struct.unpack('<H', packet[offset:offset+2])[0]
            angle = start_angle + (end_angle - start_angle) * (i / num_points)
            angle %= 360
            angles.append(np.deg2rad(angle))
            distances.append(dist)

        return angles, distances

    except:
        return [], []

try:
    buffer = bytearray()
    while True:
        buffer += ser.read(100)

        while len(buffer) >= 47:
            if buffer[0] != 0x54:
                buffer.pop(0)
                continue

            packet = buffer[:47]
            buffer = buffer[47:]

            angs, dists = parse_ldrobot_packet(packet)
            if angs and dists:
                # Zona frontal: entre -20° y +20° en radianes (~0 a 0.35 o >6.1)
                frontal = [d for a, d in zip(angs, dists) if a < 0.35 or a > 6.1]
                if frontal:
                    print(min(frontal))
                    exit()

except KeyboardInterrupt:
    ser.close()