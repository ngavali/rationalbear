import 'package:flutter/material.dart';

class RippleCircle extends StatefulWidget {
  final Color color;
  final double size;
  final bool isEnding;

  const RippleCircle({Key? key, required this.color, required this.size, required this.isEnding}) : super(key: key);

  @override
  _RippleCircleState createState() => _RippleCircleState();
}

class _RippleCircleState extends State<RippleCircle> with SingleTickerProviderStateMixin {
  late AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      duration: Duration(seconds: 3),
      vsync: this,
    )..forward().then((_) => mounted ? setState(() {}) : null);
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: _controller,
      builder: (context, child) {
        double rippleSize = widget.size + 250 * _controller.value;
        return Opacity(
          opacity: widget.isEnding ? 1.0 - _controller.value : 1.0 - _controller.value,
          child: Container(
            width: rippleSize,
            height: rippleSize,
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              color: widget.color.withOpacity(0.1),
            ),
          ),
        );
      },
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }
}
