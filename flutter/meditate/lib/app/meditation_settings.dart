import 'package:flutter/material.dart';
import 'package:flutter/cupertino.dart';
import 'package:meditate/app/breathing_control.dart';

class MeditationSettingsScreen extends StatefulWidget {
  @override
  _MeditationSettingsScreenState createState() => _MeditationSettingsScreenState();
}

class _MeditationSettingsScreenState extends State<MeditationSettingsScreen> {
  int sessionDuration = 5;
  List<String> soundOptions = ['Calming Music', 'Morning', 'Night'];
  String selectedSound = 'Morning';

  List<String> breathingTechniques = ['4-7-8', 'Box Breathing', 'Pursed Lip Breathing','Custom'];
  String selectedTechnique = '4-7-8';   

  late int customInhaleTime;
late int customHoldTime;
late int customExhaleTime;

Stopwatch stopwatch = Stopwatch();
String currentPhase = 'Inhale'; // Tracks which phase the user is measuring

  final Map<String, List<int>> techniqueDurations = {
    '4-7-8': [4, 7, 8],
    'Box Breathing': [4, 4, 4],
    'Pursed Lip Breathing': [3, 2, 4],
  };

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

Widget _buildCustomBreathingTracker() {
  if (selectedTechnique != 'Custom') return SizedBox.shrink();

  String displayTime() {
    final seconds = stopwatch.elapsed.inSeconds;
    return '$seconds sec';
  }

  void startTimer() {
    setState(() {
      stopwatch.reset();
      stopwatch.start();
    });
  }

  void stopTimer() {
    if (!stopwatch.isRunning) return; // Ensure it was running before stopping
    stopwatch.stop();
    final elapsedSeconds = stopwatch.elapsed.inSeconds;

    setState(() {
      if (currentPhase == 'Inhale') {
        customInhaleTime = elapsedSeconds;
        currentPhase = 'Hold';
      } else if (currentPhase == 'Hold') {
        customHoldTime = elapsedSeconds;
        currentPhase = 'Exhale';
      } else if (currentPhase == 'Exhale') {
        customExhaleTime = elapsedSeconds;
        currentPhase = 'Complete';
      }
      stopwatch.reset(); // Reset after stopping
    });
  }

  return Column(
    children: [
      Text('Current Phase: $currentPhase', style: TextStyle(fontSize: 20)),
      Text('Time: ${displayTime()}', style: TextStyle(fontSize: 24)),
      SizedBox(height: 20),
      ElevatedButton(
        onPressed: stopwatch.isRunning ? stopTimer : startTimer,
        child: Text(stopwatch.isRunning ? 'Stop' : 'Start'),
      ),
      if (currentPhase == 'Complete') ...[
        Text('Inhale: $customInhaleTime sec'),
        Text('Hold: $customHoldTime sec'),
        Text('Exhale: $customExhaleTime sec'),
        ElevatedButton(
          onPressed: () => setState(() {
            currentPhase = 'Inhale';
            customInhaleTime = 0;
            customHoldTime = 0;
            customExhaleTime = 0;
          }),
          child: Text('Redo Calibration'),
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
      border: Border.all(color: Colors.deepPurple.shade50, width: 1),
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
        icon: Icon(Icons.arrow_drop_down, color: Colors.blueAccent),
        dropdownColor: Colors.white,
        style: TextStyle(color: Colors.black, fontSize: 16),
        items: breathingTechniques.map((technique) {
          return DropdownMenuItem<String>(
            value: technique,
            child: Row(
              children: [
                Icon(Icons.self_improvement, color: Colors.deepPurpleAccent),
                SizedBox(width: 10),
                Text(technique),
              ],
            ),
          );
        }).toList(),
        onChanged: (newValue) => setState(() => selectedTechnique = newValue!),
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
      border: Border.all(color: Colors.deepPurple.shade50, width: 1),
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
        icon: Icon(Icons.arrow_drop_down, color: Colors.blueAccent),
        dropdownColor: Colors.white,
        style: TextStyle(color: Colors.black, fontSize: 16),
        items: soundOptions.map((sound) {
          return DropdownMenuItem<String>(
            value: sound,
            child: Row(
              children: [
                Icon(
                  sound == 'Calming Music' ? Icons.music_note :
                  sound == 'Morning' ? Icons.wb_sunny : Icons.nightlight_round,
                  color: Colors.deepPurpleAccent,
                ),
                SizedBox(width: 10),
                Text(sound),
              ],
            ),
          );
        }).toList(),
        onChanged: (newValue) { setState(() { selectedSound = newValue!; //_savePreferences(); 
        }); },
      ),
    ),
  );
}

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Meditation Settings'),
      ),
      body: Center(
        child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              _buildCustomBreathingTracker(),
              Text('Select Breathing Technique', style: TextStyle(fontSize: 18)),
              _buildBreathingTechniqueDropdown(),
              SizedBox(height: 24),
              Text('Select Sound', style: TextStyle(fontSize: 18)),
              _buildSoundDropdown(),
              SizedBox(height: 24),
              Text('Duration', style: TextStyle(fontSize: 18)),
              SizedBox(
                  height: 80,
                  child: CupertinoPicker(
                    itemExtent: 35,
                    scrollController: FixedExtentScrollController(initialItem: sessionDuration - 1),
                    onSelectedItemChanged: (index) {
                      setState(() => sessionDuration = index + 1);
                    },
                    children: List.generate(15, (index) => Center(
                        child: Text(
                            '${index + 1} min',
                          style: TextStyle(fontSize: 18, color: Colors.deepPurple),
                        )
                      )
                    ),
                  ),
              ),
              SizedBox(height: 24),
              ElevatedButton(
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.deepPurpleAccent,
                  foregroundColor: Colors.white,
                ), 
                onPressed: () {
                  final durations = techniqueDurations[selectedTechnique]!;
                  Navigator.push(
                    context,
                    MaterialPageRoute(builder: (context) => BreathControlScreen(
                      sessionDuration: sessionDuration,
                      selectedSound: selectedSound,
                      inhaleDuration: durations[0],
                      holdDuration: durations[1],
                      exhaleDuration: durations[2],
                    )),
                  );
                },
                child: Text('Start Session',
                    style: TextStyle(fontSize:18),
                ),
              ),
            
/*
    SizedBox(height: 24),
              Text('Breathing Technique:', style: TextStyle(fontSize: 18)),
              DropdownButton<String>(
                value: selectedTechnique,
                items: breathingTechniques.map((technique) {
                  return DropdownMenuItem<String>(
                    value: technique,
                    child: Text(technique),
                  );
                }).toList(),
                onChanged: (newValue) => setState(() => selectedTechnique = newValue!),
              ),
              Text('Sound:', style: TextStyle(fontSize: 18)),
              DropdownButton<String>(
                value: selectedSound,
                items: soundOptions.map((sound) {
                  return DropdownMenuItem<String>(
                    value: sound,
                    child: Text(sound),
                  );
                }).toList(),
                onChanged: (newValue) => setState(() => selectedSound = newValue!),
              ),
              GestureDetector(
                onTap: () => _showDurationPicker(context),
                child: Text(
                  'Meditation Time: $sessionDuration mins',
                  style: TextStyle(fontSize: 18, color: Colors.blue),
                ),
              ),
*/
              ],
          ),
      ),
    );
  }
}

/*
          Text('Meditation Time ($sessionDuration mins):', style: TextStyle(fontSize: 18)),
          Slider(
            value: sessionDuration.toDouble(),
            min: 1,
            max: 15,
            divisions: 14,
            label: '$sessionDuration Mins',
            onChanged: (value) {
              setState(() {
                sessionDuration = value.toInt();
              });
            },
          ),
          */
          
