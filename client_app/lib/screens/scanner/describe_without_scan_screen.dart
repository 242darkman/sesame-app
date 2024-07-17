import 'package:flutter/material.dart';

class DescribeWithoutScanScreen extends StatelessWidget {
  const DescribeWithoutScanScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Describe Without Scan')),
      body: const Center(
        child: Text('This is the Describe Without Scan screen'),
      ),
    );
  }
}
