import 'package:flutter/material.dart';
import 'progress_bar_graph.dart';

class ProgressScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('My Progress'),
      ),
      body: Center(
        child: Container(
          height: 400,
          padding: EdgeInsets.all(16),
          child: ProgressGraph(maxTimeInMinutes: 15.0),
        ),
      ),
    );
  }
}
