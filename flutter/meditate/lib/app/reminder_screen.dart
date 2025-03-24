import 'package:flutter/material.dart';
import 'reminder_service.dart';
import 'reminders.dart';

class ReminderScreen extends StatefulWidget {
  @override
  _ReminderScreenState createState() => _ReminderScreenState();
}

class _ReminderScreenState extends State<ReminderScreen> {
  final ReminderService _reminderService = ReminderService();
  final TextEditingController _messageController = TextEditingController();
  TimeOfDay? _selectedTime;

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

  Future<void> _addReminder() async {
    if (_selectedTime == null || _messageController.text.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Please select time and enter a message')));
      return;
    }
    
    final now = DateTime.now();
    final scheduledTime = DateTime(now.year, now.month, now.day, _selectedTime!.hour, _selectedTime!.minute);
    final reminder = Reminder(
      id: (DateTime.now().millisecondsSinceEpoch % 2147483647).toInt(),
      message: _messageController.text,
      time: scheduledTime,
    );

    await _reminderService.addReminder(reminder);
    _messageController.clear();
    _selectedTime = null;
    _loadReminders();
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
            child: Column(
              children: [
                TextField(
                  controller: _messageController,
                  decoration: InputDecoration(labelText: 'Enter Reminder Message'),
                ),
                SizedBox(height: 10),
                Row(
                  children: [
                    Text(
                      _selectedTime == null 
                        ? 'No time selected' 
                        : 'Selected Time: ${_selectedTime!.format(context)}',
                    ),
                    Spacer(),
                    ElevatedButton(
                      onPressed: () async {
                        TimeOfDay? pickedTime = await showTimePicker(
                          context: context,
                          initialTime: TimeOfDay.now(),
                        );
                        if (pickedTime != null) {
                          setState(() {
                            _selectedTime = pickedTime;
                          });
                        }
                      },
                      child: Text('Pick Time'),
                    ),
                  ],
                ),
                SizedBox(height: 20),
                ElevatedButton(
                  onPressed: _addReminder,
                  child: Text('Add Reminder'),
                ),
              ],
            ),
          ),
          Expanded(
            child: ListView.builder(
              itemCount: _reminders.length,
              itemBuilder: (context, index) {
                final reminder = _reminders[index];
                return ListTile(
                  title: Text(reminder.message),
                  subtitle: Text('Time: ${reminder.time.hour}:${reminder.time.minute}'),
                  trailing: IconButton(
                    icon: Icon(Icons.delete, color: Colors.red),
                    onPressed: () => _deleteReminder(reminder.id),
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

