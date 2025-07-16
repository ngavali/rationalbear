import 'package:flutter/material.dart';

abstract class PhaseController {
  void setPhase(BreathingPhase phase);
  void schedulePhaseEnd(int duration);
  void triggerAudioCue();
  void triggerHapticCue();
}

abstract class BreathingPhase {
  final int duration;
  final String label;
  final Color color;

  BreathingPhase? next;

  BreathingPhase(this.duration, this.label, this.color);

  void onEnter(PhaseController controller) {
    controller.setPhase(this);
    controller.triggerAudioCue();
    controller.triggerHapticCue();
    controller.schedulePhaseEnd(duration);
  }
}

class InhalePhase extends BreathingPhase {
  InhalePhase(int duration) : super(duration, 'Inhale', Colors.green.shade400);
}

class HoldPhase extends BreathingPhase {
  HoldPhase(int duration) : super(duration, 'Hold', Colors.grey.shade800);
}

class ExhalePhase extends BreathingPhase {
  ExhalePhase(int duration) : super(duration, 'Exhale', Colors.purple.shade300);
}

class EndHoldPhase extends BreathingPhase {
  EndHoldPhase(int duration)
    : super(duration, 'End Hold', Colors.blueGrey.shade300);
}
