import 'package:flutter/material.dart';

class MarqueeText extends StatefulWidget {
  final String text;
  const MarqueeText({required this.text});

  @override
  _MarqueeTextState createState() => _MarqueeTextState();
}

class _MarqueeTextState extends State<MarqueeText>
    with SingleTickerProviderStateMixin {
  late final ScrollController _scrollController;
  late final AnimationController _controller;
  late final double _scrollSpeed;

  @override
  void initState() {
    super.initState();
    _scrollController = ScrollController();
    _controller = AnimationController(
      vsync: this,
      duration: Duration(seconds: 30),
    );

    _scrollSpeed = 50.0;

    _controller.addListener(() {
      _scrollController.jumpTo(
        _controller.value * _scrollController.position.maxScrollExtent,
      );
    });

    WidgetsBinding.instance.addPostFrameCallback((_) {
      _controller.repeat();
    });
  }

  @override
  void dispose() {
    _controller.dispose();
    _scrollController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      scrollDirection: Axis.horizontal,
      controller: _scrollController,
      child: Text(widget.text, style: TextStyle(fontSize: 20)),
    );
  }
}
