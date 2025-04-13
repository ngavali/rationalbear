import 'dart:async';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:meditate/app/ripple_circle.dart';
import 'package:audioplayers/audioplayers.dart';

class BreathControlScreen extends StatefulWidget {
  final int sessionDuration;
  final String selectedSound;
  final int inhaleDuration;
  final int holdDuration;
  final int exhaleDuration;

  const BreathControlScreen({
    Key? key,
    required this.sessionDuration,
    required this.selectedSound,
    required this.inhaleDuration,
    required this.holdDuration,
    required this.exhaleDuration,
  }) : super(key: key);

  @override
  _BreathControlScreenState createState() => _BreathControlScreenState();
}

class _BreathControlScreenState extends State<BreathControlScreen>
    with SingleTickerProviderStateMixin {
  late int inhaleDuration;
  late int holdDuration;
  late int exhaleDuration;
  int currentPhase = 0;
  int remainingTime = 0;
  double lastSize = 150;
  Timer? timer;
  Timer? sessionTimer;
  List<Widget> ripples = [];
  bool isEnding = false;
  bool showFinishedText = false;
  int actualMeditationTime = 0;
  final AudioPlayer _audioPlayer = AudioPlayer();

  @override
  void initState() {
    super.initState();
    inhaleDuration = widget.inhaleDuration;
    holdDuration = widget.holdDuration;
    exhaleDuration = widget.exhaleDuration;
    startBreathingCycle();
    generateRipples();
    startSessionTimer();
    _playSelectedSound();
  }

  void _playCalmingSound() async {
    await _audioPlayer.play(AssetSource('chirping-04.mp4'));
    _audioPlayer.setReleaseMode(ReleaseMode.loop); // Loop the sound
    _audioPlayer.setVolume(1.0);
  }

  void _playSelectedSound() async {
    String soundFile =
        'music/${widget.selectedSound.toLowerCase().replaceAll(' ', '_')}.mp4';
    await _audioPlayer.play(AssetSource(soundFile));
    _audioPlayer.setReleaseMode(ReleaseMode.loop); // Loop the sound
  }

  void startBreathingCycle() {
    setState(() {
      currentPhase = 0;
      remainingTime = inhaleDuration;
    });
    timer = Timer.periodic(Duration(seconds: 1), (Timer t) {
      if (remainingTime > 0) {
        setState(() {
          remainingTime--;
        });
      } else {
        switchPhase();
      }
    });
  }

  void startSessionTimer() {
    sessionTimer = Timer.periodic(Duration(seconds: 1), (timer) {
      actualMeditationTime++;
      if (actualMeditationTime >= widget.sessionDuration * 60) {
        stopBreathingCycle();
      }
    });
  }

  void generateRipples() {
    Timer.periodic(Duration(seconds: 1), (timer) {
      if (mounted && !isEnding) {
        setState(() {
          double baseSize = 150;
          double expandedSize = 250; // Reduced from 250 to 200
          double size = lastSize;
          if (currentPhase == 0) {
            size =
                baseSize +
                (expandedSize - baseSize) *
                    (1 - remainingTime / inhaleDuration);
          } else if (currentPhase == 2) {
            size =
                expandedSize -
                (expandedSize - baseSize) *
                    (1 - remainingTime / exhaleDuration);
          }
          lastSize = size;
          ripples.add(
            RippleCircle(
              color: getPhaseColor(),
              size: lastSize,
              isEnding: isEnding,
            ),
          );
        });
      }
    });
  }

  void switchPhase() {
    setState(() {
      if (currentPhase == 0) {
        currentPhase = 1;
        remainingTime = holdDuration;
      } else if (currentPhase == 1) {
        currentPhase = 2;
        remainingTime = exhaleDuration;
      } else {
        currentPhase = 0;
        remainingTime = inhaleDuration;
      }
    });
  }

  Future<void> _saveSession() async {
    final prefs = await SharedPreferences.getInstance();
    final minutes = (actualMeditationTime ~/ 60).toString().padLeft(2, '0');
    final seconds = (actualMeditationTime % 60).toString().padLeft(2, '0');
    final sessionTime = '${minutes}m${seconds}s';
    final timestamp = DateTime.now();
    final formattedTime =
        '${timestamp.year}-${timestamp.month.toString().padLeft(2, '0')}-${timestamp.day.toString().padLeft(2, '0')} '
        '${timestamp.hour.toString().padLeft(2, '0')}:${timestamp.minute.toString().padLeft(2, '0')}';

    List<String> sessions = prefs.getStringList('lastSessions') ?? [];
    if (sessions.length >= 3) {
      sessions.removeAt(0);
    }
    sessions.add('$sessionTime on $formattedTime');
    await prefs.setStringList('lastSessions', sessions);
  }

  Future<void> _updateProgress(int sessionDuration) async {
    final prefs = await SharedPreferences.getInstance();

    // Load or Initialize Daily Data
    Map<String, int> dailyData = {};
    if (prefs.getString('dailyData') != null) {
      dailyData = Map<String, int>.from(
        jsonDecode(prefs.getString('dailyData')!),
      );
    }

    // Update Daily Data
    String today = DateTime.now().toIso8601String().split('T')[0];
    dailyData[today] = (dailyData[today] ?? 0) + sessionDuration;
    await prefs.setString('dailyData', jsonEncode(dailyData));

    print('Daily Data Updated: $dailyData');
  }

  void stopBreathingCycle() {
    timer?.cancel();
    sessionTimer?.cancel();
    _audioPlayer.stop();
    setState(() {
      isEnding = true;
      showFinishedText = true;
    });
    _saveSession();
    _updateProgress(actualMeditationTime);

    if (actualMeditationTime >= widget.sessionDuration * 60) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Kudos! You Made It through entire Session! 🎉'),
        ),
      );
    }

    Future.delayed(Duration(seconds: 3), () {
      Navigator.popUntil(
        context,
        (route) => route.isFirst,
      ); //Return to main screen
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            'You meditated for ${actualMeditationTime ~/ 60} min ${actualMeditationTime % 60} sec',
          ),
        ),
      );
    });
  }

  @override
  void dispose() {
    timer?.cancel();
    sessionTimer?.cancel();
    _audioPlayer.dispose();
    super.dispose();
  }

  Color getPhaseColor() {
    return currentPhase == 0
        ? Colors.red
        : currentPhase == 1
        ? Colors.black
        : Colors.pink;
  }

  @override
  Widget build(BuildContext context) {
    String phaseText =
        showFinishedText
            ? 'Meditation Over'
            : currentPhase == 0
            ? 'Inhale'
            : currentPhase == 1
            ? 'Hold'
            : 'Exhale';
    Color phaseColor = getPhaseColor();

    double baseSize = 150;
    double expandedSize = 250;
    double size = lastSize;
    if (currentPhase == 0) {
      size =
          baseSize +
          (expandedSize - baseSize) * (1 - remainingTime / inhaleDuration);
    } else if (currentPhase == 2) {
      size =
          expandedSize -
          (expandedSize - baseSize) * (1 - remainingTime / exhaleDuration);
    }
    lastSize = size;

    return Scaffold(
      extendBodyBehindAppBar: true,
      appBar: AppBar(
        automaticallyImplyLeading: false,
        backgroundColor: Colors.transparent,
        elevation: 0,
        title: Text(
          'Breath Control',
          style: TextStyle(color: Colors.black.withOpacity(0.7)),
        ),
        centerTitle: true,
      ),
      body: Stack(
        alignment: Alignment.center,
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
          Positioned(
            left: 0,
            right: 0,
            bottom: 0,
            child: LinearProgressIndicator(
              value: 1 - actualMeditationTime / (widget.sessionDuration * 60),
              backgroundColor: getPhaseColor().withOpacity(0.2),
              color: getPhaseColor(),
              minHeight: 2,
            ),
          ),
          Align(
            alignment: Alignment(0, -0.6),
            child: Text(
              phaseText,
              style: TextStyle(
                fontSize: 32,
                fontWeight: FontWeight.bold,
                color: phaseColor,
              ),
            ),
          ),
          GestureDetector(
            onTap: stopBreathingCycle,
            child: Stack(
              alignment: Alignment.center,
              children: [
                AnimatedContainer(
                  duration: Duration(milliseconds: 500),
                  width: size,
                  height: size,
                  decoration: BoxDecoration(
                    color:
                        isEnding
                            ? Colors.transparent
                            : phaseColor.withOpacity(0.3),
                    shape: BoxShape.circle,
                  ),
                  child: Center(
                    child: Text(
                      '$remainingTime',
                      style: TextStyle(fontSize: 48, color: Colors.white),
                    ),
                  ),
                ),
                ...ripples,
              ],
            ),
          ),
        ],
      ),
    );
  }
}
