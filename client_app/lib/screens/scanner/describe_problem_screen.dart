import 'package:flutter/material.dart';

class DescribeProblemScreen extends StatelessWidget {
  const DescribeProblemScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Describe Problem')),
      body: const Center(
        child: Text('This is the Describe Problem screen'),
      ),
    );
  }
}
