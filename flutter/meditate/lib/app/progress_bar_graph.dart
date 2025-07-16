import 'dart:async';
import 'dart:convert';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';

class ProgressGraph extends StatefulWidget {
  final double maxTimeInMinutes; // Max time for the Y-axis

  const ProgressGraph({Key? key, this.maxTimeInMinutes = 15.0})
    : super(key: key);

  @override
  _ProgressGraphState createState() => _ProgressGraphState();
}

class _ProgressGraphState extends State<ProgressGraph> {
  Future<List<BarChartGroupData>> _loadBarData() async {
    final prefs = await SharedPreferences.getInstance();

    Map<String, int> dailyData = {};
    if (prefs.getString('dailyData') != null) {
      try {
        dailyData = Map<String, int>.from(
          jsonDecode(prefs.getString('dailyData')!),
        );
      } catch (e) {
        //print('Error decoding dailyData: $e');
      }
    }

    List<BarChartGroupData> data = [];
    List<String> last7Days = List.generate(7, (index) {
      return DateTime.now()
          .subtract(Duration(days: 6 - index))
          .toIso8601String()
          .split('T')[0];
    });

    for (int i = 0; i < 7; i++) {
      final day = last7Days[i];
      final value = dailyData[day]?.toDouble() ?? 0.0;
      data.add(
        BarChartGroupData(
          x: i,
          barRods: [
            BarChartRodData(
              toY: value / 60.0,
              width: 10,
              borderRadius: BorderRadius.circular(2),
              gradient: LinearGradient(
                colors: [Colors.lime, Colors.limeAccent],
              ),
              backDrawRodData: BackgroundBarChartRodData(
                show: true,
                toY: widget.maxTimeInMinutes,
                color: Colors.grey.shade300.withOpacity(0.2),
              ),
            ),
          ],
        ),
      );
    }
    return data;
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<List<BarChartGroupData>>(
      future: _loadBarData(),
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return Center(child: CircularProgressIndicator());
        }
        return BarChart(
          BarChartData(
            alignment: BarChartAlignment.spaceAround,
            maxY: widget.maxTimeInMinutes,
            barGroups: snapshot.data!,
            barTouchData: BarTouchData(
              touchTooltipData: BarTouchTooltipData(
                tooltipBgColor: Colors.blueAccent.withOpacity(0.8),
                getTooltipItem: (group, groupIndex, rod, rodIndex) {
                  return BarTooltipItem(
                    '${(rod.toY * 60).toInt()} seconds',
                    TextStyle(color: Colors.white, fontWeight: FontWeight.bold),
                  );
                },
              ),
            ),
            titlesData: FlTitlesData(
              leftTitles: AxisTitles(
                sideTitles: SideTitles(
                  showTitles: true,
                  interval: widget.maxTimeInMinutes / 5,
                  getTitlesWidget:
                      (value, _) => Text(
                        '${value.toInt()} min',
                        style: TextStyle(color: Colors.black54, fontSize: 12),
                      ),
                ),
              ),
              rightTitles: AxisTitles(
                sideTitles: SideTitles(
                  showTitles: false,
                ), // Hide numbers on the right side
              ),
              topTitles: AxisTitles(
                sideTitles: SideTitles(
                  showTitles: false,
                ), // Hide numbers on the top side
              ),
              bottomTitles: AxisTitles(
                sideTitles: SideTitles(
                  showTitles: true,
                  getTitlesWidget: (value, _) {
                    final last7Days = List.generate(7, (index) {
                      return DateTime.now()
                          .subtract(Duration(days: 6 - index))
                          .toIso8601String()
                          .split('T')[0];
                    });
                    return Text(
                      last7Days[value.toInt()].split('-').last,
                      style: TextStyle(color: Colors.black87, fontSize: 12),
                    );
                  },
                ),
              ),
            ),
            gridData: FlGridData(show: false, drawHorizontalLine: true),
            borderData: FlBorderData(show: false),
          ),
        );
      },
    );
  }
}
