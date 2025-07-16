import 'package:flutter/material.dart';
import 'package:flutter/cupertino.dart';
import 'package:meditate/app/breathing_control.dart';
import 'package:shared_preferences/shared_preferences.dart';

class MeditationSettingsScreen extends StatefulWidget {
  @override
  _MeditationSettingsScreenState createState() =>
      _MeditationSettingsScreenState();
}

class _MeditationSettingsScreenState extends State<MeditationSettingsScreen> {
  int sessionDuration = 5;
  List<String> soundOptions = [
    //'Calming Music',
    'Morning', 'Night', '',
  ];
  final backgroundSounds = [
    {'emoji': '🌞', 'label': 'Morning', 'sound': 'morning'},
    {'emoji': '🌙', 'label': 'Night', 'sound': 'night'},
    {'emoji': '💤', 'label': 'Delta (Sleep, Healing)', 'sound': 'delta_beat'},
    {
      'emoji': '🧘',
      'label': 'Theta (Meditation, Creativity)',
      'sound': 'theta_beat',
    },
    {
      'emoji': '🌿',
      'label': 'Alpha (Relaxation, Focus)',
      'sound': 'alpha_beat',
    },
    {
      'emoji': '🔥',
      'label': 'Beta (Alertness, Activity)',
      'sound': 'beta_beat',
    },
    {
      'emoji': '⚡',
      'label': 'Gamma (Cognition, Awareness)',
      'sound': 'gamma_beat',
    },
  ];
  String selectedSound = 'night';

  List<String> breathingTechniques = [
    '4-7-8',
    'Box Breathing',
    'Pursed Lip Breathing',
    'Custom',
  ];
  String selectedTechnique = '4-7-8';

  int customInhaleTime = 0;
  int customHoldTime = 0;
  int customExhaleTime = 0;
  int customEndHoldTime = 0;
  bool calibrationComplete = false;
  bool showPhase = false;

  late FixedExtentScrollController _scrollController;

  Stopwatch stopwatch = Stopwatch();
  String currentPhase = 'Calibrate'; // Tracks which phase the user is measuring

  late final Map<String, List<int>> techniqueDurations = {
    '4-7-8': [4, 7, 8],
    'Box Breathing': [4, 4, 4],
    'Pursed Lip Breathing': [3, 2, 4],
    'Custom': <int>[
      customInhaleTime,
      customHoldTime,
      customExhaleTime,
      customEndHoldTime,
    ],
  };

  bool enableAudioCue = true;
  bool enableHapticFeedback = true;

  /*
     child: CupertinoPicker(
     itemExtent: 40,
     scrollController: FixedExtentScrollController(initialItem: sessionDuration - 1),
     onSelectedItemChanged: (index) {
     setState(() {
     sessionDuration = index + 1;
     });
     },
     children: List.generate(15, (index) => Text('${index + 1} min')),
     ),
     */

  @override
  void initState() {
    super.initState();
    _scrollController = FixedExtentScrollController(
      initialItem: sessionDuration - 1,
    );
    _loadPreferences();
  }

