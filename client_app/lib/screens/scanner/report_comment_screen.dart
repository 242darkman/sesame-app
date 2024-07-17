import 'package:flutter/material.dart';

class ReportCommentScreen extends StatelessWidget {
  const ReportCommentScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Report Comment')),
      body: const Center(
        child: Text('This is the Report Comment screen'),
      ),
    );
  }
}
