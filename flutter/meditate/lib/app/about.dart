import 'dart:convert';
import 'package:flutter/services.dart';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:url_launcher/url_launcher.dart';
import 'package:package_info_plus/package_info_plus.dart';

//import 'package:network_info_plus/network_info_plus.dart';

class AboutScreen extends StatefulWidget {
  const AboutScreen({Key? key}) : super(key: key);

  @override
  _AboutScreenState createState() => _AboutScreenState();
}

class _AboutScreenState extends State<AboutScreen> {
  Map<String, dynamic>? aboutData;
  bool loading = true;
  bool error = false;

  final String jsonUrl = 'https://rbear.netlify.app/static/release-info.json';
  //     'http://192.168.1.108/release-info.json'; // 🔁 Replace with your URL

  String? currentVersion;
  int currentBuild = 0;
  String? latestVersion;
  int latestBuild = 0;
  String? downloadUrl;

  @override
  void initState() {
    super.initState();
    loadVersions();
    //    fetchAboutInfo();
  }

  Future<void> loadVersions() async {
    final info = await PackageInfo.fromPlatform();
    currentVersion = info.version;
    currentBuild = int.parse(info.buildNumber);

    final response = await http.get(Uri.parse(jsonUrl));
    if (response.statusCode == 200) {
      final data = json.decode(response.body);
      latestVersion = data['version'];
      latestBuild = data['build'] != null ? data['build'] : latestBuild;
      downloadUrl = data['downloadUrl'];
      aboutData = data;
      loading = false;
      setState(() {});
    } else {
      error = true;
    }
    //print("current version: $currentVersion+$currentBuild");
    //print("latest version: $latestVersion");
  }

  void _launchURL(String url) async {
    final uri = Uri.parse(url);

    if (!await launchUrl(uri, mode: LaunchMode.externalApplication)) {
      /*
      Clipboard.setData(ClipboardData(text: url));
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Unable to find app. Website copied to clipboard.'),
        ),
      );
      */
    } else if (url.startsWith('mailto:')) {
      Clipboard.setData(ClipboardData(text: url.replaceFirst('mailto:', '')));
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('Email copied to clipboard.')));
    } else {
      Clipboard.setData(ClipboardData(text: url));
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('No browser app found. Website copied to clipboard.'),
        ),
      );
    }
  }

  /*
  Future<void> fetchAboutInfo() async {
    try {
      //final info = NetworkInfo();
      //final wifiIP = await info.getWifiIP();
      //print('📡 URL: $jsonUrl');
      //print('📡 IP Address: $wifiIP');
      final response = await http.get(Uri.parse(jsonUrl));
      //print(response);
      if (!mounted) return;

      if (response.statusCode == 200) {
        setState(() {
          aboutData = json.decode(response.body);
          loading = false;
        });
      } else {
        if (!mounted) return;
        setState(() {
          error = true;
          loading = false;
        });
      }
    } catch (e) {
      if (!mounted) return;
      setState(() {
        error = true;
        loading = false;
      });
    }
  }
*/
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('About')),
      body: Center(
        child:
            loading
                ? CircularProgressIndicator()
                : error
                ? Text('Failed to load info')
                : Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: SingleChildScrollView(
                    padding: const EdgeInsets.all(16.0),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          aboutData!['appName'] ?? '',
                          style: Theme.of(context).textTheme.titleLarge,
                        ),
                        Text("Version: ${aboutData!['version'] ?? ''}"),
                        SizedBox(height: 12),
                        Text(aboutData!['description'] ?? ''),
                        if ((aboutData?['features'] as List?) != null)
                          Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              SizedBox(height: 20),
                              Text(
                                "Features:",
                                style: Theme.of(context).textTheme.titleMedium,
                              ),
                              SizedBox(height: 8),
                              ...List<Widget>.from(
                                (aboutData!['features'] as List).map(
                                  (feature) => Padding(
                                    padding: const EdgeInsets.symmetric(
                                      vertical: 2,
                                    ),
                                    child: Row(
                                      crossAxisAlignment:
                                          CrossAxisAlignment.start,
                                      children: [
                                        Text(
                                          "• ",
                                          style: TextStyle(
                                            fontSize: 16,
                                            color: Colors.blueGrey,
                                          ),
                                        ),
                                        Expanded(
                                          child: Text(
                                            feature,
                                            style: TextStyle(fontSize: 16),
                                          ),
                                        ),
                                      ],
                                    ),
                                  ),
                                ),
                              ),
                              SizedBox(height: 8),
                              Text(
                                "Somethine is missing! Feel free to reach me at my email id.",
                                style: Theme.of(context).textTheme.titleMedium,
                              ),
                            ],
                          ),
                        SizedBox(height: 8),
                        Text(
                          "Website:",
                          style: Theme.of(context).textTheme.titleMedium,
                        ),
                        GestureDetector(
                          onTap:
                              () => _launchURL(
                                'https://rbear.netlify.app/#random/timeandenergy',
                              ),
                          child: Text(
                            aboutData!['website'],
                            style: TextStyle(
                              color: Colors.blue,
                              decoration: TextDecoration.underline,
                            ),
                          ),
                        ),
                        SizedBox(height: 8),
                        Text(
                          "Contact:",
                          style: Theme.of(context).textTheme.titleMedium,
                        ),
                        GestureDetector(
                          onTap:
                              () => _launchURL(
                                "mailto:${aboutData!['contactEmail']}",
                              ),
                          child: Text(
                            aboutData!['contactEmail'],
                            style: TextStyle(
                              color: Colors.blue,
                              decoration: TextDecoration.underline,
                            ),
                          ),
                        ),
                        if (currentVersion != null &&
                            latestVersion != null &&
                            currentVersion != latestVersion &&
                            currentBuild < latestBuild &&
                            downloadUrl != null)
                          Padding(
                            padding: const EdgeInsets.only(top: 24.0),
                            child: ElevatedButton.icon(
                              icon: Icon(Icons.system_update),
                              label: Text('Download v$latestVersion'),
                              onPressed: () async {
                                final uri = Uri.parse(downloadUrl!);
                                if (await canLaunchUrl(uri)) {
                                  await launchUrl(
                                    uri,
                                    mode: LaunchMode.externalApplication,
                                  );
                                } else {
                                  ScaffoldMessenger.of(context).showSnackBar(
                                    SnackBar(
                                      content: Text(
                                        'Could not open update link',
                                      ),
                                    ),
                                  );
                                }
                              },
                            ),
                          ),
                        SizedBox(height: 20),
                      ],
                    ),
                  ),
                ),
      ),
    );
  }
}
