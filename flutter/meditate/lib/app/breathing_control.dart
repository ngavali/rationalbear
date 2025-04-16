import 'dart:async';
import 'dart:convert';
import 'package:flutter/services.dart';
import 'package:flutter/material.dart';
import 'package:vibration/vibration.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:meditate/app/ripple_circle.dart';
import 'package:audioplayers/audioplayers.dart';

class BreathControlScreen extends StatefulWidget {
  final int sessionDuration;
  final String selectedSound;
  final int inhaleDuration;
  final int holdDuration;
  final int exhaleDuration;
  final int endHoldDuration;
  final bool enableAudioCue;
  final bool enableHapticFeedback;

  const BreathControlScreen({
    Key? key,
    required this.sessionDuration,
    required this.selectedSound,
    required this.inhaleDuration,
    required this.holdDuration,
    required this.exhaleDuration,
    this.endHoldDuration = 0,
    required this.enableAudioCue,
    required this.enableHapticFeedback,
  }) : super(key: key);

  @override
  _BreathControlScreenState createState() => _BreathControlScreenState();
}

class _BreathControlScreenState extends State<BreathControlScreen>
    with SingleTickerProviderStateMixin {
  late int inhaleDuration;
  late int holdDuration;
  late int exhaleDuration;
  late int endHoldDuration;

  late bool enableAudioCue;
  late bool enableHapticFeedback;

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
  final AudioPlayer _cuePlayer = AudioPlayer();

  String cueFile = 'music/Bell.mp3';

  int preSessionCountdown = 3;
  bool showPreCountdown = true;
  Timer? preCountdownTimer;

  double countdownOpacity = 1.0;
  String countdownText = "3";
  double countdownScale = 1.0;

  bool wasStoppedByUser = false;

  List<Color> currentGradient = [
    Color(0xFFFFFFFF),
    Color(0xFFFFFDE7),
    Color(0xFFE0F7FA),
  ];

  /*
  List<List<Color>> gradientPhases = [
    [Color(0xFFE1F5FE), Color(0xFFB3E5FC), Color(0xFF81D4FA)], // Inhale
    [Color(0xFFEEEEEE), Color(0xFFBDBDBD), Color(0xFF9E9E9E)], // Hold
    [Color(0xFFF8BBD0), Color(0xFFF48FB1), Color(0xFFF06292)], // Exhale
    [Color(0xFFE8F5E9), Color(0xFFC8E6C9), Color(0xFFA5D6A7)], // End Hold
  ];
  */

  List<List<Color>> gradientPhases = [
    [
      Color(0xFFE8F5E9),
      Color(0xFFC8E6C9),
      Color(0xFFA5D6A7),
    ], // 🌿 Inhale (green)
    [Color(0xFFEEEEEE), Color(0xFFBDBDBD), Color(0xFF9E9E9E)], // Hold (gray)
    [
      Color(0xFFEDE7F6),
      Color(0xFFD1C4E9),
      Color(0xFFB39DDB),
    ], // Exhale (lavender 💜)
    [
      Color(0xFFD7CCC8),
      Color(0xFFBCAAA4),
      Color(0xFFA1887F),
    ], // End Hold (neutral earth)
  ];

  final double baseSize = 150;
  final double expandedSize = 250;

  void updateGradientForPhase(int phase) {
    if (phase >= 0 && phase < gradientPhases.length) {
      setState(() {
        currentGradient = gradientPhases[phase];
      });
    }
  }

  bool shouldEndAfterCycle = false;
  bool showFinishingOverlay = false;

  @override
  void initState() {
    super.initState();
    inhaleDuration = widget.inhaleDuration;
    holdDuration = widget.holdDuration;
    exhaleDuration = widget.exhaleDuration;
    endHoldDuration = widget.endHoldDuration ?? 0;

    enableAudioCue = widget.enableAudioCue ?? true;
    enableHapticFeedback = widget.enableHapticFeedback ?? true;

    startPreCountdown();
    startSessionTimer();
  }

  void triggerHapticCue() async {
    //HapticFeedback.mediumImpact();
    if (await Vibration.hasVibrator() ?? false) {
      Vibration.vibrate(
        duration: 300,
      ); //, intensities: [240]); // 300ms vibration
    }
  }

  void triggerAudioCue() async {
    await _cuePlayer.play(AssetSource(cueFile), volume: 0.8);
    Future.delayed(Duration(milliseconds: 2500), () {
      _cuePlayer.stop();
    });
  }

  void startPreCountdown() {
    countdownText = "3";
    countdownOpacity = 1.0;
    countdownScale = 1.0;

    preCountdownTimer = Timer.periodic(const Duration(seconds: 1), (timer) {
      if (preSessionCountdown > 1) {
        setState(() {
          countdownScale = 1.5;
          countdownOpacity = 0.0;
        });

        Future.delayed(const Duration(milliseconds: 300), () {
          if (!mounted) return;
          setState(() {
            preSessionCountdown--;
            countdownText = '${preSessionCountdown}';
            countdownScale = 0.8;
            countdownOpacity = 1.0;
          });

          Future.delayed(const Duration(milliseconds: 200), () {
            if (!mounted) return;
            setState(() {
              countdownScale = 1.0;
            });
          });
        });
      } else {
        // Show "Let's Start"
        timer.cancel();

        // Start pop-out for "Let's Start"
        setState(() {
          countdownOpacity = 0.0;
          countdownScale = 1.5;
        });

        Future.delayed(const Duration(milliseconds: 300), () {
          if (!mounted) return;
          setState(() {
            countdownText = "Let's Start";
            countdownOpacity = 1.0;
            countdownScale = 0.8;
          });

          Future.delayed(const Duration(milliseconds: 100), () {
            if (!mounted) return;
            setState(() {
              countdownScale = 1.0;
            });
          });

          Future.delayed(const Duration(milliseconds: 500), () {
            if (!mounted) return;
            setState(() {
              countdownOpacity = 0.0;
              countdownScale = 1.5; // optional: a slight scale up while fading
            });

            Future.delayed(const Duration(milliseconds: 500), () {
              if (!mounted) return;
              setState(() {
                showPreCountdown = false;
              });
              startBreathingCycle();
            });
          });
        });
      }
    });
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
    generateRipples();
    _playSelectedSound();
    timer = Timer.periodic(Duration(seconds: 1), (Timer t) {
      if (remainingTime > 0) {
        setState(() {
          remainingTime--;
        });
      } else {
        switchPhase();
      }
    });
    if (enableAudioCue) triggerAudioCue();
    if (enableHapticFeedback) triggerHapticCue();
  }

  void startSessionTimer() {
    sessionTimer = Timer.periodic(Duration(seconds: 1), (timer) {
      actualMeditationTime++;
      if (actualMeditationTime >= widget.sessionDuration * 60) {
        shouldEndAfterCycle = true;
        //stopBreathingCycle();
      }
    });
  }

  void generateRipples() {
    Timer.periodic(Duration(seconds: 1), (timer) {
      if (inhaleDuration == 0 || exhaleDuration == 0) return;
      if (mounted && !isEnding && !showPreCountdown) {
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
      } else if (currentPhase == 2 && endHoldDuration != 0) {
        currentPhase = 3;
        remainingTime = endHoldDuration;
      } else {
        currentPhase = 0;
        remainingTime = inhaleDuration;
      }
      updateGradientForPhase(currentPhase);
      if (enableAudioCue) triggerAudioCue();
      if (enableHapticFeedback) triggerHapticCue();
      if (shouldEndAfterCycle && currentPhase == 0) {
        // Gracefully end after full cycle
        setState(() {
          showFinishingOverlay = true;
        });

        _audioPlayer.setVolume(0.2); // soften background music
        triggerOutroSound(); // soft chime

        Future.delayed(Duration(seconds: 3), () {
          stopBreathingCycle();
        });
      }
    });
  }

  void triggerOutroSound() async {
    final outroPlayer = AudioPlayer();
    await outroPlayer.play(AssetSource(cueFile), volume: 0.8);
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
      if (wasStoppedByUser) showFinishedText = true;
    });
    _saveSession();
    _updateProgress(actualMeditationTime);

    if (actualMeditationTime >= widget.sessionDuration * 60) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('Kudos! You made it! 🎉')));
    }

    Future.delayed(Duration(seconds: 2), () {
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
    preCountdownTimer?.cancel();
    timer?.cancel();
    sessionTimer?.cancel();
    _audioPlayer.dispose();
    super.dispose();
  }

  Color getPhaseColor() {
    switch (currentPhase) {
      case 0:
        return Colors.indigo;
      case 1:
        return Colors.black;
      case 2:
        return Colors.pink;
      case 3:
        return Colors.blueGrey; // end-hold
      default:
        return Colors.grey;
    }
  }

  @override
  Widget build(BuildContext context) {
    String phaseText =
        showFinishedText
            ? 'Sad to see you go'
            : currentPhase == 0
            ? 'Inhale'
            : currentPhase == 1
            ? 'Hold'
            : currentPhase == 2
            ? 'Exhale'
            : 'Hold';
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
          /*Container(
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
          ),*/
          if (showFinishingOverlay)
            Container(
              color: Colors.black.withOpacity(0.6),
              alignment: Alignment.center,
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Icon(Icons.bedtime, size: 48, color: Colors.white),
                  SizedBox(height: 12),
                  Text(
                    'Finishing Session...',
                    style: TextStyle(fontSize: 24, color: Colors.white70),
                  ),
                ],
              ),
            ),

          // 🔹 Everything else dims here
          AnimatedOpacity(
            duration: Duration(seconds: 2),
            opacity: showFinishingOverlay ? 0.0 : 1.0,
            child: Stack(
              alignment: Alignment.center,
              children: [
                AnimatedContainer(
                  duration: Duration(seconds: 2),
                  decoration: BoxDecoration(
                    gradient: LinearGradient(
                      colors: currentGradient,
                      begin: Alignment.topLeft,
                      end: Alignment.bottomRight,
                    ),
                  ),
                ),

                if (showPreCountdown)
                  AnimatedOpacity(
                    opacity: countdownOpacity,
                    duration: const Duration(milliseconds: 300),
                    child: AnimatedScale(
                      scale: countdownScale,
                      duration: const Duration(milliseconds: 300),
                      child: Text(
                        countdownText,
                        style: const TextStyle(
                          fontSize: 36,
                          fontWeight: FontWeight.bold,
                          color: Colors.black87,
                        ),
                      ),
                    ),
                  )
                else ...[
                  // Outer boundary ring
                  Container(
                    width: expandedSize,
                    height: expandedSize,
                    decoration: BoxDecoration(
                      shape: BoxShape.circle,
                      border: Border.all(
                        color: Colors.white.withOpacity(0.3),
                        width: 2,
                      ),
                    ),
                  ),

                  // Inner boundary ring
                  Container(
                    width: baseSize,
                    height: baseSize,
                    decoration: BoxDecoration(
                      shape: BoxShape.circle,
                      border: Border.all(
                        color: Colors.white.withOpacity(0.15),
                        width: 2,
                      ),
                    ),
                  ),

                  Positioned(
                    left: 0,
                    right: 0,
                    bottom: 0,
                    child: LinearProgressIndicator(
                      value:
                          1 -
                          actualMeditationTime / (widget.sessionDuration * 60),
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
                    onTap: () {
                      wasStoppedByUser = true;
                      stopBreathingCycle();
                    },
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
                              style: TextStyle(
                                fontSize: 48,
                                color: Colors.white,
                              ),
                            ),
                          ),
                        ),
                        ...ripples,
                      ],
                    ),
                  ),
                ],
              ],
            ),
          ),
        ],
      ),
    );
  }
}
