import 'package:flutter/material.dart';
import 'package:flutter/cupertino.dart';
import 'package:meditate/app/breathing_control.dart';
import 'package:shared_preferences/shared_preferences.dart';

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

    int customInhaleTime = 0;
    int customHoldTime = 0;
    int customExhaleTime = 0;
    bool calibrationComplete = false;
    bool showPhase = false;

late FixedExtentScrollController _scrollController;

    Stopwatch stopwatch = Stopwatch();
    String currentPhase = 'Start'; // Tracks which phase the user is measuring

  late final Map<String, List<int>> techniqueDurations = {
    '4-7-8': [4, 7, 8],
    'Box Breathing': [4, 4, 4],
    'Pursed Lip Breathing': [3, 2, 4],
    'Custom': <int>[customInhaleTime, customHoldTime, customExhaleTime],
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

@override
void initState() {
  super.initState();
  _scrollController = FixedExtentScrollController(initialItem: sessionDuration - 1);
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
}

Future<void> _loadPreferences() async {
  final prefs = await SharedPreferences.getInstance();
  setState(() {
    selectedTechnique = prefs.getString('selectedTechnique') ?? '4-7-8';
    selectedSound = prefs.getString('selectedSound') ?? 'Morning';
    sessionDuration = prefs.getInt('sessionDuration') ?? 5;
    print('Loaded -> $sessionDuration');
    customInhaleTime = prefs.getInt('customInhaleTime') ?? 0;
    customHoldTime = prefs.getInt('customHoldTime') ?? 0;
    customExhaleTime = prefs.getInt('customExhaleTime') ?? 0;
    _scrollController.jumpToItem(sessionDuration - 1);
    if (customInhaleTime!=0 || customHoldTime!= 0 || customExhaleTime!= 0) {
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
      if (currentPhase == 'Start') {
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
      if (calibrationComplete) Text(calibrationComplete ? 'Custom Breath Timings' : currentPhase),
      if (!calibrationComplete) ElevatedButton(
        onPressed: currentPhase == 'Done' && !calibrationComplete ? null : handlePhase,
        child: Text(currentPhase),
      ),
      if (currentPhase == 'Done' && calibrationComplete) ...[
        Text('Inhale: $customInhaleTime sec'),
        Text('Hold: $customHoldTime sec'),
        Text('Exhale: $customExhaleTime sec'),
        ElevatedButton(
          style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.deepOrangeAccent.shade200,
                  foregroundColor: Colors.white,
                ), 
          onPressed: () => setState(() {
            currentPhase = 'Start';
            customInhaleTime = 0;
            customHoldTime = 0;
            customExhaleTime = 0;
            calibrationComplete = false;
            showPhase = false;
            stopwatch.reset();
          }),
          child: Text('Redo Calibration',
                    style: TextStyle(fontSize:11),
            ),
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
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              Text('Select Breathing Technique', style: TextStyle(fontSize: 18)),
              _buildBreathingTechniqueDropdown(),
              _buildCustomBreathingTracker(),
              SizedBox(height: 10),
              Text('Select Sound', style: TextStyle(fontSize: 18)),
              _buildSoundDropdown(),
              SizedBox(height: 10),
              Text('Duration', style: TextStyle(fontSize: 18)),
              SizedBox(
                  height: 50,
                  child: CupertinoPicker(
                    itemExtent: 35,
                    scrollController: _scrollController,//FixedExtentScrollController(initialItem: sessionDuration - 1),
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
              SizedBox(height: 10),
              ElevatedButton(
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.deepPurpleAccent,
                  foregroundColor: Colors.white,
                ), 
                onPressed: () {
                  _savePreferences();
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
                    style: TextStyle(fontSize:16),
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
          
