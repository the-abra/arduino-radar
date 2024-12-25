#include <Servo.h>  // Include the Servo library

// Define pins for the HC-SR04 sensor
const int trigPin = 3;  // Trig pin connected to D3
const int echoPin = 2;  // Echo pin connected to D2

// Define pin for the servo motor
const int servoPin = 9; // Yellow wire of the servo to D9

// Create a Servo object
Servo myServo;

// Variable to store the duration of the pulse
long duration;

// Variable to store the calculated distance
int distance;

void setup() {
  // Initialize serial communication
  Serial.begin(9600);

  // Set the trigPin as an OUTPUT and the echoPin as an INPUT
  pinMode(trigPin, OUTPUT);
  pinMode(echoPin, INPUT);

  // Attach the servo to the specified pin
  myServo.attach(servoPin);

  // Move servo to the initial position
  myServo.write(0);
  delay(500);
}

void loop() {
  // Rotate the servo from 0° to 180° and back
  for (int angle = 15; angle <= 165; angle += 2) {
    scanArea(angle);  // Perform a scan at the current angle
  }
  for (int angle = 165; angle >= 15; angle -= 2) {
    scanArea(angle);  // Perform a scan at the current angle
  }
}

void scanArea(int angle) {
  // Rotate servo to the specified angle
  myServo.write(angle);
  delay(50);  // Allow time for the servo to reach the position

  // Measure the distance
  distance = measureDistance(trigPin, echoPin);

  // Print the angle and distance to the Serial Monitor
  Serial.print("Angle: ");
  Serial.print(angle);
  Serial.print("°, Distance: ");
  if (distance > 400 || distance < 2) {
    Serial.println("Out of range");  // If the distance is beyond sensor range
  } else {
    Serial.print(distance);
    Serial.println(" cm");
  }
}

int measureDistance(int trigPin, int echoPin) {
  long duration;

  // Clear the trigPin
  digitalWrite(trigPin, LOW);
  delayMicroseconds(2);

  // Trigger the sensor with a 10us pulse
  digitalWrite(trigPin, HIGH);
  delayMicroseconds(10);
  digitalWrite(trigPin, LOW);

  // Measure the echo pulse duration
  duration = pulseIn(echoPin, HIGH);

  // Calculate the distance
  int distance = duration * 0.0343 / 2;

  return distance;
}
