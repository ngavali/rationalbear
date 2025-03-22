import 'package:flutter/material.dart';
import 'package:meditate/app/breathing_control.dart';

class MeditationSettingsScreen extends StatefulWidget {
  @override
  _MeditationSettingsScreenState createState() => _MeditationSettingsScreenState();
}

class _MeditationSettingsScreenState extends State<MeditationSettingsScreen> {
  int sessionDuration = 5; // default 5 minutes
  List<String> soundOptions = ['Calming Music', 'Morning', 'Night'];
  String selectedSound = 'Morning';

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Set Meditation Duration'),
      ),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
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
         Text('Select Sound:', style: TextStyle(fontSize: 18)),
          DropdownButton<String>(
            value: selectedSound,
            items: soundOptions.map((sound) {
              return DropdownMenuItem<String>(
                value: sound,
                child: Text(sound),
              );
            }).toList(),
            onChanged: (String? newValue) {
              setState(() {
                selectedSound = newValue!;
              });
            },
          ),
         ElevatedButton(
            onPressed: () {
              Navigator.push(
                context,
                MaterialPageRoute(builder: (context) => BreathControlScreen(sessionDuration: sessionDuration, selectedSound: selectedSound)),
              );
            },
            child: Text('Start Session'),
          ),
        ],
      ),
    );
  }
}
