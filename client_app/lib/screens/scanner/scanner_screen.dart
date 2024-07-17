import 'package:client_app/widgets/custom_app_bar.dart';
import 'package:client_app/widgets/custom_menu.dart';
import 'package:flutter/material.dart';

class ScannerScreen extends StatelessWidget {
  const ScannerScreen({super.key});

  @override
  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      appBar: CustomAppBar(title: 'Scanner'),
      body: Center(child: Text('Contenu de la page.')),
      bottomNavigationBar: CustomMenu(),
    );
  }
}
