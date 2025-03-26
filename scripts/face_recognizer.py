import face_recognition
import cv2
import os

REFERENCE_IMAGE_PATH = os.path.join("dataset", "faces", "hector.jpg")

# Cargar imagen de referencia
reference_image = face_recognition.load_image_file(REFERENCE_IMAGE_PATH)
reference_encodings = face_recognition.face_encodings(reference_image)

if len(reference_encodings) == 0:
    raise ValueError("⚠️ No se detectó ninguna cara en la imagen de referencia.")

reference_encoding = reference_encodings[0]

# Cámara
video_capture = cv2.VideoCapture(0)
print("[INFO] Escaneando... pulsa 'q' para salir.")

while True:
    ret, frame = video_capture.read()
    if not ret:
        break

    small_frame = cv2.resize(frame, (0, 0), fx=0.25, fy=0.25)
    rgb_small_frame = small_frame[:, :, ::-1]

    # Detectar caras
    face_locations = face_recognition.face_locations(rgb_small_frame)

    # Solo si se detectan caras
    if face_locations:
        face_encodings = face_recognition.face_encodings(rgb_small_frame, face_locations)

        for (top, right, bottom, left), face_encoding in zip(face_locations, face_encodings):
            match = face_recognition.compare_faces([reference_encoding], face_encoding)[0]
            name = "Héctor" if match else "Desconocido"

            # Volver a escalar coordenadas
            top *= 4
            right *= 4
            bottom *= 4
            left *= 4

            color = (0, 255, 0) if match else (0, 0, 255)
            cv2.rectangle(frame, (left, top), (right, bottom), color, 2)
            cv2.putText(frame, name, (left, top - 10), cv2.FONT_HERSHEY_SIMPLEX, 0.9, (255, 255, 255), 2)

    cv2.imshow('Face Recognition', frame)

    if cv2.waitKey(1) & 0xFF == ord('q'):
        break

video_capture.release()
cv2.destroyAllWindows()
