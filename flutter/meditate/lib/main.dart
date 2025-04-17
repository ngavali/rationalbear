import 'dart:async';
import 'dart:math';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:meditate/app/meditation_settings.dart';
import 'package:meditate/app/progress_screen.dart';
import 'package:meditate/app/reminder_screen.dart';
import 'package:meditate/app/notification_service.dart';
import 'package:meditate/app/permission_notify.dart';
import 'package:meditate/app/about.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  final GlobalKey<NavigatorState> navigatorKey = GlobalKey<NavigatorState>();
  await NotificationService().initialize((payload) {
    // print("🧘 Recived notificatoin $payload, $navigatorKey");
    if (navigatorKey.currentState != null) {
      navigatorKey.currentState!.push(
        MaterialPageRoute(
          builder:
              (_) => MeditationSettingsScreen(), // Update with actual screen
        ),
      );
    }
  });
  runApp(const MeditationApp());
}

class MeditationApp extends StatelessWidget {
  const MeditationApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(fontFamily: 'FunelSans'),
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

  late Timer _timer;
  double _opacityTopLeft = 0.0;
  double _opacityBottomRight = 0.0;

  @override
  void initState() {
    super.initState();
    _loadLastSessions();
    _startSparkleAnimation();
  }

  Future<void> _loadLastSessions() async {
    final prefs = await SharedPreferences.getInstance();
    setState(() {
      lastSessions = prefs.getStringList('lastSessions') ?? [];
    });
  }

  void _startSparkleAnimation() {
    _timer = Timer.periodic(Duration(seconds: 3), (timer) {
      setState(() {
        _opacityTopLeft = Random().nextBool() ? 1.0 : 0.0;
        _opacityBottomRight = Random().nextBool() ? 1.0 : 0.0;
      });
    });
  }

  @override
  void dispose() {
    _timer.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      extendBodyBehindAppBar: true,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        title: Text(
          'Welcome to GoodLife!',
          style: TextStyle(color: Colors.black.withOpacity(0.7)),
        ),
        centerTitle: true,
      ),
      body: Stack(
        children: [
          Container(
            decoration: const BoxDecoration(
              gradient: LinearGradient(
                colors: [
                  Color(0xFFFFFFFF), // Very light pink/red
                  Color(0xFFFFFDE7), // Pale yellow
                  Color(0xFFE0F7FA), // Light cyan/blue
                  Color(0xFFE8F5E9), // Light green
                  Color(0xFFF8BBD0), // Soft pink
                ],
                begin: Alignment.topLeft,
                end: Alignment.bottomRight,
              ),
            ),
          ),
          Center(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    // Meditation Icon
                    AnimatedIconButton(
                      icon: Icons.self_improvement,
                      color: Colors.indigo,
                      size: 50,
                      label: 'Meditate',
                      onPressed: () {
                        Navigator.push(
                          context,
                          MaterialPageRoute(
                            builder: (context) => MeditationSettingsScreen(),
                          ),
                        ).then((_) => _loadLastSessions());
                      },
                    ),
                    SizedBox(width: 20),
                    /*
                AnimatedIconButton(
                  icon: Icons.account_circle,
                  color: Colors.blueGrey,
                  size: 60, // Use Material Icons
                  label: 'Settings',
                  onPressed:
                      () => Navigator.push(
                        context,
                        MaterialPageRoute(
                          builder: (context) => MeditationSettingsScreen(),
                        ),
                      ),
                ),
              ],
            ),
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                */
                    // Progress Icon
                    Column(
                      children: [
                        AnimatedIconButton(
                          icon: Icons.bar_chart,
                          color: Colors.lightGreen.shade200,
                          size: 50,
                          label: 'Progress',
                          onPressed: () {
                            Navigator.push(
                              context,
                              MaterialPageRoute(
                                builder: (context) => ProgressScreen(),
                              ),
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
                          color: Colors.pink.shade400,
                          size: 50,
                          label: 'Reminders',
                          onPressed: () {
                            checkAndRequestPermission();
                            Navigator.push(
                              context,
                              MaterialPageRoute(
                                builder: (context) => ReminderScreen(),
                              ),
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
                        style: TextStyle(
                          fontSize: 18,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      ...lastSessions
                          .map(
                            (session) => Text(
                              session,
                              textAlign: TextAlign.left,
                              style: TextStyle(fontSize: 16),
                            ),
                          )
                          .toList(),
                    ],
                  ),
                ),
                SizedBox(height: 20),
                Column(
                  children: [
                    AnimatedIconButton(
                      icon: Icons.info,
                      color: Colors.blue.shade200,
                      size: 50,
                      label: 'About',
                      onPressed: () {
                        Navigator.push(
                          context,
                          MaterialPageRoute(
                            builder: (context) => AboutScreen(),
                          ),
                        );
                      },
                    ),
                  ],
                ),
                /*
                SizedBox(height: 20),
                Text.rich(
                  TextSpan(
                    text: 'About',
                    style: TextStyle(
                      color: Colors.blue,
                      decoration: TextDecoration.underline,
                      fontWeight: FontWeight.bold,
                    ),
                    recognizer:
                        TapGestureRecognizer()
                          ..onTap = () {
                            Navigator.push(
                              context,
                              MaterialPageRoute(
                                builder: (_) => const AboutScreen(),
                              ),
                            );
                          },
                  ),
                ),
                */
              ],
            ),
          ),
        ],
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
