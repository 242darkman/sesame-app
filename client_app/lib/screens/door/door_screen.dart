import 'package:flutter/material.dart';

class DoorScreen extends StatelessWidget {
  const DoorScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Open Door')),
      body: const Center(
        child: Text('This is the Door screen'),
      ),
    );
  }
}
