import 'package:permission_handler/permission_handler.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';

void checkAndRequestPermission() async {
  final flutterLocalNotificationsPlugin = FlutterLocalNotificationsPlugin();
  final AndroidFlutterLocalNotificationsPlugin? androidImplementation =
      flutterLocalNotificationsPlugin
          .resolvePlatformSpecificImplementation<
            AndroidFlutterLocalNotificationsPlugin
          >();

  final bool? granted =
      await androidImplementation?.requestNotificationsPermission();
  if (granted != null && granted) {
    //print('Notification permission granted!');
  } else {
    //print('Notification permission denied!');
  }
}
