import 'package:flutter/material.dart';

class ReportDetailsScreen extends StatelessWidget {
  const ReportDetailsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Report Details')),
      body: const Center(
        child: Text('This is the Report Details screen'),
      ),
    );
  }
}
