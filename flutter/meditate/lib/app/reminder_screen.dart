import 'package:flutter/material.dart';
import 'reminder_service.dart';
import 'reminders.dart';

class ReminderScreen extends StatefulWidget {
  @override
  _ReminderScreenState createState() => _ReminderScreenState();
}

class _ReminderScreenState extends State<ReminderScreen> {
  final ReminderService _reminderService = ReminderService();
  List<Reminder> _reminders = [];

  @override
  void initState() {
    super.initState();
    _loadReminders();
  }

  Future<void> _loadReminders() async {
    final reminders = await _reminderService.getReminders();
    setState(() {
      _reminders = reminders;
    });
  }

  Future<void> _createReminder() async {
    TimeOfDay? pickedTime = await showTimePicker(
      context: context,
      initialTime: TimeOfDay.now(),
    );

    if (pickedTime != null) {
      final now = DateTime.now();
      DateTime scheduledTime = DateTime(
        now.year,
        now.month,
        now.day,
        pickedTime.hour,
        pickedTime.minute,
      );
      if (scheduledTime.isBefore(DateTime.now())) {
        scheduledTime = scheduledTime.add(const Duration(days: 1));
      }
      final reminder = Reminder(
        id: (DateTime.now().millisecondsSinceEpoch % 2147483647).toInt(),
        message: 'Time to meditate!!!',
        time: scheduledTime,
      );

      await _reminderService.addReminder(reminder);
      _loadReminders();
    }
  }

  Future<void> _deleteReminder(int id) async {
    await _reminderService.deleteReminder(id);
    _loadReminders();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Manage Reminders')),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: ElevatedButton(
              onPressed: _createReminder,
              style: ElevatedButton.styleFrom(
                backgroundColor: Colors.blueGrey,
                foregroundColor: Colors.white,
              ),
              child: Text('Remind Me'),
            ),
          ),
          Expanded(
            child: ListView.builder(
              itemCount: _reminders.length,
              itemBuilder: (context, index) {
                final reminder = _reminders[index];
                final hR = reminder.time.hour.toString().padLeft(2, '0');
                final mN = reminder.time.minute.toString().padLeft(2, '0');
                return ListTile(
                  title: Text('Reminder at ${hR}:${mN} - ${reminder.message}'),
                  trailing: IconButton(
                    icon: Icon(Icons.delete, color: Colors.red),
                    onPressed: () => _deleteReminder(reminder.id),
                  ),
                  tileColor: Colors.grey[100],
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(12.0),
                  ),
                  contentPadding: EdgeInsets.symmetric(
                    horizontal: 16.0,
                    vertical: 8.0,
                  ),
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}
