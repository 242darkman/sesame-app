import 'package:client_app/screens/scanner/scanner_screen.dart';
import 'package:flutter/material.dart';

class CustomMenu extends StatelessWidget {
  const CustomMenu({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: const BoxDecoration(
        border: Border(top: BorderSide(color: Color(0xFFA8A8A8))),
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
        children: [
          Padding(
            padding: const EdgeInsets.only(top: 10.0, bottom: 10.0),
            child: IconButton(
              icon: const Icon(
                Icons.qr_code_scanner,
                color: Color(0xFF779DA0),
                size: 35.0,
              ),
              onPressed: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(
                      builder: (context) => const ScannerScreen()),
                );
              },
            ),
          ),
          Padding(
            padding: const EdgeInsets.only(top: 10.0, bottom: 10.0),
            child: IconButton(
              icon: const Icon(
                Icons.question_mark,
                color: Color(0xFF779DA0),
                size: 35.0,
              ),
              onPressed: () {},
            ),
          ),
          Padding(
            padding: const EdgeInsets.only(top: 10.0, bottom: 10.0),
            child: IconButton(
              icon: const Icon(
                Icons.settings,
                color: Color(0xFF779DA0),
                size: 35.0,
              ),
              onPressed: () {},
            ),
          ),
        ],
      ),
    );
  }
}
