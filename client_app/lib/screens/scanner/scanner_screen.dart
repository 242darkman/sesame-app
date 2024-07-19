import 'package:client_app/main.dart';
import 'package:client_app/widgets/custom_app_bar.dart';
import 'package:client_app/widgets/custom_menu.dart';
import 'package:client_app/widgets/scanner_widget.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

class ScannerScreen extends StatefulWidget {
  const ScannerScreen({super.key});

  @override
  _ScannerScreenState createState() => _ScannerScreenState();
}

class _ScannerScreenState extends State<ScannerScreen> {
  final ValueNotifier<bool> _isPressed = ValueNotifier<bool>(false);

  void _onCircleTap() {
    _isPressed.value = !_isPressed.value;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: const CustomAppBar(title: 'Scanner'),
      bottomNavigationBar: const CustomMenu(),
      body: Container(
        color: Colors.white,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            ScannerWidget(
              title: "Scannez le QR Code pour interagir avec la porte",
              subTitle: "Scanner",
              sizeTitle: 18,
              colorScanner: const Color(0xFF30959B),
              onCircleTap: _onCircleTap,
              isPressed: _isPressed,
            ),
            ValueListenableBuilder<bool>(
              valueListenable: showCantScanTextNotifier,
              builder: (context, value, child) {
                return Visibility(
                  visible: value,
                  child: Padding(
                    padding: const EdgeInsets.only(top: 20.0),
                    child: GestureDetector(
                      onTap: () {
                        context.go(
                            '/app/scanner/report_problem/describe_without_scan');
                      },
                      child: const Text(
                        "Je ne peux pas scanner",
                        style: TextStyle(
                          color: Color(0xFF4B4B4B),
                          decoration: TextDecoration.underline,
                        ),
                      ),
                    ),
                  ),
                );
              },
            ),
          ],
        ),
      ),
    );
  }
}
