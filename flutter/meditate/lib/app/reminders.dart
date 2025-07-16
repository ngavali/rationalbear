class Reminder {
  int id;
  String message;
  DateTime time;

  Reminder({required this.id, required this.message, required this.time});

  Map<String, dynamic> toMap() {
    return {
      'id': id,
      'message': message,
      'time': time.toIso8601String(),
    };
  }

  factory Reminder.fromMap(Map<String, dynamic> map) {
    return Reminder(
      id: map['id'],
      message: map['message'],
      time: DateTime.parse(map['time']),
    );
  }
}

