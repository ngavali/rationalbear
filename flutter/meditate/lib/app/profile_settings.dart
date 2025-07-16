import 'package:shared_preferences/shared_preferences.dart';
import 'dart:convert';
import 'profile.dart';

List<Profile> profiles = [];
Profile? activeProfile;

Future<void> saveProfiles() async {
  final prefs = await SharedPreferences.getInstance();
  List<String> profileList = profiles.map((p) => jsonEncode(p.toJson())).toList();
  await prefs.setStringList('profiles', profileList);
  await prefs.setString('activeProfile', activeProfile?.name ?? '');
}

Future<void> loadProfiles() async {
  final prefs = await SharedPreferences.getInstance();
  List<String>? storedProfiles = prefs.getStringList('profiles');
  String activeProfileName = prefs.getString('activeProfile') ?? '';

  if (storedProfiles != null) {
    profiles = storedProfiles.map((p) => Profile.fromJson(jsonDecode(p))).toList();
    activeProfile = profiles.firstWhere((p) => p.name == activeProfileName, orElse: () => profiles.first);
  }
}