  Future<void> _savePreferences() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('selectedTechnique', selectedTechnique);
    await prefs.setString('selectedSound', selectedSound);
    await prefs.setInt('sessionDuration', sessionDuration);
    await prefs.setInt('customInhaleTime', customInhaleTime);
    await prefs.setInt('customHoldTime', customHoldTime);
    await prefs.setInt('customExhaleTime', customExhaleTime);
    await prefs.setInt('customEndHoldTime', customEndHoldTime);
    await prefs.setBool('enableAudioCue', enableAudioCue);
    await prefs.setBool('enableHapticFeedback', enableHapticFeedback);
  }

  Future<void> _loadPreferences() async {
    final prefs = await SharedPreferences.getInstance();
    setState(() {
      selectedTechnique = prefs.getString('selectedTechnique') ?? '4-7-8';

      final loadedSound = prefs.getString('selectedSound') ?? 'morning';
      final validSounds = backgroundSounds.map((e) => e['sound']).toSet();
      selectedSound =
          validSounds.contains(loadedSound) ? loadedSound : 'morning';
      sessionDuration = prefs.getInt('sessionDuration') ?? 5;
      //print('Loaded -> $sessionDuration');
      customInhaleTime = prefs.getInt('customInhaleTime') ?? 0;
      customHoldTime = prefs.getInt('customHoldTime') ?? 0;
      customExhaleTime = prefs.getInt('customExhaleTime') ?? 0;
      customEndHoldTime = prefs.getInt('customEndHoldTime') ?? 0;

      enableAudioCue = prefs.getBool('enableAudioCue') ?? true;
      enableHapticFeedback = prefs.getBool('enableHapticFeedback') ?? true;

      _scrollController.jumpToItem(sessionDuration - 1);
      if (customInhaleTime != 0 ||
          customHoldTime != 0 ||
          customExhaleTime != 0) {
        calibrationComplete = true;
        currentPhase = 'Done';
      }
    });
  }

  Widget _buildCustomBreathingTracker() {
    if (selectedTechnique != 'Custom') return SizedBox.shrink();

    void _updateSelectedTechnique(String newValue) {
      setState(() {
        selectedTechnique = newValue;
        _savePreferences();
      });
    }

    void _updateSelectedSound(String newValue) {
      setState(() {
        selectedSound = newValue;
        _savePreferences();
      });
    }

    void _updateSessionDuration(int newValue) {
      setState(() {
        sessionDuration = newValue;
        _savePreferences();
      });
    }

    String displayTime() {
      final seconds = stopwatch.elapsed.inSeconds;
      return '$seconds sec';
    }

    void handlePhase() {
      setState(() {
        if (currentPhase == 'Calibrate') {
          stopwatch.reset();
          stopwatch.start();
          currentPhase = 'Inhale';
          showPhase = true;
        } else if (currentPhase == 'Inhale') {
          customInhaleTime = stopwatch.elapsed.inSeconds;
          stopwatch.reset();
          stopwatch.start();
          currentPhase = 'Hold';
        } else if (currentPhase == 'Hold') {
          customHoldTime = stopwatch.elapsed.inSeconds;
          stopwatch.reset();
          stopwatch.start();
          currentPhase = 'Exhale';
        } else if (currentPhase == 'Exhale') {
          customExhaleTime = stopwatch.elapsed.inSeconds;
          stopwatch.reset();
          stopwatch.start();
          currentPhase = 'End Hold';
          _savePreferences();
        } else if (currentPhase == 'End Hold') {
          customEndHoldTime = stopwatch.elapsed.inSeconds;
          stopwatch.stop();
          calibrationComplete = true;
          showPhase = false;
          currentPhase = 'Done';
          _savePreferences();
        }
      });
    }

    return Column(
      children: [
        if (calibrationComplete)
          Text(calibrationComplete ? 'Custom Breath Timings' : currentPhase),
        if (!calibrationComplete)
          ElevatedButton(
            onPressed:
                currentPhase == 'Done' && !calibrationComplete
                    ? null
                    : handlePhase,
            child: Text(currentPhase),
          ),
        if (currentPhase == 'Done' && calibrationComplete) ...[
          Text('Inhale: $customInhaleTime sec'),
          Text('Hold: $customHoldTime sec'),
          Text('Exhale: $customExhaleTime sec'),
          Text('End Hold: $customEndHoldTime sec'),
          ElevatedButton(
            style: ElevatedButton.styleFrom(
              backgroundColor: Colors.deepOrangeAccent.shade200,
              foregroundColor: Colors.white,
            ),
            onPressed:
                () => setState(() {
                  currentPhase = 'Calibrate';
                  customInhaleTime = 0;
                  customHoldTime = 0;
                  customExhaleTime = 0;
                  calibrationComplete = false;
                  showPhase = false;
                  stopwatch.reset();
                }),
            child: Text('Redo Calibration', style: TextStyle(fontSize: 11)),
          ),
        ],
      ],
    );
  }

  Widget _buildBreathingTechniqueDropdown() {
    return Container(
      padding: EdgeInsets.symmetric(horizontal: 16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.grey.shade50, width: 1),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.1),
            blurRadius: 8,
            offset: Offset(0, 4),
          ),
        ],
      ),
      child: DropdownButtonHideUnderline(
        child: DropdownButton<String>(
          value: selectedTechnique,
          icon: Icon(Icons.arrow_drop_down, color: Colors.grey),
          dropdownColor: Colors.white,
          style: TextStyle(color: Colors.black, fontSize: 16),
          items:
              breathingTechniques.map((technique) {
                return DropdownMenuItem<String>(
                  value: technique,
                  child: Row(
                    children: [
                      Icon(
                        Icons.self_improvement,
                        color:
                            technique == '4-7-8'
                                ? Colors.blue
                                : technique == 'Box Breathing'
                                ? Colors.yellow
                                : technique == 'Pursed Lip Breathing'
                                ? Colors.pink
                                : Colors.green,
                      ),
                      SizedBox(width: 10),
                      Text(technique),
                    ],
                  ),
                );
              }).toList(),
          onChanged:
              (newValue) => setState(() => selectedTechnique = newValue!),
        ),
      ),
    );
  }

  Widget _buildSoundDropdown() {
    return Container(
      padding: EdgeInsets.symmetric(horizontal: 16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.grey.shade50, width: 1),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.1),
            blurRadius: 8,
            offset: Offset(0, 4),
          ),
        ],
      ),
      child: DropdownButtonHideUnderline(
        child: DropdownButton<String>(
          value: selectedSound,
          icon: Icon(Icons.arrow_drop_down, color: Colors.indigo),
          dropdownColor: Colors.white,
          style: TextStyle(color: Colors.black, fontSize: 16),
          items:
              backgroundSounds.map((item) {
                return DropdownMenuItem<String>(
                  value: item['sound'],
                  child: Row(
                    children: [
                      Text(item['emoji']!, style: TextStyle(fontSize: 20)),
                      SizedBox(width: 10),
                      Text(item['label']!),
                    ],
                  ),
                );
              }).toList(),
          onChanged: (newValue) {
            setState(() {
              selectedSound = newValue!;
              _savePreferences();
            });
          },
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      extendBodyBehindAppBar: true,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        title: Text(
          'Meditation Settings',
          style: TextStyle(color: Colors.black.withOpacity(0.7)),
        ),
        centerTitle: true,
      ),

      body: Container(
        width: double.infinity,
        height: double.infinity,
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
        child: SafeArea(
          child: LayoutBuilder(
            builder: (context, constraints) {
              return SingleChildScrollView(
                child: ConstrainedBox(
                  constraints: BoxConstraints(minHeight: constraints.maxHeight),
                  child: IntrinsicHeight(
                    child: Padding(
                      padding: const EdgeInsets.symmetric(
                        horizontal: 16,
                        vertical: 32,
                      ),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          Text(
                            'Select Breathing Technique',
                            style: TextStyle(fontSize: 18),
                          ),
                          _buildBreathingTechniqueDropdown(),
                          _buildCustomBreathingTracker(),
                          SizedBox(height: 10),
                          Text('Select Sound', style: TextStyle(fontSize: 18)),
                          _buildSoundDropdown(),
                          SizedBox(height: 10),
                          Text('Duration', style: TextStyle(fontSize: 18)),
                          SizedBox(
                            height: 60,
                            child: CupertinoPicker(
                              itemExtent: 35,
                              scrollController:
                                  _scrollController, //FixedExtentScrollController(initialItem: sessionDuration - 1),
                              onSelectedItemChanged: (index) {
                                setState(() => sessionDuration = index + 1);
                              },
                              children: List.generate(
                                15,
                                (index) => Center(
                                  child: Text(
                                    '${index + 1} min',
                                    style: TextStyle(
                                      fontSize: 18,
                                      color: Colors.deepOrange,
                                    ),
                                  ),
                                ),
                              ),
                            ),
                          ),
                          SwitchListTile(
                            title: Text('Bells'),
                            value: enableAudioCue,
                            onChanged: (value) {
                              setState(() {
                                enableAudioCue = value;
                              });
                            },
                          ),
                          SwitchListTile(
                            title: Text('Haptic Feedback'),
                            value: enableHapticFeedback,
                            onChanged: (value) {
                              setState(() {
                                enableHapticFeedback = value;
                              });
                            },
                          ),
                          SizedBox(height: 10),
                          ElevatedButton(
                            style: ElevatedButton.styleFrom(
                              backgroundColor: Colors.green.shade200,
                              foregroundColor: Colors.black,
                            ),
                            onPressed: () {
                              _savePreferences();
                              final durations =
                                  techniqueDurations[selectedTechnique]!;
                              final breathDuration = durations.reduce(
                                (a, b) => a + b,
                              );
                              print("Breath durations ${breathDuration}");
                              if (durations.reduce((a, b) => a + b) < 4) {
                                ScaffoldMessenger.of(context).showSnackBar(
                                  SnackBar(
                                    content: Text('Not enough breath time'),
                                  ),
                                );
                              } else {
                                Navigator.push(
                                  context,
                                  MaterialPageRoute(
                                    builder:
                                        (context) => BreathControlScreen(
                                          sessionDuration: sessionDuration,
                                          selectedSound: selectedSound,
                                          inhaleDuration: durations[0],
                                          holdDuration: durations[1],
                                          exhaleDuration: durations[2],
                                          endHoldDuration:
                                              durations.length > 3
                                                  ? durations[3]
                                                  : 0,
                                          enableAudioCue: enableAudioCue,
                                          enableHapticFeedback:
                                              enableHapticFeedback,
                                        ),
                                  ),
                                );
                              }
                            },
                            child: Text(
                              'Start Session',
                              style: TextStyle(fontSize: 16),
                            ),
                          ),
                          SizedBox(height: 10),
                          ElevatedButton(
                            style: ElevatedButton.styleFrom(
                              backgroundColor: Colors.indigo.shade200,
                              foregroundColor: Colors.black,
                            ),
                            onPressed: () {
                              _savePreferences();
                              ScaffoldMessenger.of(context).showSnackBar(
                                SnackBar(content: Text('Settings Saved')),
                              );
                            },
                            child: Text(
                              'Save Settings',
                              style: TextStyle(fontSize: 16),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              );
            },
          ),
        ),
      ),
    );
  }
}
