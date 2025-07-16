import 'dart:convert';

class Profile {
  String name;
  String technique;
  int duration;
  String sound;
  int inhaleTime;
  int holdTime;
  int exhaleTime;
  List<Map<String, String>> reminders; // Store time and message

  Profile({
    required this.name,
    required this.technique,
    required this.duration,
    required this.sound,
    required this.inhaleTime,
    required this.holdTime,
    required this.exhaleTime,
    required this.reminders,
  });

  Map<String, dynamic> toJson() => {
        'name': name,
        'technique': technique,
        'duration': duration,
        'sound': sound,
        'inhaleTime': inhaleTime,
        'holdTime': holdTime,
        'exhaleTime': exhaleTime,
        'reminders': reminders,
      };

  factory Profile.fromJson(Map<String, dynamic> json) => Profile(
        name: json['name'],
        technique: json['technique'],
        duration: json['duration'],
        sound: json['sound'],
        inhaleTime: json['inhaleTime'],
        holdTime: json['holdTime'],
        exhaleTime: json['exhaleTime'],
        reminders: List<Map<String, String>>.from(json['reminders']),
      );
}
