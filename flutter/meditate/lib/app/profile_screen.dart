import 'package:flutter/material.dart';
import 'package:meditate/app/meditation_settings.dart';
import 'profile.dart';
import 'profile_settings.dart';

class ProfileScreen extends StatefulWidget {
  @override
  _ProfileScreenState createState() => _ProfileScreenState();
}

class _ProfileScreenState extends State<ProfileScreen> {
  TextEditingController nameController = TextEditingController();

  void createProfile() {
  if (nameController.text.isEmpty) return;

  Profile newProfile = Profile(
    name: nameController.text,
    technique: '4-7-8',
    duration: 5,
    sound: 'Morning',
    inhaleTime: 0,
    holdTime: 0,
    exhaleTime: 0,
    reminders: [],
  );

  setState(() {
    profiles.add(newProfile);
    activeProfile = newProfile;
  });

  nameController.clear();

  // Navigate to meditation settings for the new profile
  Navigator.push(
    context,
    MaterialPageRoute(builder: (context) => MeditationSettingsScreen()),//profile: newProfile)),
  ).then((_) => saveProfiles());
}

  void editProfile(Profile profile) {
      Navigator.push(
        context,
        MaterialPageRoute(builder: (context) => MeditationSettingsScreen())//profile: profile)),
      ).then((_) => saveProfiles());
    }
 
  void deleteProfile(Profile profile) {
    setState(() {
      profiles.remove(profile);
      if (activeProfile == profile) activeProfile = profiles.isNotEmpty ? profiles.first : null;
      saveProfiles();
    });
  }

  void selectProfile(Profile profile) {
    setState(() {
      activeProfile = profile;
      saveProfiles();
    });
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Manage Profiles')),
      body: Column(
        children: [
          TextField(
            controller: nameController,
            decoration: InputDecoration(labelText: 'New Profile Name'),
          ),
          ElevatedButton(onPressed: createProfile, child: Text('Create Profile')),
          Expanded(
            child: ListView.builder(
              itemCount: profiles.length,
              itemBuilder: (context, index) {
                final profile = profiles[index];
                return ListTile(
                  title: Text(profile.name, style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
                  subtitle: Text(
                    'Technique: ${profile.technique} | Duration: ${profile.duration} min | Sound: ${profile.sound}',
                    style: TextStyle(fontSize: 14, color: Colors.grey[700]),
                  ),
                  onTap: () => editProfile(profile), // Opens Meditation Settings
                  trailing: IconButton(
                    icon: Icon(Icons.delete, color: Colors.red),
                    onPressed: () => deleteProfile(profile),
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

