import 'package:shared_preferences/shared_preferences.dart';
import 'dart:convert';
import 'notification_service.dart';
import 'reminders.dart';

class ReminderService {
  final NotificationService _notificationService = NotificationService();

  Future<void> addReminder(Reminder reminder) async {
    final prefs = await SharedPreferences.getInstance();
    List<String> reminders = prefs.getStringList('reminders') ?? [];
    reminders.add(jsonEncode(reminder.toMap()));
    await prefs.setStringList('reminders', reminders);
    await _notificationService.scheduleNotification(reminder.id, 'Meditation Reminder', reminder.message, reminder.time);
  }

  Future<List<Reminder>> getReminders() async {
    final prefs = await SharedPreferences.getInstance();
    List<String> reminders = prefs.getStringList('reminders') ?? [];
    return reminders.map((e) => Reminder.fromMap(jsonDecode(e))).toList();
  }

  Future<void> deleteReminder(int id) async {
    final prefs = await SharedPreferences.getInstance();
    List<Reminder> reminders = await getReminders();
    reminders.removeWhere((reminder) => reminder.id == id);
    await prefs.setStringList('reminders', reminders.map((e) => jsonEncode(e.toMap())).toList());
    await _notificationService.cancelNotification(id);
  }
}

