import 'dart:async';
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:meditate/app/meditation_settings.dart';
import 'package:meditate/app/progress_screen.dart';
import 'package:meditate/app/reminder_screen.dart';
import 'package:meditate/app/notification_service.dart';
import 'package:meditate/app/permission_notify.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await NotificationService().initialize();
  runApp(const MeditationApp());
}

class MeditationApp extends StatelessWidget {
  const MeditationApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(
        fontFamily: 'FunelSans',
      ),
      debugShowCheckedModeBanner: false,
      home: HomeScreen(),
    );
  }
}

class HomeScreen extends StatefulWidget {
  @override
  _HomeScreenState createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  List<String> lastSessions = [];

  @override
  void initState() {
    super.initState();
    _loadLastSessions();
  }

  Future<void> _loadLastSessions() async {
    final prefs = await SharedPreferences.getInstance();
    setState(() {
      lastSessions = prefs.getStringList('lastSessions') ?? [];
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Good Life'),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                // Meditation Icon
                Column(
                  children: [
                     AnimatedIconButton(
                      icon:Icons.self_improvement,
                      color: Colors.deepPurple.shade400,
                      size: 60,
                      label: 'Meditate',
                      onPressed: () {
                        Navigator.push(
                          context,
                          MaterialPageRoute(builder: (context) => MeditationSettingsScreen()),
                        ).then((_) => _loadLastSessions());
                      },
                     ),
                  ],
                ),
                SizedBox(width: 20),
                // Progress Icon
                Column(
                  children: [
                     AnimatedIconButton(
                      icon: Icons.bar_chart,
                      color: Colors.lightGreen.shade200,
                      size: 60,
                      label: 'Progress',
                      onPressed: () {
                        Navigator.push(
                          context,
                          MaterialPageRoute(builder: (context) => ProgressScreen()),
                        );
                      },
                    ),
                  ],
                ),
                SizedBox(width: 20),
                // Reminder Icon
                Column(
                  children: [
                    AnimatedIconButton(
                      icon: Icons.notifications_active,
                      color: Colors.deepOrange.shade400,
                      size: 60,
                      label: 'Remind Me',
                      onPressed: () {
                        checkAndRequestPermission();
                        Navigator.push(
                          context,
                          MaterialPageRoute(builder: (context) => ReminderScreen()),
                        );
                      },
                    ),
                  ],
                ),
              ],
            ),
            SizedBox(height: 20),
            SizedBox(
              width: 300,
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                    Text(
                      'Past 3 Sessions',
                      style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                    ),
                    ...lastSessions.map((session) => Text(
                        session,
                        textAlign: TextAlign.left,
                        style: TextStyle(fontSize: 16),
                    )).toList(),
                ],
              ),
            ),
            ],
        ),
      ),
   );
  }
}

class AnimatedIconButton extends StatefulWidget {
  final IconData icon;
  final Color color;
  final double size;
  final String label;
  final VoidCallback onPressed;

  const AnimatedIconButton({
    Key? key,
    required this.icon,
    required this.color,
    required this.size,
    required this.label,
    required this.onPressed,
  }) : super(key: key);

  @override
  _AnimatedIconButtonState createState() => _AnimatedIconButtonState();
}

class _AnimatedIconButtonState extends State<AnimatedIconButton> {
  double _scale = 1.0;

  void _onTap() async {
    setState(() => _scale = 0.9);
    await Future.delayed(Duration(milliseconds: 150));
    setState(() => _scale = 1.0);
    widget.onPressed();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        GestureDetector(
          onTap: _onTap,
          child: AnimatedScale(
            scale: _scale,
            duration: Duration(milliseconds: 150),
            child: Icon(widget.icon, size: widget.size, color: widget.color),
          ),
        ),
        Text(widget.label, style: TextStyle(fontSize: 16)),
      ],
    );
  }
}

