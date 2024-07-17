import 'package:client_app/widgets/custom_app_bar.dart';
import 'package:client_app/widgets/custom_menu.dart';
import 'package:client_app/widgets/scanner_widget.dart';
import 'package:flutter/material.dart';

class ScannerScreen extends StatefulWidget {
  @override
  _ScannerScreenState createState() => _ScannerScreenState();
}

class _ScannerScreenState extends State<ScannerScreen> {
  final ValueNotifier<bool> _isPressed =
      ValueNotifier<bool>(false); // Initialisation de ValueNotifier

  void _onCircleTap() {
    _isPressed.value = !_isPressed.value;
    // Ouvrir l'appareil photo ici après l'animation
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: const CustomAppBar(title: 'Scanner'),
      bottomNavigationBar: const CustomMenu(),
      body: ScannerWidget(
        title: "Scannez le QR Code pour interagir avec la porte",
        subTitle: "Scanner",
        sizeTitle: 18,
        colorScanner: const Color(0xFF30959B),
        onCircleTap: _onCircleTap,
        isPressed: _isPressed,
      ),
    );
  }
}
